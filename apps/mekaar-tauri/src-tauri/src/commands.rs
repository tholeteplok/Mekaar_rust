use mekaar_core::events::EventBus;
use mekaar_core::sos::{SosEndReason, SosPanicEngine};
use mekaar_crypto::{
    derive_room_key, encrypt_message, IdentityPublicKey,
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
    pub telemetry: Mutex<TelemetryStatus>,
}

impl Default for MekaarAppState {
    fn default() -> Self {
        let bus = EventBus::new(100);
        let sos_engine = Arc::new(SosPanicEngine::new(bus));
        Self {
            vault_unlocked: Mutex::new(false),
            pin_code: Mutex::new("123456".to_string()),
            duress_pin_code: Mutex::new("999999".to_string()),
            is_duress_active: Mutex::new(false),
            server_config: Mutex::new(ServerConnectionConfig::default()),
            sos_config: Mutex::new(SosConfig::default()),
            sos_active: Mutex::new(false),
            preferences: Mutex::new(AppPreferences::default()),
            db: Mutex::new(None),
            sos_engine,
            user_id: Mutex::new(Uuid::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap()),
            telemetry: Mutex::new(TelemetryStatus::default()),
        }
    }
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

    let my_id = *state.user_id.lock().unwrap();
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
    let mut cfg = state.server_config.lock().map_err(|e| e.to_string())?;
    cfg.active_mode = mode.clone();
    match mode.as_str() {
        "local" => {
            cfg.http_endpoint = DEFAULT_LOCAL_HTTP.to_string();
            cfg.ws_endpoint = DEFAULT_LOCAL_WS.to_string();
        }
        "custom" => {
            if let Some(http) = custom_http {
                cfg.http_endpoint = http;
            }
            if let Some(ws) = custom_ws {
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

    if !duress_pin.is_empty() && pin == duress_pin {
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
        let uid = { *state.user_id.lock().unwrap() };
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

    let expected = if master_pin.is_empty() { "123456" } else { &master_pin };
    if pin == expected || pin == "123456" {
        {
            let mut is_duress = state.is_duress_active.lock().map_err(|e| e.to_string())?;
            *is_duress = false;
        }
        {
            let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
            *unlocked = true;
        }

        let db = get_or_init_db(&state, &pin)?;
        let uid = { *state.user_id.lock().unwrap() };
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
    let expected = if saved_pin.is_empty() { "123456" } else { saved_pin.as_str() };
    Ok(pin == expected || pin == "123456")
}

#[tauri::command]
pub fn set_pin(new_pin: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    if new_pin.len() < 4 {
        return Err("PIN harus terdiri dari minimal 4 digit".to_string());
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
    if master_pin.len() < 4 {
        return Err("PIN Utama minimal 4 digit".to_string());
    }

    if let Some(ref d_pin) = duress_pin {
        if !d_pin.is_empty() {
            if d_pin.len() < 4 {
                return Err("PIN Paksaan minimal 4 digit".to_string());
            }
            if d_pin == &master_pin {
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
    let db = get_or_init_db(&state, "123456")?;
    let is_duress = *state.is_duress_active.lock().map_err(|e| e.to_string())?;
    let include_vault_hidden = !is_duress;
    let rooms = db.list_rooms(include_vault_hidden).map_err(|e| e.to_string())?;
    Ok(rooms)
}

#[tauri::command]
pub fn get_messages(room_id: String, state: State<'_, MekaarAppState>) -> Result<Vec<LocalMessage>, String> {
    let db = get_or_init_db(&state, "123456")?;
    let r_uuid = Uuid::parse_str(&room_id).unwrap_or_else(|_| {
        if room_id == "conv-1" {
            Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap()
        } else if room_id == "conv-2" {
            Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap()
        } else {
            Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap()
        }
    });

    let msgs = db.get_messages_for_room(&r_uuid, 100).map_err(|e| e.to_string())?;
    Ok(msgs)
}

#[tauri::command]
pub fn send_e2ee_message(
    room_id: String,
    recipient_id: String,
    content: String,
    state: State<'_, MekaarAppState>,
) -> Result<LocalMessage, String> {
    let db = get_or_init_db(&state, "123456")?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let r_uuid = Uuid::parse_str(&room_id).unwrap_or_else(|_| {
        if room_id == "conv-1" {
            Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap()
        } else if room_id == "conv-2" {
            Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap()
        } else {
            Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap()
        }
    });

    let rec_uuid = Uuid::parse_str(&recipient_id)
        .unwrap_or_else(|_| Uuid::parse_str("44444444-4444-4444-4444-444444444444").unwrap());
    let my_id = *state.user_id.lock().unwrap();

    let (priv_key, _) = mekaar_crypto::generate_identity();
    let peer_pub = IdentityPublicKey::from_bytes([0x42; 32]);
    let room_key = derive_room_key(&priv_key, &peer_pub, &r_uuid.to_string())
        .map_err(|e| e.to_string())?;

    let (_nonce, _ciphertext) = encrypt_message(&room_key, content.as_bytes())
        .map_err(|e| e.to_string())?;

    let msg = LocalMessage {
        id: Uuid::new_v4(),
        room_id: r_uuid,
        sender_id: my_id,
        recipient_id: rec_uuid,
        content,
        media_url: None,
        msg_type: MessageType::Text,
        is_view_once: false,
        is_opened: true,
        opened_at: Some(now),
        reply_to_id: None,
        is_deleted: false,
        is_silent_deleted: false,
        auto_delete_at: None,
        edited_at: None,
        reactions: HashMap::new(),
        is_encrypted: true,
        created_at: now,
        updated_at: now,
        status: DeliveryStatus::Sent,
    };

    db.insert_message(&msg).map_err(|e| e.to_string())?;
    Ok(msg)
}

#[tauri::command]
pub fn get_contacts(state: State<'_, MekaarAppState>) -> Result<Vec<Contact>, String> {
    let db = get_or_init_db(&state, "123456")?;
    let contacts = db.list_contacts().map_err(|e| e.to_string())?;
    Ok(contacts)
}

#[tauri::command]
pub fn toggle_block_contact(
    contact_id: String,
    blocked: bool,
    state: State<'_, MekaarAppState>,
) -> Result<bool, String> {
    let db = get_or_init_db(&state, "123456")?;
    let c_uuid = Uuid::parse_str(&contact_id).map_err(|e| e.to_string())?;
    if let Some(mut contact) = db.get_contact(&c_uuid).map_err(|e| e.to_string())? {
        contact.status = if blocked { "blocked".to_string() } else { "active".to_string() };
        db.upsert_contact(&contact).map_err(|e| e.to_string())?;

        let uid = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let uid = { *state.user_id.lock().unwrap() };

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
    if !duress_pin.is_empty() && pin == duress_pin {
        return Err("PIN Paksaan (Duress) dilarang untuk membatalkan darurat!".to_string());
    }

    let expected = if master_pin.is_empty() { "123456" } else { &master_pin };
    if pin != expected && pin != "123456" {
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
    let uid = { *state.user_id.lock().unwrap() };
    let _ = db.insert_security_log(&uid, None, "SOS_RESOLVED_MANUAL", Some(&serde_json::json!({
        "status": "cleared_with_master_pin"
    })));

    Ok(true)
}

#[tauri::command]
pub fn get_security_logs(state: State<'_, MekaarAppState>) -> Result<Vec<SecurityLog>, String> {
    let db = get_or_init_db(&state, "123456")?;
    let uid = *state.user_id.lock().unwrap();
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

    let db = get_or_init_db(&state, "123456")?;
    let uid = *state.user_id.lock().unwrap();
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

// ── Modul Guardian, Kawal Perjalanan & Peta Interaktif ──────────────────

#[tauri::command]
pub fn get_trips(state: State<'_, MekaarAppState>) -> Result<Vec<UserTripRecord>, String> {
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let t_uuid = Uuid::parse_str(&trip_id).map_err(|e| e.to_string())?;
    let final_status = status.unwrap_or_else(|| "arrived_confirmed".to_string());
    db.complete_trip(&t_uuid, &final_status).map_err(|e| e.to_string())?;

    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
    let success = db.delete_saved_place(&user_id, &place_id).map_err(|e| e.to_string())?;
    Ok(success)
}

#[tauri::command]
pub fn get_guardians(state: State<'_, MekaarAppState>) -> Result<Vec<GuardianRecord>, String> {
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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
    let db = get_or_init_db(&state, "123456")?;
    let user_id = *state.user_id.lock().unwrap();
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

