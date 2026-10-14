//! # mekaar-storage: Error Types
//!
//! Error yang mungkin terjadi saat operasi database lokal terenkripsi SQLCipher.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Enkripsi database gagal atau passkey tidak valid: {0}")]
    Encryption(String),

    #[error("Data tidak ditemukan")]
    NotFound,

    #[error("Format data rusak atau tidak valid: {0}")]
    CorruptedData(String),

    #[error("Integritas data audit rusak: Checksum tidak cocok!")]
    ChecksumMismatch,
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::Database(err.to_string())
    }
}
