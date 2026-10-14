//! # mekaar-proto: Error Types
//!
//! Error yang mungkin terjadi saat encoding/decoding protokol wire MEKAAR.

use thiserror::Error;

/// Seluruh kemungkinan kegagalan protokol wire.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtoError {
    /// Versi protokol tidak didukung.
    #[error("Versi wire protocol tidak didukung: {0}")]
    UnsupportedVersion(u8),

    /// Kegagalan serialisasi atau deserialisasi Postcard (biner).
    #[error("Postcard encoding/decoding error: {0}")]
    Postcard(String),

    /// Kegagalan serialisasi atau deserialisasi JSON.
    #[error("JSON encoding/decoding error: {0}")]
    Json(String),

    /// Format paket atau header tidak valid.
    #[error("Paket wire rusak atau format tidak valid: {0}")]
    InvalidPacket(String),
}
