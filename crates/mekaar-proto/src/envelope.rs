//! # mekaar-proto: Message Envelope
//!
//! Definisi envelope pesan terenkripsi untuk transmisi wire dan format penyimpanan.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tipe pesan yang didukung sistem MEKAAR.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// Pesan teks biasa.
    Text,
    /// Berkas gambar terenkripsi.
    Image,
    /// Pesan suara / audio note terenkripsi.
    Voice,
    /// Siaran video / klip pendek terenkripsi.
    Video,
    /// Koordinat lokasi darurat atau titik temu terenkripsi.
    Location,
    /// Pesan kontrol sistem (misal: panggilan darurat, event join).
    System,
}

/// Paket envelope pesan terenkripsi E2EE.
///
/// Server relay mandiri di teplok-pc bertindak murni sebagai Blind Relay:
/// hanya membaca `sender_id`, `recipient_id`, dan `room_id` untuk routing.
/// Bagian `ciphertext` tidak dapat dibuka oleh server karena server tidak memiliki kunci sesi.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageEnvelope {
    /// Versi protokol envelope (saat ini: 1).
    pub version: u8,
    /// ID unik pesan.
    pub message_id: Uuid,
    /// ID pengguna pengirim.
    pub sender_id: Uuid,
    /// ID pengguna penerima.
    pub recipient_id: Uuid,
    /// ID ruang obrolan.
    pub room_id: Uuid,
    /// Nonce 24-byte untuk dekripsi XChaCha20-Poly1305.
    pub nonce: [u8; 24],
    /// Ciphertext terenkripsi (termasuk tag MAC 16-byte).
    pub ciphertext: Vec<u8>,
    /// Waktu pengiriman (epoch milliseconds).
    pub timestamp_ms: u64,
    /// Tipe konten pesan (membantu klien menentukan alur parsing UI).
    pub msg_type: MessageType,
    /// Flag media sekali lihat (View-Once).
    pub is_view_once: bool,
    /// Waktu kedaluwarsa otomatis (Disappearing messages), dalam epoch ms.
    pub expires_at: Option<u64>,
}

impl MessageEnvelope {
    /// Membuat envelope pesan baru dengan versi default 1.
    pub fn new(
        sender_id: Uuid,
        recipient_id: Uuid,
        room_id: Uuid,
        nonce: [u8; 24],
        ciphertext: Vec<u8>,
        timestamp_ms: u64,
        msg_type: MessageType,
    ) -> Self {
        Self {
            version: 1,
            message_id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            room_id,
            nonce,
            ciphertext,
            timestamp_ms,
            msg_type,
            is_view_once: false,
            expires_at: None,
        }
    }

    /// Menandai pesan sebagai view-once (sekali lihat).
    pub fn with_view_once(mut self, view_once: bool) -> Self {
        self.is_view_once = view_once;
        self
    }

    /// Menetapkan batas waktu kedaluwarsa (disappearing message timer).
    pub fn with_expiry(mut self, expires_at: Option<u64>) -> Self {
        self.expires_at = expires_at;
        self
    }
}
