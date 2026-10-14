//! # mekaar-proto: Wire Commands & Signaling
//!
//! Tipe data perintah wire untuk pertukaran pesan, signaling WebRTC, dan notifikasi krisis SOS
//! melalui koneksi WebSocket antara Klien Tauri v2 dan Relay Server di teplok-pc.

use crate::envelope::MessageEnvelope;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Perintah yang dikirim dari Klien ke Relay Server.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClientCommand {
    /// Inisialisasi koneksi klien: daftarkan session socket ke user_id dan publikasikan kunci publik.
    AuthSubscribe {
        user_id: Uuid,
        public_key: [u8; 32],
    },

    /// Kirim envelope pesan terenkripsi ke penerima.
    SendEnvelope(MessageEnvelope),

    /// Konfirmasi tanda terima pesan (Acknowledgment) dari klien penerima.
    DeliveryAck {
        message_id: Uuid,
        sender_id: Uuid,
    },

    /// Pemicu sinyal darurat SOS darurat tinggi.
    SosTrigger {
        user_id: Uuid,
        latitude: f64,
        longitude: f64,
        accuracy: f32,
        timestamp_ms: u64,
    },

    /// Penyelesaian atau pembatalan status darurat SOS.
    SosResolve {
        user_id: Uuid,
        reason: String,
    },

    /// Pertukaran sinyal WebRTC P2P (SDP Offer/Answer atau ICE Candidate).
    WebRtcSignal {
        target_user_id: Uuid,
        signal_type: String,
        payload: String,
    },

    /// Heartbeat ping untuk menjaga koneksi TCP/WebSocket tetap hidup.
    Ping,
}

/// Perintah atau respon yang dikirim dari Relay Server ke Klien.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ServerCommand {
    /// Konfirmasi pendaftaran sesi klien berhasil.
    SubscribedAck {
        user_id: Uuid,
    },

    /// Pesan terenkripsi masuk yang diteruskan oleh server ke penerima.
    IncomingEnvelope(MessageEnvelope),

    /// Konfirmasi bahwa pesan telah berhasil terkirim ke server atau ke penerima.
    DeliveryReceipt {
        message_id: Uuid,
        timestamp_ms: u64,
    },

    /// Siaran darurat SOS prioritas tinggi untuk para Guardian aktif korban.
    GuardianSosBroadcast {
        victim_id: Uuid,
        latitude: f64,
        longitude: f64,
        accuracy: f32,
        timestamp_ms: u64,
    },

    /// Relay sinyal WebRTC dari lawan bicara.
    WebRtcRelay {
        from_user_id: Uuid,
        signal_type: String,
        payload: String,
    },

    /// Heartbeat pong respon dari server.
    Pong,

    /// Pesan error dari server ke klien.
    Error {
        code: u16,
        message: String,
    },
}
