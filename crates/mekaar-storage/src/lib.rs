//! # mekaar-storage — Database Lokal Terenkripsi Penuh (SQLCipher)
//!
//! Abstraksi penyimpanan data lokal terenkripsi AES-256-CBC untuk klien MEKAAR:
//! - Pesan obrolan lokal dengan siklus View-Once dan timer penghapusan otomatis.
//! - Ruang obrolan (Rooms) dengan dukungan 11 preset Colorful Room dan Private Vault.
//! - Buku kontak dan safety number verifikasi identitas.
//! - Log audit bukti hukum permanen anti-tamper dengan verifikasi SHA-256 checksum.

pub mod db;
pub mod error;
pub mod models;

pub use db::EncryptedDatabase;
pub use error::StorageError;
pub use models::{
    ChatRoom, Contact, DeliveryStatus, GuardianRecord, LocalMessage, SavedPlaceRecord, SecurityLog,
    SosSessionRecord, UserTripRecord,
};
