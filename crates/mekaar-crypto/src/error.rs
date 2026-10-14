//! # mekaar-crypto: Error Types
//!
//! Tipe error untuk seluruh operasi kriptografi Aegis Protocol.
//! Menerapkan prinsip *fail-closed*: kegagalan enkripsi/dekripsi/derivasi
//! selalu menghasilkan error eksplisit dan tidak pernah menghasilkan plaintext.

use thiserror::Error;

/// Seluruh kemungkinan kegagalan operasi kriptografi Aegis.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CryptoError {
    /// Format kunci publik tidak valid (misal: panjang byte salah).
    #[error("Kunci lawan bicara tidak valid atau rusak")]
    InvalidPeerKey,

    /// Otentikasi tag MAC Poly1305 gagal atau ciphertext rusak / di-tamper.
    #[error("Gagal melakukan dekripsi (otentikasi MAC gagal atau data rusak)")]
    DecryptionFailed,

    /// Kegagalan dalam proses ekspansi HKDF-SHA256.
    #[error("Kegagalan derivasi kunci sesi")]
    KeyDerivationFailed,

    /// Kegagalan derivasi PIN melalui Argon2id.
    #[error("Kegagalan derivasi PIN Argon2id: {0}")]
    PinDerivationFailed(String),

    /// Sumber entropi acak aman gagal.
    #[error("Kegagalan random number generator OS")]
    RngFailure,

    /// Format data terenkripsi atau salt tidak sesuai format yang diharapkan.
    #[error("Format paket kripto tidak valid: {0}")]
    InvalidFormat(String),

    /// Percobaan unwrap private key gagal (PIN salah).
    #[error("PIN pemulihan salah atau data backup rusak")]
    InvalidPinOrCorruptedBackup,
}
