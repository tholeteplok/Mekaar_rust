//! # mekaar-storage: Local Domain Models
//!
//! Model data lokal yang 100% diselaraskan dengan prototipe Flutter `Mekaar_chat`.

use mekaar_proto::MessageType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Status pengiriman pesan lokal.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    /// Menunggu di antrean offline lokal.
    Queued,
    /// Berhasil terkirim ke server relay.
    Sent,
    /// Telah sampai di perangkat lawan bicara.
    Delivered,
    /// Telah dibaca oleh lawan bicara.
    Read,
    /// Gagal terkirim.
    Failed,
}

impl DeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeliveryStatus::Queued => "queued",
            DeliveryStatus::Sent => "sent",
            DeliveryStatus::Delivered => "delivered",
            DeliveryStatus::Read => "read",
            DeliveryStatus::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "sent" => DeliveryStatus::Sent,
            "delivered" => DeliveryStatus::Delivered,
            "read" => DeliveryStatus::Read,
            "failed" => DeliveryStatus::Failed,
            _ => DeliveryStatus::Queued,
        }
    }
}

/// Struktur data pesan obrolan yang disimpan di penyimpanan lokal terenkripsi.
/// Model ini mencakup seluruh atribut dari `message_model.dart` pada Flutter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
    pub media_url: Option<String>,
    pub msg_type: MessageType,
    pub is_view_once: bool,
    pub is_opened: bool,
    pub opened_at: Option<u64>,
    pub reply_to_id: Option<Uuid>,
    pub is_deleted: bool,
    pub is_silent_deleted: bool,
    pub auto_delete_at: Option<u64>,
    pub edited_at: Option<u64>,
    pub reactions: HashMap<String, Vec<String>>,
    pub is_encrypted: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub status: DeliveryStatus,
}

/// Struktur data ruang obrolan (Room) lokal.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChatRoom {
    pub id: Uuid,
    pub room_type: String, // "normal", "guardian", "self_device", "group"
    pub name: String,
    pub peer_id: Option<Uuid>,
    pub disappearing_timer: Option<u64>, // milliseconds (misal 3600000 = 1 jam)
    pub theme_preset: String,           // 11 presets: "mekaar", "neon_dreams", "comic_pop", dll
    pub last_message_at: u64,
    pub is_archived: bool,
    pub is_muted: bool,
    pub is_vault_hidden: bool, // true jika disembunyikan di balik PIN Private Vault
}

/// Struktur data kontak lokal.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub public_key: [u8; 32],
    pub safety_number: String,
    pub status: String, // "active", "blocked"
    pub updated_at: u64,
}

/// Log audit bukti hukum permanen anti-tamper dengan verifikasi checksum SHA-256.
/// Selaras dengan `security_log_model.dart` dari Flutter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SecurityLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sos_session_id: Option<Uuid>,
    pub event_type: String, // "sos_started", "msg_deleted", "pin_locked", dll
    pub details_json: Option<String>,
    pub created_at: u64,
    pub checksum: String,
}

/// Sesi darurat SOS lokal.
/// Selaras dengan `sos_session_model.dart` dari Flutter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SosSessionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub guardian_id: Option<Uuid>,
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub status: String, // "active", "ended", "auto_ended"
    pub gps_enabled: bool,
    pub mic_enabled: bool,
    pub video_enabled: bool,
    pub ended_reason: Option<String>, // "manual", "timer", "inactivity"
    pub created_at: u64,
}

/// Struktur data sesi perjalanan dan kawal Guardian.
/// Selaras dengan `trip_model.dart` dari Flutter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserTripRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub origin_name: String,
    pub destination_name: String,
    pub dest_lat: f64,
    pub dest_lng: f64,
    pub dest_radius: u32,
    pub eta_minutes: u32,
    pub status: String, // "ongoing", "arrived_auto", "arrived_confirmed", "delayed_alerted", "completed"
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub active_guardians_json: String, // JSON array of guardian names
    pub progress_percent: u32,
    pub created_at: u64,
}

/// Struktur data zona aman / lokasi tersimpan.
/// Selaras dengan `saved_place_model.dart` dari Flutter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SavedPlaceRecord {
    pub id: String, // e.g. "plc-01", "home", "office"
    pub user_id: Uuid,
    pub name: String,
    pub category: String, // "home", "office", "safe_zone", "favorite"
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_meters: u32,
    pub auto_notify_guardian: bool,
    pub updated_at: u64,
}

/// Struktur data kontak Guardian pelindung.
/// Selaras dengan `guardian_model.dart` dari Flutter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GuardianRecord {
    pub id: String,
    pub user_id: Uuid,
    pub name: String,
    pub username: String,
    pub avatar_color: String,
    pub relation: String,
    pub status: String, // "active", "standby", "pending_request"
    pub can_view_live_location: bool,
    pub can_receive_sos_alert: bool,
    pub battery_percent: u8,
    pub last_latitude: Option<f64>,
    pub last_longitude: Option<f64>,
    pub updated_at: u64,
}
