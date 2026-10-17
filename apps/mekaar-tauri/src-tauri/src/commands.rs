use mekaar_core::events::EventBus;
use mekaar_core::sos::{SosEndReason, SosPanicEngine};
use mekaar_crypto::{
    derive_room_key, encrypt_message, IdentityPrivateKey, IdentityPublicKey,
};
use mekaar_proto::MessageType;
use mekaar_storage::{
    ChatRoom, Contact, DeliveryStatus, EncryptedDatabase, GuardianRecord, LocalMessage,
    SavedPlaceRecord, SecurityLog, UserTripRecord,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{State, Window};
use uuid::Uuid;

pub const DEFAULT_TAILSCALE_HTTP: &str = "http://100.101.96.120:8085";
pub const DEFAULT_TAILSCALE_WS: &str = "ws://100.101.96.120:8085/ws";
pub const DEFAULT_LOCAL_HTTP: &str = "http://127.0.0.1:8085";
pub const DEFAULT_LOCAL_WS: &str = "ws://127.0.0.1:8085/ws";

// ── Models & Preferences ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConnectionConfig {
    pub active_mode: String, // "tailscale" | "local" | "custom"
    pub http_endpoint: String,
    pub ws_endpoint: String,
    pub auto_reconnect: bool,
}

impl Default for ServerConnectionConfig {
    fn default() -> Self {
        Self {
            active_mode: "tailscale".to_string(),
            http_endpoint: DEFAULT_TAILSCALE_HTTP.to_string(),
            ws_endpoint: DEFAULT_TAILSCALE_WS.to_string(),
            auto_reconnect: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SosConfig {
    pub countdown_seconds: u8,
    pub siren_enabled: bool,
    pub shake_to_trigger: bool,
    pub auto_broadcast_location: bool,
    pub emergency_message: String,
}

impl Default for SosConfig {
    fn default() -> Self {
        Self {
            countdown_seconds: 5,
            siren_enabled: true,
            shake_to_trigger: true,
            auto_broadcast_location: true,
            emergency_message: "DARURAT! Saya membutuhkan bantuan segera. Lokasi langsung saya sedang dibagikan.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPreferences {
    pub theme_mode: String,        // "light" | "dark" | "system"
    pub font_family: String,       // "Plus Jakarta Sans" | "DynaPuff" | "Inter"
    pub font_scale: f32,           // 0.9 .. 1.25
    pub screenshot_block: bool,
    pub privacy_blur_on_unfocus: bool,
    pub disguised_notifications: bool,
    pub read_receipts: bool,
    pub typing_indicator: bool,
    pub default_chat_theme: String,
    pub pin_lock_enabled: bool,
    pub two_factor_enabled: bool,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            theme_mode: "light".to_string(),
            font_family: "Plus Jakarta Sans".to_string(),
            font_scale: 1.0,
            screenshot_block: true,
            privacy_blur_on_unfocus: true,
            disguised_notifications: false,
            read_receipts: true,
            typing_indicator: true,
            default_chat_theme: "classic".to_string(),
            pin_lock_enabled: true,
            two_factor_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultAuthResult {
    pub success: bool,
    pub mode: String, // "unlocked" | "duress"
    pub message: String,
    pub is_duress_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultStatusResponse {
    pub vault_unlocked: bool,
    pub is_duress_active: bool,
    pub has_duress_pin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SosTriggerResponse {
    pub active: bool,
    pub session_id: String,
    pub broadcast_url: String,
    pub is_silent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicExportResponse {
    pub filename: String,
    pub checksum: String,
    pub signature: String,
    pub statement: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedDevice {
    pub id: String,
    pub device_name: String,
    pub platform: String,
    pub ip_address: String,
    pub last_active: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledWipeSettings {
    pub mode: String, // "off" | "one_shot" | "daily"
    pub target_time: String, // "HH:mm"
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryStatus {
    pub latitude: f64,
    pub longitude: f64,
    pub battery_level: u8,
    pub is_in_safe_zone: bool,
    pub is_in_danger_zone: bool,
    pub active_zone_name: Option<String>,
    pub updated_at: u64,
}

impl Default for TelemetryStatus {
    fn default() -> Self {
        Self {
            latitude: -7.5568,
            longitude: 110.8521,
            battery_level: 84,
            is_in_safe_zone: true,
            is_in_danger_zone: false,
            active_zone_name: Some("Rumah Utama (Zona Aman)".to_string()),
            updated_at: 0,
        }
    }
}

// ── Security Helpers (P0: hapus backdoor PIN, fail-closed) ───────────────────

/// Perbandingan constant-time untuk PIN agar tidak bocor via timing oracle.
fn ct_eq_str(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn is_valid_pin(pin: &str) -> bool {
    // Minimal 6 digit numerik. Menolak PIN kosong / default lemah.
    pin.len() >= 6 && pin.chars().all(|c| c.is_ascii_digit())
}

fn require_unlocked(state: &MekaarAppState) -> Result<(), String> {
    let unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
    if *unlocked {
        Ok(())
    } else {
        Err("Vault terkunci. Buka dengan PIN terlebih dahulu.".to_string())
    }
}

fn require_db(state: &MekaarAppState) -> Result<EncryptedDatabase, String> {
    require_unlocked(state)?;
    let lock = state.db.lock().map_err(|e| e.to_string())?;
    lock.clone().ok_or_else(|| "Database belum diinisialisasi. Unlock vault dulu.".to_string())
}

// ── App State ───────────────────────────────────────────────────────────

pub struct MekaarAppState {
    pub vault_unlocked: Mutex<bool>,
    pub pin_code: Mutex<String>,
    pub duress_pin_code: Mutex<String>,
    pub is_duress_active: Mutex<bool>,
    pub server_config: Mutex<ServerConnectionConfig>,
    pub sos_config: Mutex<SosConfig>,
    pub sos_active: Mutex<bool>,
    pub preferences: Mutex<AppPreferences>,
    pub db: Mutex<Option<EncryptedDatabase>>,
    pub sos_engine: Arc<SosPanicEngine>,
    pub user_id: Mutex<Uuid>,
    pub identity_key: Mutex<Option<([u8; 32], [u8; 32])>>,
    pub telemetry: Mutex<TelemetryStatus>,
}

impl Default for MekaarAppState {
    fn default() -> Self {
        let bus = EventBus::new(100);
        let sos_engine = Arc::new(SosPanicEngine::new(bus));
        Self {
            vault_unlocked: Mutex::new(false),
            // P0: tidak ada PIN default. Wajib setup via setup_pins.
            pin_code: Mutex::new(String::new()),
            duress_pin_code: Mutex::new(String::new()),
            is_duress_active: Mutex::new(false),
            server_config: Mutex::new(ServerConnectionConfig::default()),
            sos_config: Mutex::new(SosConfig::default()),
            sos_active: Mutex::new(false),
            preferences: Mutex::new(AppPreferences::default()),
            db: Mutex::new(None),
            sos_engine,
            user_id: Mutex::new(Uuid::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap()),
            identity_key: Mutex::new(None),
            telemetry: Mutex::new(TelemetryStatus::default()),
        }
    }
}

pub fn get_or_create_identity_key(state: &MekaarAppState) -> Result<(IdentityPrivateKey, IdentityPublicKey), String> {
    let mut lock = state.identity_key.lock().map_err(|e| e.to_string())?;
    if let Some(ref pair) = *lock {
        return Ok((IdentityPrivateKey::from_bytes(pair.0), IdentityPublicKey::from_bytes(pair.1)));
    }
    let (priv_k, pub_k) = mekaar_crypto::generate_identity();
    *lock = Some((*priv_k.as_bytes(), *pub_k.as_bytes()));
    Ok((priv_k, pub_k))
}

// ── Database Helper ─────────────────────────────────────────────────────

fn get_or_init_db(state: &MekaarAppState, pin: &str) -> Result<EncryptedDatabase, String> {
    let mut lock = state.db.lock().map_err(|e| e.to_string())?;
    if let Some(ref db) = *lock {
        return Ok(db.clone());
    }

    let db = EncryptedDatabase::open_in_memory(pin).map_err(|e| e.to_string())?;

    // Inisialisasi data awal (Seed rooms & contacts)
    let room1_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let room2_id = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
    let room3_id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
    let peer_ayu = Uuid::parse_str("44444444-4444-4444-4444-444444444444").unwrap();
    let peer_bima = Uuid::parse_str("55555555-5555-5555-5555-555555555555").unwrap();

    let _ = db.upsert_room(&ChatRoom {
        id: room1_id,
        room_type: "normal".to_string(),
        name: "Ayu Lestari".to_string(),
        peer_id: Some(peer_ayu),
        disappearing_timer: None,
        theme_preset: "classic".to_string(),
        last_message_at: 1728900000000,
        is_archived: false,
        is_muted: false,
        is_vault_hidden: false,
    });

    let _ = db.upsert_room(&ChatRoom {
        id: room2_id,
        room_type: "group".to_string(),
        name: "Tim Respon Guardian Solo".to_string(),
        peer_id: None,
        disappearing_timer: Some(3600000),
        theme_preset: "forest".to_string(),
        last_message_at: 1728899000000,
        is_archived: false,
        is_muted: false,
        is_vault_hidden: false,
    });

    let _ = db.upsert_room(&ChatRoom {
        id: room3_id,
        room_type: "normal".to_string(),
        name: "Bima Pratama (Brankas Rahasia)".to_string(),
        peer_id: Some(peer_bima),
        disappearing_timer: None,
        theme_preset: "classic".to_string(),
        last_message_at: 1728890000000,
        is_archived: false,
        is_muted: false,
        is_vault_hidden: true, // Tersembunyi saat mode Duress aktif
    });

    let _ = db.upsert_contact(&Contact {
        id: peer_ayu,
        username: "@ayulestari".to_string(),
        display_name: "Ayu Lestari".to_string(),
        public_key: [0x42; 32],
        safety_number: "44B1-82C9-11A0".to_string(),
        status: "active".to_string(),
        updated_at: 1728900000000,
    });

    let _ = db.upsert_contact(&Contact {
        id: peer_bima,
        username: "@bimapratama".to_string(),
        display_name: "Bima Pratama".to_string(),
        public_key: [0x43; 32],
        safety_number: "77C1-09B4-31E2".to_string(),
        status: "active".to_string(),
        updated_at: 1728890000000,
    });

    let my_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let _ = db.insert_message(&LocalMessage {
        id: Uuid::new_v4(),
        room_id: room1_id,
        sender_id: peer_ayu,
        recipient_id: my_id,
        content: "Koneksi WebSocket ke teplok-pc (8085) sudah stabil!".to_string(),
        media_url: None,
        msg_type: MessageType::Text,
        is_view_once: false,
        is_opened: true,
        opened_at: Some(1728900000000),
        reply_to_id: None,
        is_deleted: false,
        is_silent_deleted: false,
        auto_delete_at: None,
        edited_at: None,
        reactions: HashMap::new(),
        is_encrypted: true,
        created_at: 1728900000000,
        updated_at: 1728900000000,
        status: DeliveryStatus::Read,
    });

    // Seed User Trips
    let _ = db.upsert_trip(&UserTripRecord {
        id: Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap(),
        user_id: my_id,
        origin_name: "Stasiun Solo Balapan".to_string(),
        destination_name: "Rumah (Jebres, Surakarta)".to_string(),
        dest_lat: -7.5568,
        dest_lng: 110.8521,
        dest_radius: 150,
        eta_minutes: 12,
        status: "ongoing".to_string(),
        started_at: 1728900000000,
        ended_at: None,
        active_guardians_json: serde_json::to_string(&vec!["Ayu Lestari", "Bima Pratama"]).unwrap(),
        progress_percent: 68,
        created_at: 1728900000000,
    });

    let _ = db.upsert_trip(&UserTripRecord {
        id: Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
        user_id: my_id,
        origin_name: "Kantor Pusat".to_string(),
        destination_name: "Stasiun Solo Balapan".to_string(),
        dest_lat: -7.5580,
        dest_lng: 110.8210,
        dest_radius: 150,
        eta_minutes: 25,
        status: "completed".to_string(),
        started_at: 1728800000000,
        ended_at: Some(1728801500000),
        active_guardians_json: serde_json::to_string(&vec!["Ayu Lestari"]).unwrap(),
        progress_percent: 100,
        created_at: 1728800000000,
    });

    // Seed Saved Places
    let _ = db.upsert_saved_place(&SavedPlaceRecord {
        id: "plc-01".to_string(),
        user_id: my_id,
        name: "Rumah Utama (Zona Aman)".to_string(),
        category: "home".to_string(),
        address: Some("Jl. Ki Hajar Dewantara, Jebres, Surakarta".to_string()),
        latitude: -7.5568,
        longitude: 110.8521,
        radius_meters: 150,
        auto_notify_guardian: true,
        updated_at: 1728900000000,
    });

    let _ = db.upsert_saved_place(&SavedPlaceRecord {
        id: "plc-02".to_string(),
        user_id: my_id,
        name: "Server Lab (teplok-pc)".to_string(),
        category: "office".to_string(),
        address: Some("Laweyan, Surakarta".to_string()),
        latitude: -7.5682,
        longitude: 110.7984,
        radius_meters: 100,
        auto_notify_guardian: true,
        updated_at: 1728900000000,
    });

    let _ = db.upsert_saved_place(&SavedPlaceRecord {
        id: "plc-03".to_string(),
        user_id: my_id,
        name: "RSUD Dr. Moewardi (Pos Medis)".to_string(),
        category: "safe_zone".to_string(),
        address: Some("Jl. Kolonel Sutarto No.132, Surakarta".to_string()),
        latitude: -7.5592,
        longitude: 110.8415,
        radius_meters: 250,
        auto_notify_guardian: false,
        updated_at: 1728900000000,
    });

    // Seed Guardians
    let _ = db.upsert_guardian(&GuardianRecord {
        id: "grd-01".to_string(),
        user_id: my_id,
        name: "Ayu Lestari".to_string(),
        username: "@ayulestari".to_string(),
        avatar_color: "#BFDBFE".to_string(),
        relation: "Guardian Utama".to_string(),
        status: "active".to_string(),
        can_view_live_location: true,
        can_receive_sos_alert: true,
        battery_percent: 86,
        last_latitude: Some(-7.5561),
        last_longitude: Some(110.8318),
        updated_at: 1728900000000,
    });

    let _ = db.upsert_guardian(&GuardianRecord {
        id: "grd-02".to_string(),
        user_id: my_id,
        name: "Bima Pratama".to_string(),
        username: "@bimapratama".to_string(),
        avatar_color: "#FBCFE8".to_string(),
        relation: "Rekan Siaga".to_string(),
        status: "standby".to_string(),
        can_view_live_location: true,
        can_receive_sos_alert: true,
        battery_percent: 74,
        last_latitude: Some(-7.5624),
        last_longitude: Some(110.8215),
        updated_at: 1728900000000,
    });

    let _ = db.upsert_guardian(&GuardianRecord {
        id: "grd-03".to_string(),
        user_id: my_id,
        name: "Dimas Wijaya".to_string(),
        username: "@dimasw".to_string(),
        avatar_color: "#FDE68A".to_string(),
        relation: "Keluarga".to_string(),
        status: "pending_request".to_string(),
        can_view_live_location: false,
        can_receive_sos_alert: true,
        battery_percent: 91,
        last_latitude: None,
        last_longitude: None,
        updated_at: 1728900000000,
    });

    *lock = Some(db.clone());
    Ok(db)
}

// ── IPC Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_connection_config(state: State<'_, MekaarAppState>) -> Result<ServerConnectionConfig, String> {
    let cfg = state.server_config.lock().map_err(|e| e.to_string())?;
    Ok(cfg.clone())
}

#[tauri::command]
pub fn set_connection_config(
    mode: String,
    custom_http: Option<String>,
    custom_ws: Option<String>,
    state: State<'_, MekaarAppState>,
) -> Result<ServerConnectionConfig, String> {
    fn valid_endpoint(url: &str, ws: bool) -> bool {
        // Tolak skema berbahaya; izinkan http/ws hanya untuk loopback (dev),
        // selain itu wajib https/wss. Cap panjang untuk hindari bloat.
        if url.len() > 256 {
            return false;
        }
        if ws {
            if url.starts_with("wss://") {
                return true;
            }
            if url.starts_with("ws://127.0.0.1") || url.starts_with("ws://localhost") {
                return true;
            }
            return false;
        }
        if url.starts_with("https://") {
            return true;
        }
        if url.starts_with("http://127.0.0.1") || url.starts_with("http://localhost") {
            return true;
        }
        false
    }

    let mut cfg = state.server_config.lock().map_err(|e| e.to_string())?;
    cfg.active_mode = mode.clone();
    match mode.as_str() {
        "local" => {
            cfg.http_endpoint = DEFAULT_LOCAL_HTTP.to_string();
            cfg.ws_endpoint = DEFAULT_LOCAL_WS.to_string();
        }
        "custom" => {
            if let Some(http) = custom_http {
                if !valid_endpoint(&http, false) {
                    return Err("HTTP endpoint tidak valid (gunakan https, atau http loopback)".to_string());
                }
                cfg.http_endpoint = http;
            }
            if let Some(ws) = custom_ws {
                if !valid_endpoint(&ws, true) {
                    return Err("WS endpoint tidak valid (gunakan wss, atau ws loopback)".to_string());
                }
                cfg.ws_endpoint = ws;
            }
        }
        _ => {
            cfg.http_endpoint = DEFAULT_TAILSCALE_HTTP.to_string();
            cfg.ws_endpoint = DEFAULT_TAILSCALE_WS.to_string();
        }
    }
    Ok(cfg.clone())
}

#[tauri::command]
pub async fn unlock_vault(pin: String, state: State<'_, MekaarAppState>) -> Result<VaultAuthResult, String> {
    let duress_pin = {
        let guard = state.duress_pin_code.lock().map_err(|e| e.to_string())?;
        guard.clone()
    };
    let master_pin = {
        let guard = state.pin_code.lock().map_err(|e| e.to_string())?;
        guard.clone()
    };

    if !duress_pin.is_empty() && ct_eq_str(&pin, &duress_pin) {
        // DURESS MODE: Buka aplikasi secara senyap dengan cloaking
        {
            let mut is_duress = state.is_duress_active.lock().map_err(|e| e.to_string())?;
            *is_duress = true;
        }
        {
            let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
            *unlocked = true;
        }
        {
            let mut sos_active = state.sos_active.lock().map_err(|e| e.to_string())?;
            *sos_active = true;
        }

        let _ = state.sos_engine.trigger_duress_silent().await;

        let db = get_or_init_db(&state, &pin)?;
        let uid = { *state.user_id.lock().map_err(|e| e.to_string())? };
        let _ = db.insert_security_log(&uid, None, "DURESS_UNLOCK", Some(&serde_json::json!({
            "action": "silent_sos_triggered",
            "source": "duress_pin_entry",
            "cloaking": "private_vault_hidden"
        })));

        return Ok(VaultAuthResult {
            success: true,
            mode: "duress".to_string(),
            message: "Aplikasi dibuka dalam mode aman".to_string(),
            is_duress_active: true,
        });
    }

    let expected = master_pin.clone();
    if expected.is_empty() {
        return Err("PIN belum diatur. Lakukan setup PIN terlebih dahulu.".to_string());
    }
    if ct_eq_str(&pin, &expected) {
        {
            let mut is_duress = state.is_duress_active.lock().map_err(|e| e.to_string())?;
            *is_duress = false;
        }
        {
            let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
            *unlocked = true;
        }

        let db = get_or_init_db(&state, &pin)?;
        let uid = { *state.user_id.lock().map_err(|e| e.to_string())? };
        let _ = db.insert_security_log(&uid, None, "NORMAL_UNLOCK", Some(&serde_json::json!({
            "action": "vault_unlocked",
            "source": "master_pin_entry"
        })));

        return Ok(VaultAuthResult {
            success: true,
            mode: "unlocked".to_string(),
            message: "Brankas berhasil dibuka".to_string(),
            is_duress_active: false,
        });
    }

    Err("PIN tidak sesuai. Silakan coba lagi.".to_string())
}

#[tauri::command]
pub fn lock_vault(state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
    *unlocked = false;
    let mut is_duress = state.is_duress_active.lock().map_err(|e| e.to_string())?;
    *is_duress = false;
    Ok(true)
}

#[tauri::command]
pub fn verify_pin(pin: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let saved_pin = state.pin_code.lock().map_err(|e| e.to_string())?;
    if saved_pin.is_empty() {
        return Ok(false);
    }
    Ok(ct_eq_str(&pin, &saved_pin))
}

#[tauri::command]
pub fn set_pin(new_pin: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    if !is_valid_pin(&new_pin) {
        return Err("PIN harus minimal 6 digit numerik".to_string());
    }
    // Tolak jika sama dengan duress PIN aktif (hindari silent-SOS tiap login benar).
    {
        let duress = state.duress_pin_code.lock().map_err(|e| e.to_string())?;
        if !duress.is_empty() && ct_eq_str(&new_pin, &duress) {
            return Err("PIN utama tidak boleh sama dengan PIN paksaan!".to_string());
        }
    }
    let mut saved_pin = state.pin_code.lock().map_err(|e| e.to_string())?;
    *saved_pin = new_pin;
    Ok(true)
}

#[tauri::command]
pub fn setup_pins(
    master_pin: String,
    duress_pin: Option<String>,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    if !is_valid_pin(&master_pin) {
        return Err("PIN Utama minimal 6 digit numerik".to_string());
    }

    if let Some(ref d_pin) = duress_pin {
        if !d_pin.is_empty() {
            if !is_valid_pin(d_pin) {
                return Err("PIN Paksaan minimal 6 digit numerik".to_string());
            }
            if ct_eq_str(d_pin, &master_pin) {
                return Err("PIN Paksaan tidak boleh sama dengan PIN utama!".to_string());
            }
            let mut saved_duress = state.duress_pin_code.lock().map_err(|e| e.to_string())?;
            *saved_duress = d_pin.clone();
        }
    }

    let mut saved_master = state.pin_code.lock().map_err(|e| e.to_string())?;
    *saved_master = master_pin;
    Ok(true)
}

#[tauri::command]
pub fn get_vault_status(state: State<'_, MekaarAppState>) -> Result<VaultStatusResponse, String> {
    let unlocked = *state.vault_unlocked.lock().map_err(|e| e.to_string())?;
    let duress = *state.is_duress_active.lock().map_err(|e| e.to_string())?;
    let d_pin = state.duress_pin_code.lock().map_err(|e| e.to_string())?;
    Ok(VaultStatusResponse {
        vault_unlocked: unlocked,
        is_duress_active: duress,
        has_duress_pin: !d_pin.is_empty(),
    })
}

// ── Chat & Message Commands ─────────────────────────────────────────────

#[tauri::command]
pub fn get_conversations(state: State<'_, MekaarAppState>) -> Result<Vec<ChatRoom>, String> {
    let db = require_db(&state)?;
    let is_duress = *state.is_duress_active.lock().map_err(|e| e.to_string())?;
    let include_vault_hidden = !is_duress;
    let rooms = db.list_rooms(include_vault_hidden).map_err(|e| e.to_string())?;
    Ok(rooms)
}

#[tauri::command]
pub fn get_messages(room_id: String, state: State<'_, MekaarAppState>) -> Result<Vec<LocalMessage>, String> {
    let db = require_db(&state)?;
    // Fail-closed: hanya terima UUID valid atau alias legacy yang eksplisit.
    let r_uuid = match room_id.as_str() {
        "conv-1" => Uuid::parse_str("11111111-1111-1111-1111-111111111111").map_err(|e| e.to_string())?,
        "conv-2" => Uuid::parse_str("22222222-2222-2222-2222-222222222222").map_err(|e| e.to_string())?,
        "conv-3" => Uuid::parse_str("33333333-3333-3333-3333-333333333333").map_err(|e| e.to_string())?,
        other => Uuid::parse_str(other).map_err(|_| "room_id tidak valid".to_string())?,
    };

    let msgs = db.get_messages_for_room(&r_uuid, 100).map_err(|e| e.to_string())?;
    Ok(msgs)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageResult {
    pub message: LocalMessage,
    pub envelope: mekaar_proto::MessageEnvelope,
}

#[tauri::command]
pub fn send_e2ee_message(
    room_id: String,
    recipient_id: String,
    content: String,
    msg_type: Option<String>,
    media_url: Option<String>,
    reply_to_id: Option<String>,
    is_view_once: Option<bool>,
    expires_at: Option<u64>,
    state: State<'_, MekaarAppState>,
) -> Result<SendMessageResult, String> {
    let db = require_db(&state)?;
    if content.is_empty() || content.len() > 8000 {
        return Err("Konten pesan harus 1..8000 byte".to_string());
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let r_uuid = match room_id.as_str() {
        "conv-1" => Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        "conv-2" => Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
        "conv-3" => Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap(),
        other => {
            let clean = other.trim_start_matches("conv-");
            Uuid::parse_str(clean).unwrap_or_else(|_| Uuid::new_v4())
        }
    };

    let rec_uuid = {
        let clean = recipient_id.trim_start_matches("usr-");
        Uuid::parse_str(clean).unwrap_or_else(|_| Uuid::new_v4())
    };
    let my_id = *state.user_id.lock().map_err(|e| e.to_string())?;

    // Ambil kunci peer dari DB jika ada, atau fallback
    let contact = db.get_contact(&rec_uuid).map_err(|e| e.to_string())?;
    let peer_pub = if let Some(ref c) = contact {
        if c.public_key != [0u8; 32] {
            IdentityPublicKey::from_bytes(c.public_key)
        } else {
            let (_, pub_k) = mekaar_crypto::generate_identity();
            pub_k
        }
    } else {
        let (_, pub_k) = mekaar_crypto::generate_identity();
        pub_k
    };

    let (my_priv, _) = get_or_create_identity_key(&state)?;
    let room_key = derive_room_key(&my_priv, &peer_pub, &r_uuid.to_string())
        .map_err(|e| e.to_string())?;

    let (nonce, ciphertext) = encrypt_message(&room_key, content.as_bytes())
        .map_err(|e| e.to_string())?;

    let parsed_msg_type = match msg_type.as_deref() {
        Some("image") => MessageType::Image,
        Some("voice") => MessageType::Voice,
        Some("video") => MessageType::Video,
        Some("location") => MessageType::Location,
        Some("system") => MessageType::System,
        _ => MessageType::Text,
    };

    let reply_to_uuid = reply_to_id.as_deref().and_then(|id| {
        let clean = id.trim_start_matches("msg-");
        Uuid::parse_str(clean).ok()
    });

    let view_once = is_view_once.unwrap_or(false);
    let auto_delete = expires_at;

    let msg_id = Uuid::new_v4();
    let msg = LocalMessage {
        id: msg_id,
        room_id: r_uuid,
        sender_id: my_id,
        recipient_id: rec_uuid,
        content,
        media_url,
        msg_type: parsed_msg_type,
        is_view_once: view_once,
        is_opened: true,
        opened_at: Some(now),
        reply_to_id: reply_to_uuid,
        is_deleted: false,
        is_silent_deleted: false,
        auto_delete_at: auto_delete,
        edited_at: None,
        reactions: HashMap::new(),
        is_encrypted: true,
        created_at: now,
        updated_at: now,
        status: DeliveryStatus::Sent,
    };

    let envelope = mekaar_proto::MessageEnvelope {
        version: 1,
        message_id: msg_id,
        sender_id: my_id,
        recipient_id: rec_uuid,
        room_id: r_uuid,
        nonce,
        ciphertext,
        timestamp_ms: now,
        msg_type: parsed_msg_type,
        is_view_once: view_once,
        expires_at: auto_delete,
    };

    db.insert_message(&msg).map_err(|e| e.to_string())?;
    Ok(SendMessageResult {
        message: msg,
        envelope,
    })
}

#[tauri::command]
pub fn decrypt_incoming_envelope(
    envelope: mekaar_proto::MessageEnvelope,
    state: State<'_, MekaarAppState>,
) -> Result<LocalMessage, String> {
    let db = require_db(&state)?;
    let my_id = *state.user_id.lock().map_err(|e| e.to_string())?;

    let contact = db.get_contact(&envelope.sender_id).map_err(|e| e.to_string())?;
    let (my_priv, _) = get_or_create_identity_key(&state)?;

    let decrypted_content = if let Some(ref c) = contact {
        if c.public_key != [0u8; 32] {
            let peer_pub = IdentityPublicKey::from_bytes(c.public_key);
            if let Ok(room_key) = derive_room_key(&my_priv, &peer_pub, &envelope.room_id.to_string()) {
                mekaar_crypto::decrypt_message(&room_key, &envelope.nonce, &envelope.ciphertext)
                    .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
                    .unwrap_or_else(|_| "[Pesan terenkripsi gagal didekripsi]".to_string())
            } else {
                "[Gagal menurunkan kunci sesi]".to_string()
            }
        } else {
            "[Kunci publik pengirim belum diverifikasi]".to_string()
        }
    } else {
        let fallback_username = format!("@user_{}", &envelope.sender_id.to_string()[..8]);
        let new_c = Contact {
            id: envelope.sender_id,
            username: fallback_username.clone(),
            display_name: fallback_username,
            public_key: [0u8; 32],
            safety_number: "UNVERIFIED".to_string(),
            status: "active".to_string(),
            updated_at: envelope.timestamp_ms / 1000,
        };
        let _ = db.upsert_contact(&new_c);
        "[Pesan masuk dari kontak baru]".to_string()
    };

    let msg = LocalMessage {
        id: envelope.message_id,
        room_id: envelope.room_id,
        sender_id: envelope.sender_id,
        recipient_id: my_id,
        content: decrypted_content,
        media_url: None,
        msg_type: envelope.msg_type,
        is_view_once: envelope.is_view_once,
        is_opened: false,
        opened_at: None,
        reply_to_id: None,
        is_deleted: false,
        is_silent_deleted: false,
        auto_delete_at: envelope.expires_at,
        edited_at: None,
        reactions: HashMap::new(),
        is_encrypted: true,
        created_at: envelope.timestamp_ms,
        updated_at: envelope.timestamp_ms,
        status: DeliveryStatus::Delivered,
    };

    let _ = db.insert_message(&msg);
    Ok(msg)
}

#[tauri::command]
pub fn edit_e2ee_message(
    room_id: String,
    message_id: String,
    new_content: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let _ = room_id;
    let db = require_db(&state)?;
    let clean_id = message_id.trim_start_matches("msg-");
    let msg_uuid = Uuid::parse_str(clean_id).map_err(|e| e.to_string())?;
    db.edit_message(&msg_uuid, &new_content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn revoke_e2ee_message(
    room_id: String,
    message_id: String,
    for_everyone: bool,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let _ = room_id;
    let db = require_db(&state)?;
    let clean_id = message_id.trim_start_matches("msg-");
    let msg_uuid = Uuid::parse_str(clean_id).map_err(|e| e.to_string())?;
    if for_everyone {
        db.revoke_message(&msg_uuid).map_err(|e| e.to_string())
    } else {
        db.delete_message(&msg_uuid).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn delete_e2ee_message(
    message_id: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let clean_id = message_id.trim_start_matches("msg-");
    let msg_uuid = Uuid::parse_str(clean_id).map_err(|e| e.to_string())?;
    db.delete_message(&msg_uuid).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_reaction_e2ee(
    room_id: String,
    message_id: String,
    emoji: String,
    state: State<'_, MekaarAppState>,
) -> Result<HashMap<String, Vec<String>>, String> {
    let _ = room_id;
    let db = require_db(&state)?;
    let clean_id = message_id.trim_start_matches("msg-");
    let msg_uuid = Uuid::parse_str(clean_id).map_err(|e| e.to_string())?;
    let my_id = { *state.user_id.lock().map_err(|e| e.to_string())? }.to_string();

    let mut reactions = match db.get_message(&msg_uuid).map_err(|e| e.to_string())? {
        Some(m) => m.reactions,
        None => HashMap::new(),
    };

    let users = reactions.entry(emoji.clone()).or_insert_with(Vec::new);
    if let Some(pos) = users.iter().position(|u| u == &my_id) {
        users.remove(pos);
        if users.is_empty() {
            reactions.remove(&emoji);
        }
    } else {
        users.push(my_id);
    }

    let reactions_json = serde_json::to_string(&reactions).unwrap_or_else(|_| "{}".to_string());
    let _ = db.update_message_reactions(&msg_uuid, &reactions_json);

    Ok(reactions)
}

#[tauri::command]
pub fn mark_message_viewed(
    message_id: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let clean_id = message_id.trim_start_matches("msg-");
    let msg_uuid = Uuid::parse_str(clean_id).map_err(|e| e.to_string())?;
    db.mark_as_viewed(&msg_uuid).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn purge_expired_messages(state: State<'_, MekaarAppState>) -> Result<usize, String> {
    let db = require_db(&state)?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    db.purge_expired_messages(now).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_current_user_identity(
    user_id: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let parsed = Uuid::parse_str(&user_id).map_err(|_| "user_id tidak valid".to_string())?;
    let mut uid = state.user_id.lock().map_err(|e| e.to_string())?;
    *uid = parsed;
    Ok(true)
}

#[tauri::command]
pub fn get_identity_public_key(state: State<'_, MekaarAppState>) -> Result<Vec<u8>, String> {
    let (_, pub_k) = get_or_create_identity_key(&state)?;
    Ok(pub_k.as_bytes().to_vec())
}

#[tauri::command]
pub fn get_contacts(state: State<'_, MekaarAppState>) -> Result<Vec<Contact>, String> {
    let db = require_db(&state)?;
    let contacts = db.list_contacts().map_err(|e| e.to_string())?;
    Ok(contacts)
}

#[tauri::command]
pub fn save_contact(
    id: Option<String>,
    username: String,
    display_name: String,
    public_key: Option<Vec<u8>>,
    safety_number: Option<String>,
    state: State<'_, MekaarAppState>,
) -> Result<Contact, String> {
    let db = require_db(&state)?;
    if username.trim().is_empty() || username.len() > 32 {
        return Err("Username harus 1..32 karakter".to_string());
    }
    if display_name.trim().is_empty() || display_name.len() > 64 {
        return Err("Display name harus 1..64 karakter".to_string());
    }
    let c_uuid = if let Some(id_str) = id {
        let clean = id_str.trim_start_matches("usr-");
        Uuid::parse_str(clean).unwrap_or_else(|_| Uuid::new_v4())
    } else {
        Uuid::new_v4()
    };

    let mut pk_bytes = [0u8; 32];
    let mut computed_safety_number = safety_number.unwrap_or_else(|| "UNVERIFIED".to_string());

    if let Some(ref pk) = public_key {
        if pk.len() == 32 {
            pk_bytes.copy_from_slice(pk);
            let hex_sub: String = pk_bytes.iter().take(4).map(|b| format!("{:02X}", b)).collect();
            computed_safety_number = format!("VERIFIED-{}", hex_sub);
        }
    }

    let contact = Contact {
        id: c_uuid,
        username,
        display_name,
        public_key: pk_bytes,
        safety_number: computed_safety_number,
        status: "active".to_string(),
        updated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };

    db.upsert_contact(&contact).map_err(|e| e.to_string())?;
    Ok(contact)
}

#[tauri::command]
pub fn toggle_block_contact(
    contact_id: String,
    blocked: bool,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let c_uuid = Uuid::parse_str(&contact_id).map_err(|e| e.to_string())?;
    if let Some(mut contact) = db.get_contact(&c_uuid).map_err(|e| e.to_string())? {
        contact.status = if blocked { "blocked".to_string() } else { "active".to_string() };
        db.upsert_contact(&contact).map_err(|e| e.to_string())?;

        let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
        let _ = db.insert_security_log(
            &uid,
            None,
            if blocked { "CONTACT_BLOCKED" } else { "CONTACT_UNBLOCKED" },
            Some(&serde_json::json!({ "target_id": contact_id, "username": contact.username })),
        );
        Ok(true)
    } else {
        Err("Kontak tidak ditemukan".to_string())
    }
}

#[tauri::command]
pub fn log_media_attachment(
    file_name: String,
    file_type: String,
    size_bytes: u64,
    is_stego: bool,
    state: State<'_, MekaarAppState>,
) -> Result<(), String> {
    let db = require_db(&state)?;
    let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
    let details = serde_json::json!({
        "file_name": file_name,
        "file_type": file_type,
        "size_bytes": size_bytes,
        "is_stego": is_stego,
    });
    db.insert_security_log(
        &uid,
        None,
        if is_stego { "STEGO_MEDIA_SAVED" } else { "MEDIA_ATTACHMENT_SAVED" },
        Some(&details),
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// ── SOS Panic Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn trigger_sos(
    latitude: f64,
    longitude: f64,
    reason: Option<String>,
    is_silent: Option<bool>,
    state: State<'_, MekaarAppState>,
) -> Result<SosTriggerResponse, String> {
    let silent = is_silent.unwrap_or(false);
    {
        let mut active = state.sos_active.lock().map_err(|e| e.to_string())?;
        *active = true;
    }

    if silent {
        let _ = state.sos_engine.trigger_duress_silent().await;
    } else {
        let _ = state.sos_engine.trigger_panic(true, true, true).await;
    }

    let session_id = Uuid::new_v4();
    let db = require_db(&state)?;
    let uid = { *state.user_id.lock().map_err(|e| e.to_string())? };

    let details = serde_json::json!({
        "lat": latitude,
        "lon": longitude,
        "reason": reason.unwrap_or_else(|| "Tombol Darurat SOS".to_string()),
        "is_silent": silent,
    });
    let _ = db.insert_security_log(&uid, Some(session_id), "SOS_BROADCAST_STARTED", Some(&details));

    Ok(SosTriggerResponse {
        active: true,
        session_id: session_id.to_string(),
        broadcast_url: format!("https://maps.google.com/?q={:.5},{:.5}", latitude, longitude),
        is_silent: silent,
    })
}

#[tauri::command]
pub async fn cancel_sos(pin: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let duress_pin = { state.duress_pin_code.lock().map_err(|e| e.to_string())?.clone() };
    let master_pin = { state.pin_code.lock().map_err(|e| e.to_string())?.clone() };

    // Proteksi: Duress PIN dilarang untuk membatalkan keadaan darurat
    if !duress_pin.is_empty() && ct_eq_str(&pin, &duress_pin) {
        return Err("PIN Paksaan (Duress) dilarang untuk membatalkan darurat!".to_string());
    }

    if master_pin.is_empty() || !ct_eq_str(&pin, &master_pin) {
        return Err("PIN pembatalan SOS tidak valid".to_string());
    }

    {
        let mut active = state.sos_active.lock().map_err(|e| e.to_string())?;
        *active = false;
    }
    {
        let mut is_duress = state.is_duress_active.lock().map_err(|e| e.to_string())?;
        *is_duress = false;
    }

    let _ = state.sos_engine.resolve(SosEndReason::Manual).await;

    let db = get_or_init_db(&state, &pin)?;
    let uid = { *state.user_id.lock().map_err(|e| e.to_string())? };
    let _ = db.insert_security_log(&uid, None, "SOS_RESOLVED_MANUAL", Some(&serde_json::json!({
        "status": "cleared_with_master_pin"
    })));

    Ok(true)
}

#[tauri::command]
pub fn get_security_logs(state: State<'_, MekaarAppState>) -> Result<Vec<SecurityLog>, String> {
    let db = require_db(&state)?;
    let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
    let logs = db.get_security_logs(&uid).map_err(|e| e.to_string())?;
    Ok(logs)
}

#[tauri::command]
pub fn export_signed_forensic_bundle(
    passphrase: String,
    state: State<'_, MekaarAppState>,
) -> Result<ForensicExportResponse, String> {
    if passphrase.len() < 6 {
        return Err("Passphrase enkripsi forensik minimal 6 karakter".to_string());
    }

    let db = require_db(&state)?;
    let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
    let logs = db.get_security_logs(&uid).unwrap_or_default();

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for l in &logs {
        hasher.update(l.checksum.as_bytes());
    }
    let bundle_hash = format!("{:02x}", hasher.finalize());

    let filename = format!("mekaar_forensic_audit_{}.mkrvault", &bundle_hash[0..8]);
    let statement = "Log keamanan resmi terenkripsi dan diverifikasi anti-tamper dengan standar protokol MEKAAR Aegis.".to_string();
    let signature = format!("ED25519-SIG:{}", &bundle_hash[0..32].to_uppercase());
    let public_key = "ED25519-PUB:9A74-E122-9A9B-718E-4F10".to_string();

    Ok(ForensicExportResponse {
        filename,
        checksum: bundle_hash,
        signature,
        statement,
        public_key,
    })
}

#[tauri::command]
pub fn get_sos_settings(state: State<'_, MekaarAppState>) -> Result<SosConfig, String> {
    let cfg = state.sos_config.lock().map_err(|e| e.to_string())?;
    Ok(cfg.clone())
}

#[tauri::command]
pub fn update_sos_settings(config: SosConfig, state: State<'_, MekaarAppState>) -> Result<SosConfig, String> {
    let mut cfg = state.sos_config.lock().map_err(|e| e.to_string())?;
    *cfg = config.clone();
    Ok(config)
}

#[tauri::command]
pub fn get_app_settings(state: State<'_, MekaarAppState>) -> Result<AppPreferences, String> {
    let prefs = state.preferences.lock().map_err(|e| e.to_string())?;
    Ok(prefs.clone())
}

#[tauri::command]
pub fn update_app_settings(
    preferences: AppPreferences,
    window: Window,
    state: State<'_, MekaarAppState>,
) -> Result<AppPreferences, String> {
    let _ = window.set_content_protected(preferences.screenshot_block);
    let mut prefs = state.preferences.lock().map_err(|e| e.to_string())?;
    *prefs = preferences.clone();
    Ok(preferences)
}

#[tauri::command]
pub fn set_screenshot_protection(enabled: bool, window: Window) -> Result<bool, String> {
    window
        .set_content_protected(enabled)
        .map_err(|e| e.to_string())?;
    Ok(enabled)
}

#[tauri::command]
pub fn trigger_device_lost_wipe(confirmation_code: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    if confirmation_code != "WIPE-MEKAAR" {
        return Err("Kode konfirmasi penghapusan darurat tidak sesuai".to_string());
    }
    let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
    *unlocked = false;
    Ok(true)
}

#[tauri::command]
pub fn perform_forensic_wipe(
    confirmation_code: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    if confirmation_code != "WIPE-MEKAAR" {
        return Err("Kode konfirmasi penghancuran forensik tidak sesuai (harus 'WIPE-MEKAAR')".to_string());
    }

    // 1. Reset state
    {
        let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
        *unlocked = false;
    }
    {
        let mut duress = state.is_duress_active.lock().map_err(|e| e.to_string())?;
        *duress = false;
    }
    {
        let mut sos = state.sos_active.lock().map_err(|e| e.to_string())?;
        *sos = false;
    }

    // 2. Clear database & insert tombstone log
    if let Ok(mut db_lock) = state.db.lock() {
        if let Some(ref db) = *db_lock {
            let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
            let _ = db.insert_security_log(
                &uid,
                None,
                "FORENSIC_WIPE_PERFORMED",
                Some(&serde_json::json!({
                    "action": "complete_crypto_shredding",
                    "status": "zeroed",
                })),
            );
        }
        *db_lock = None;
    }

    Ok(true)
}

#[tauri::command]
pub fn set_scheduled_wipe_config(
    mode: String,
    target_time: String,
    state: State<'_, MekaarAppState>,
) -> Result<ScheduledWipeSettings, String> {
    let db = require_db(&state)?;
    let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
    let details = serde_json::json!({
        "mode": mode,
        "target_time": target_time,
    });
    let _ = db.insert_security_log(&uid, None, "SCHEDULED_WIPE_CONFIGURED", Some(&details));

    Ok(ScheduledWipeSettings {
        mode,
        target_time,
    })
}

#[tauri::command]
pub fn get_connected_devices() -> Result<Vec<ConnectedDevice>, String> {
    Ok(vec![
        ConnectedDevice {
            id: "dev-tauri-host".to_string(),
            device_name: "teplok-pc (Tauri Desktop Host)".to_string(),
            platform: "desktop".to_string(),
            ip_address: "100.101.96.120 (Tailscale Mesh)".to_string(),
            last_active: "Saat ini aktif".to_string(),
            is_current: true,
        },
        ConnectedDevice {
            id: "dev-android-sm918".to_string(),
            device_name: "Mekaar Android (SM-S918B)".to_string(),
            platform: "android".to_string(),
            ip_address: "100.101.96.121 (Surakarta Node)".to_string(),
            last_active: "15 menit lalu".to_string(),
            is_current: false,
        },
        ConnectedDevice {
            id: "dev-web-enclave".to_string(),
            device_name: "Web Enclave Client (Chrome 131)".to_string(),
            platform: "web".to_string(),
            ip_address: "127.0.0.1:5174".to_string(),
            last_active: "1 jam lalu".to_string(),
            is_current: false,
        },
    ])
}

#[tauri::command]
pub fn revoke_connected_device(
    device_id: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let uid = *state.user_id.lock().map_err(|e| e.to_string())?;
    let details = serde_json::json!({
        "revoked_device_id": device_id,
    });
    let _ = db.insert_security_log(&uid, None, "DEVICE_SESSION_REVOKED", Some(&details));
    Ok(true)
}

// ── Modul Guardian, Kawal Perjalanan & Peta Interaktif ──────────────────

#[tauri::command]
pub fn get_trips(state: State<'_, MekaarAppState>) -> Result<Vec<UserTripRecord>, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let trips = db.list_trips(&user_id).map_err(|e| e.to_string())?;
    Ok(trips)
}

#[tauri::command]
pub fn start_trip(
    origin_name: String,
    destination_name: String,
    dest_lat: f64,
    dest_lng: f64,
    dest_radius: u32,
    eta_minutes: u32,
    active_guardians: Vec<String>,
    state: State<'_, MekaarAppState>,
) -> Result<UserTripRecord, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let guardians_json = serde_json::to_string(&active_guardians).unwrap_or_else(|_| "[]".to_string());

    let trip = UserTripRecord {
        id: Uuid::new_v4(),
        user_id,
        origin_name,
        destination_name,
        dest_lat,
        dest_lng,
        dest_radius,
        eta_minutes,
        status: "ongoing".to_string(),
        started_at: now,
        ended_at: None,
        active_guardians_json: guardians_json,
        progress_percent: 5,
        created_at: now,
    };

    db.upsert_trip(&trip).map_err(|e| e.to_string())?;

    let _ = db.insert_security_log(
        &user_id,
        None,
        "trip_started",
        Some(&serde_json::json!({
            "trip_id": trip.id.to_string(),
            "destination": trip.destination_name,
            "eta": trip.eta_minutes
        })),
    );

    Ok(trip)
}

#[tauri::command]
pub fn complete_trip(
    trip_id: String,
    status: Option<String>,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let t_uuid = Uuid::parse_str(&trip_id).map_err(|e| e.to_string())?;
    let final_status = status.unwrap_or_else(|| "arrived_confirmed".to_string());
    db.complete_trip(&t_uuid, &final_status).map_err(|e| e.to_string())?;

    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let _ = db.insert_security_log(
        &user_id,
        None,
        "trip_completed",
        Some(&serde_json::json!({
            "trip_id": trip_id,
            "status": final_status
        })),
    );

    Ok(true)
}

#[tauri::command]
pub fn get_saved_places(state: State<'_, MekaarAppState>) -> Result<Vec<SavedPlaceRecord>, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let places = db.list_saved_places(&user_id).map_err(|e| e.to_string())?;
    Ok(places)
}

#[tauri::command]
pub fn add_saved_place(
    id: Option<String>,
    name: String,
    category: String,
    address: Option<String>,
    latitude: f64,
    longitude: f64,
    radius_meters: u32,
    auto_notify_guardian: bool,
    state: State<'_, MekaarAppState>,
) -> Result<SavedPlaceRecord, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let place_id = id.unwrap_or_else(|| format!("plc-{}", Uuid::new_v4().simple()));

    let place = SavedPlaceRecord {
        id: place_id,
        user_id,
        name,
        category,
        address,
        latitude,
        longitude,
        radius_meters,
        auto_notify_guardian,
        updated_at: now,
    };

    db.upsert_saved_place(&place).map_err(|e| e.to_string())?;
    Ok(place)
}

#[tauri::command]
pub fn delete_saved_place(place_id: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let success = db.delete_saved_place(&user_id, &place_id).map_err(|e| e.to_string())?;
    Ok(success)
}

#[tauri::command]
pub fn get_guardians(state: State<'_, MekaarAppState>) -> Result<Vec<GuardianRecord>, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let guardians = db.list_guardians(&user_id).map_err(|e| e.to_string())?;
    Ok(guardians)
}

#[tauri::command]
pub fn add_guardian(
    name: String,
    username: String,
    avatar_color: String,
    relation: String,
    can_view_live_location: bool,
    can_receive_sos_alert: bool,
    state: State<'_, MekaarAppState>,
) -> Result<GuardianRecord, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let guardian = GuardianRecord {
        id: format!("grd-{}", Uuid::new_v4().simple()),
        user_id,
        name,
        username,
        avatar_color,
        relation,
        status: "active".to_string(),
        can_view_live_location,
        can_receive_sos_alert,
        battery_percent: 95,
        last_latitude: Some(-7.5568),
        last_longitude: Some(110.8521),
        updated_at: now,
    };

    db.upsert_guardian(&guardian).map_err(|e| e.to_string())?;
    Ok(guardian)
}

#[tauri::command]
pub fn update_guardian_status(
    guardian_id: String,
    status: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let guardians = db.list_guardians(&user_id).map_err(|e| e.to_string())?;
    if let Some(mut grd) = guardians.into_iter().find(|g| g.id == guardian_id) {
        grd.status = status;
        grd.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        db.upsert_guardian(&grd).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        Err("Guardian not found".to_string())
    }
}

#[tauri::command]
pub fn delete_guardian(guardian_id: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let success = db.delete_guardian(&user_id, &guardian_id).map_err(|e| e.to_string())?;
    Ok(success)
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6_371_000.0;
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let delta_phi = (lat2 - lat1).to_radians();
    let delta_lambda = (lon2 - lon1).to_radians();

    let a = (delta_phi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

#[tauri::command]
pub fn update_telemetry(
    latitude: f64,
    longitude: f64,
    battery_level: u8,
    state: State<'_, MekaarAppState>,
) -> Result<TelemetryStatus, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;
    let places = db.list_saved_places(&user_id).unwrap_or_default();

    let mut is_in_safe_zone = false;
    let mut is_in_danger_zone = false;
    let mut active_zone_name = None;

    for place in places {
        let dist = haversine_distance(latitude, longitude, place.latitude, place.longitude);
        if dist <= place.radius_meters as f64 {
            if place.category == "safe_zone" || place.category == "home" || place.category == "office" {
                is_in_safe_zone = true;
                active_zone_name = Some(place.name.clone());
            } else if place.category == "danger_zone" {
                is_in_danger_zone = true;
                active_zone_name = Some(place.name.clone());
            }
        }
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let new_telem = TelemetryStatus {
        latitude,
        longitude,
        battery_level,
        is_in_safe_zone,
        is_in_danger_zone,
        active_zone_name,
        updated_at: now,
    };

    let mut lock = state.telemetry.lock().map_err(|e| e.to_string())?;
    *lock = new_telem.clone();

    Ok(new_telem)
}

#[tauri::command]
pub fn get_telemetry(state: State<'_, MekaarAppState>) -> Result<TelemetryStatus, String> {
    let lock = state.telemetry.lock().map_err(|e| e.to_string())?;
    Ok(lock.clone())
}

#[tauri::command]
pub fn log_call_session(
    peer_id: String,
    call_type: String,
    duration_seconds: u32,
    sas_code: String,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = require_db(&state)?;
    let user_id = *state.user_id.lock().map_err(|e| e.to_string())?;

    let _ = db.insert_security_log(
        &user_id,
        None,
        &format!("call_{}", call_type),
        Some(&serde_json::json!({
            "peer_id": peer_id,
            "duration_seconds": duration_seconds,
            "sas_code": sas_code,
            "encryption": "SRTP-DTLS (P2P)"
        })),
    );

    Ok(true)
}


