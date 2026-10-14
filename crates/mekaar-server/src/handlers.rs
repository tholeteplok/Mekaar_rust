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
    hub.db()
        .add_guardian(&payload.victim_id, &payload.guardian_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(json!({"status": "linked"}))))
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
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerCommand>();

    let mut current_user_id = initial_user_id;

    // Jika user_id sudah diberikan di query parameter awal, langsung hubungkan ke Hub
    if let Some(uid) = current_user_id {
        hub.connect_peer(uid, None, tx.clone()).await;
    }

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

    while let Some(Ok(msg)) = ws_receiver.next().await {
        let client_cmd = match msg {
            Message::Binary(bytes) => {
                // Decode frame biner Postcard
                decode_postcard::<ClientCommand>(&bytes).ok()
            }
            Message::Text(text) => {
                // Decode frame JSON teks
                decode_json::<ClientCommand>(&text).ok()
            }
            Message::Ping(payload) => {
                // Respon internal WebSocket ping
                let _ = tx_clone.send(ServerCommand::Pong);
                debug!("Received ws ping with payload len {}", payload.len());
                continue;
            }
            Message::Close(_) => break,
            _ => continue,
        };

        if let Some(cmd) = client_cmd {
            if let ClientCommand::AuthSubscribe { user_id, public_key } = &cmd {
                current_user_id = Some(*user_id);
                hub_clone
                    .connect_peer(*user_id, Some(*public_key), tx_clone.clone())
                    .await;
            }

            hub_clone.handle_command(current_user_id, cmd).await;
        } else {
            warn!("Paket ClientCommand tidak dapat di-parse dari klien {:?}", current_user_id);
        }
    }

    // Bersihkan koneksi saat socket terputus
    if let Some(uid) = current_user_id {
        hub.disconnect_peer(&uid).await;
    }

    send_task.abort();
    debug!("Socket loop selesai untuk klien {:?}", current_user_id);
}
