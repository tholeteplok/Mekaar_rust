use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{State, Window};

pub const DEFAULT_TAILSCALE_HTTP: &str = "http://100.101.96.120:8085";
pub const DEFAULT_TAILSCALE_WS: &str = "ws://100.101.96.120:8085/ws";
pub const DEFAULT_LOCAL_HTTP: &str = "http://127.0.0.1:8085";
pub const DEFAULT_LOCAL_WS: &str = "ws://127.0.0.1:8085/ws";

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
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub endpoint: String,
    pub latency_ms: u64,
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

#[derive(Default)]
pub struct MekaarAppState {
    pub vault_unlocked: Mutex<bool>,
    pub pin_code: Mutex<String>,
    pub server_config: Mutex<ServerConnectionConfig>,
    pub sos_config: Mutex<SosConfig>,
    pub sos_active: Mutex<bool>,
    pub preferences: Mutex<AppPreferences>,
}

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
pub fn unlock_vault(passphrase: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    if passphrase.trim().len() < 4 {
        return Err("Passphrase brankas minimal 4 karakter".to_string());
    }
    let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
    *unlocked = true;
    Ok(true)
}

#[tauri::command]
pub fn lock_vault(state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let mut unlocked = state.vault_unlocked.lock().map_err(|e| e.to_string())?;
    *unlocked = false;
    Ok(true)
}

#[tauri::command]
pub fn verify_pin(pin: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let saved_pin = state.pin_code.lock().map_err(|e| e.to_string())?;
    let expected = if saved_pin.is_empty() { "123456" } else { saved_pin.as_str() };
    Ok(pin == expected)
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
pub fn trigger_sos(
    latitude: f64,
    longitude: f64,
    reason: Option<String>,
    state: State<'_, MekaarAppState>,
) -> Result<String, String> {
    let mut active = state.sos_active.lock().map_err(|e| e.to_string())?;
    *active = true;
    Ok(format!(
        "SOS_ACTIVE@{:.5},{:.5}:{}",
        latitude,
        longitude,
        reason.unwrap_or_else(|| "Darurat Manual".to_string())
    ))
}

#[tauri::command]
pub fn cancel_sos(pin: String, state: State<'_, MekaarAppState>) -> Result<bool, String> {
    let saved_pin = state.pin_code.lock().map_err(|e| e.to_string())?;
    let expected = if saved_pin.is_empty() { "123456" } else { saved_pin.as_str() };
    if pin != expected {
        return Err("PIN pembatalan SOS tidak valid".to_string());
    }
    let mut active = state.sos_active.lock().map_err(|e| e.to_string())?;
    *active = false;
    Ok(true)
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
pub fn export_forensic_bundle(
    conversation_id: String,
    passphrase: String,
    include_media: bool,
) -> Result<String, String> {
    if passphrase.len() < 6 {
        return Err("Passphrase enkripsi ekspor minimal 6 karakter".to_string());
    }
    Ok(format!(
        "mekaar_forensic_{}_media-{}_blake3-signed.mkrvault",
        conversation_id, include_media
    ))
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
