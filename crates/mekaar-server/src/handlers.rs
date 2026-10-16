//! # mekaar-server: HTTP REST & WebSocket Handlers
//!
//! Endpoint HTTP dan WebSocket upgrade untuk komunikasi klien MEKAAR.

use crate::hub::Hub;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures_util::{SinkExt, StreamExt};
use mekaar_proto::{
    decode_json, decode_postcard, encode_json, encode_postcard, ClientCommand, ServerCommand,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, warn};
use uuid::Uuid;

// ── Request & Response Structs ─────────────────────────────────────────

#[derive(Deserialize)]
pub struct WsParams {
    pub user_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct RegisterKeyRequest {
    pub user_id: Uuid,
    pub public_key: [u8; 32],
}

#[derive(Deserialize)]
pub struct StoreBackupRequest {
    pub user_id: Uuid,
    pub backup_json: String,
}

#[derive(Deserialize)]
pub struct AddGuardianRequest {
    pub victim_id: Uuid,
    pub guardian_id: Uuid,
}

#[derive(Deserialize)]
pub struct RegisterAccountRequest {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginAccountRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ResolveEmailRequest {
    pub input_query: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub token: String,
    pub new_password: String,
}

// ── REST Handlers ──────────────────────────────────────────────────────

/// Health check server: status sistem dan jumlah koneksi aktif.
pub async fn health_check(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
    let connections = hub.active_connection_count().await;
    Json(json!({
        "status": "ok",
        "service": "mekaar-server",
        "mode": "Zero-Knowledge Blind Relay",
        "version": env!("CARGO_PKG_VERSION"),
        "active_connections": connections
    }))
}

/// Mengambil public key X25519 pengguna dari direktori server.
pub async fn get_public_key(
    State(hub): State<Arc<Hub>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match hub.db().get_public_key(&user_id) {
        Ok(Some(key)) => {
            let hex_key = key.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            Ok(Json(json!({
                "user_id": user_id,
                "public_key_hex": hex_key,
                "public_key": key
            })))
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "Kunci publik tidak ditemukan".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Mendaftarkan atau memperbarui kunci publik identitas pengguna.
pub async fn upsert_public_key(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<RegisterKeyRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.public_key == [0u8; 32] {
        return Err((StatusCode::BAD_REQUEST, "public_key nol ditolak".to_string()));
    }
    hub.db()
        .upsert_public_key(&payload.user_id, &payload.public_key)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(json!({"status": "saved"}))))
}

/// Mengambil data backup kunci privat terenkripsi Argon2id milik pengguna.
pub async fn get_key_backup(
    State(hub): State<Arc<Hub>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match hub.db().get_key_backup(&user_id) {
        Ok(Some(backup)) => Ok(Json(json!({
            "user_id": user_id,
            "backup_json": backup
        }))),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Backup tidak ditemukan".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Menyimpan data backup kunci privat terenkripsi Argon2id ke server.
pub async fn upsert_key_backup(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<StoreBackupRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.backup_json.is_empty() || payload.backup_json.len() > 32 * 1024 {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, "backup_json di luar batas".to_string()));
    }
    serde_json::from_str::<serde_json::Value>(&payload.backup_json)
        .map_err(|_| (StatusCode::BAD_REQUEST, "backup_json bukan JSON".to_string()))?;
    hub.db()
        .upsert_key_backup(&payload.user_id, &payload.backup_json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(json!({"status": "saved"}))))
}

/// Menambahkan tautan relasi Guardian baru.
pub async fn add_guardian(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<AddGuardianRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.victim_id == payload.guardian_id {
        return Err((StatusCode::BAD_REQUEST, "victim == guardian ditolak".to_string()));
    }
    hub.db()
        .add_guardian(&payload.victim_id, &payload.guardian_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(json!({"status": "added"}))))
}

// ── Central Directory Auth Handlers (Mekaar_chat Parity) ───────────────

/// Register akun baru di direktori server.
pub async fn register_account(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<RegisterAccountRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let clean_user = payload.username.trim().to_lowercase();
    let clean_email = payload.email.trim().to_lowercase();
    let username_with_prefix = if clean_user.starts_with('@') { clean_user.clone() } else { format!("@{}", clean_user) };

    if clean_user.len() < 3 || clean_email.is_empty() || payload.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "Data registrasi tidak memenuhi syarat".to_string()));
    }

    // Cek duplikasi
    if let Ok(Some(_)) = hub.db().find_user_by_identifier(&username_with_prefix) {
        return Err((StatusCode::CONFLICT, "Username sudah digunakan".to_string()));
    }
    if let Ok(Some(_)) = hub.db().find_user_by_identifier(&clean_email) {
        return Err((StatusCode::CONFLICT, "Email sudah terdaftar".to_string()));
    }

    // Hash password dengan Argon2id + salt acak
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Hash error: {}", e)))?
        .to_string();

    let user_id = Uuid::new_v4();
    hub.db()
        .create_user_account(
            &user_id,
            &username_with_prefix,
            &payload.display_name.trim(),
            &clean_email,
            &password_hash,
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "user_id": user_id,
            "username": username_with_prefix,
            "display_name": payload.display_name.trim(),
            "email": clean_email
        })),
    ))
}

/// Login akun menggunakan username atau email + password.
pub async fn login_account(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<LoginAccountRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let account = match hub.db().find_user_by_identifier(&payload.identifier) {
        Ok(Some(acc)) => acc,
        Ok(None) => return Err((StatusCode::UNAUTHORIZED, "Akun tidak ditemukan".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    // Verifikasi hash password Argon2id
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let parsed_hash = match PasswordHash::new(&account.password_hash) {
        Ok(h) => h,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Format password hash korup".to_string())),
    };

    let valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Password tidak sesuai".to_string()));
    }

    // Cek apakah akun ini memiliki backup kunci E2EE di server
    let key_backup = hub.db().get_key_backup(&account.id).unwrap_or(None);
    let has_key_backup = key_backup.is_some();

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "user_id": account.id,
            "username": account.username,
            "display_name": account.display_name,
            "email": account.email,
            "has_key_backup": has_key_backup,
            "backup_json": key_backup
        })),
    ))
}

/// Resolusi email dari username untuk flow lupa password.
pub async fn resolve_email(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<ResolveEmailRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let input = payload.input_query.trim().to_lowercase();
    if input.contains('@') {
        return Ok(Json(json!({ "email": input })));
    }

    match hub.db().find_user_by_identifier(&input) {
        Ok(Some(acc)) => Ok(Json(json!({ "email": acc.email }))),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Username tidak ditemukan".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Reset password akun.
pub async fn reset_password(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.new_password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "Password minimal 6 karakter".to_string()));
    }

    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let new_hash = argon2
        .hash_password(payload.new_password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Hash error: {}", e)))?
        .to_string();

    match hub.db().update_user_password(&payload.email, &new_hash) {
        Ok(true) => Ok(Json(json!({ "status": "success", "message": "Password berhasil diperbarui" }))),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Akun tidak ditemukan".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Mengambil daftar seluruh Guardian untuk seorang korban.
pub async fn get_guardians(
    State(hub): State<Arc<Hub>>,
    Path(victim_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let list = hub
        .db()
        .get_guardians_for_victim(&victim_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "victim_id": victim_id,
        "guardians": list
    })))
}

// ── WebSocket Handler ──────────────────────────────────────────────────

/// Upgrade koneksi HTTP ke duplex WebSocket stream.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<Hub>>,
    Query(params): Query<WsParams>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, hub, params.user_id))
}

/// Penanganan siklus hidup WebSocket socket: framing biner/teks, auth, dan pump channel.
async fn handle_socket(socket: WebSocket, hub: Arc<Hub>, initial_user_id: Option<Uuid>) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    // Bounded channel (256) agar satu klien zalim tidak OOM server.
    let (tx, mut rx) = mpsc::channel::<ServerCommand>(256);

    // P0: jangan auto-connect via query pre-auth. Wajib AuthSubscribe dulu.
    // Query hanya jadi petunjuk; koneksi Hub dibuat setelah AuthSubscribe terverifikasi.
    if initial_user_id.is_some() {
        warn!("Query user_id pre-auth diabaikan; menunggu AuthSubscribe");
    }
    let mut current_user_id: Option<Uuid> = None;
    let mut current_epoch: Option<u64> = None;

    // Task 1: Outgoing pump (Kirim pesan ServerCommand dari channel mpsc ke klien WebSocket)
    let send_task = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            // Coba encode dengan Postcard biner untuk efisiensi kuota transmisi
            let msg = match encode_postcard(&cmd) {
                Ok(bytes) => Message::Binary(bytes.into()),
                Err(_) => {
                    // Fallback ke JSON jika postcard gagal
                    if let Ok(json_str) = encode_json(&cmd) {
                        Message::Text(json_str.into())
                    } else {
                        continue;
                    }
                }
            };

            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Task 2: Incoming pump (Baca frame WebSocket dari klien dan eksekusi di Hub)
    let hub_clone = hub.clone();
    let tx_clone = tx.clone();

    while let Some(msg_res) = ws_receiver.next().await {
        let msg = match msg_res {
            Ok(m) => m,
            Err(e) => {
                warn!("WebSocket error dari {:?}: {}", current_user_id, e);
                break;
            }
        };
        // Cap frame 64KiB agar flood langsung ditolak.
        let too_big = match &msg {
            Message::Binary(b) => b.len() > 64 * 1024,
            Message::Text(t) => t.len() > 64 * 1024,
            _ => false,
        };
        if too_big {
            warn!("Frame >64KiB dari {:?} ditolak", current_user_id);
            continue;
        }
        let client_cmd = match msg {
            Message::Binary(bytes) => {
                // Decode frame biner Postcard
                match decode_postcard::<ClientCommand>(&bytes) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        warn!("Decode postcard gagal dari {:?}: {}", current_user_id, e);
                        None
                    }
                }
            }
            Message::Text(text) => {
                // Decode frame JSON teks
                match decode_json::<ClientCommand>(&text) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        warn!("Decode JSON gagal dari {:?}: {}", current_user_id, e);
                        None
                    }
                }
            }
            Message::Ping(payload) => {
                // Respon internal WebSocket ping
                let _ = tx_clone.try_send(ServerCommand::Pong);
                debug!("Received ws ping with payload len {}", payload.len());
                continue;
            }
            Message::Pong(_) => continue,
            Message::Close(_) => break,
        };

        if let Some(cmd) = client_cmd {
            if let ClientCommand::AuthSubscribe { user_id, public_key } = &cmd {
                if *public_key == [0u8; 32] {
                    warn!("AuthSubscribe kunci nol ditolak");
                } else {
                    current_user_id = Some(*user_id);
                    let epoch = hub_clone
                        .connect_peer(*user_id, Some(*public_key), tx_clone.clone())
                        .await;
                    current_epoch = Some(epoch);
                }
            }

            hub_clone.handle_command(current_user_id, cmd).await;
        } else {
            warn!("Paket ClientCommand tidak dapat di-parse dari klien {:?}", current_user_id);
        }
    }

    // Bersihkan koneksi saat socket terputus (guard epoch agar tidak tendang sesi baru)
    if let (Some(uid), Some(epoch)) = (current_user_id, current_epoch) {
        hub.disconnect_peer_if_epoch(&uid, epoch).await;
    }

    send_task.abort();
    debug!("Socket loop selesai untuk klien {:?}", current_user_id);
}
