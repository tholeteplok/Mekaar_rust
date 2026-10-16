//! # mekaar-crypto: Memory-Safe Key Types
//!
//! Tipe kunci dan rahasia kriptografi yang dilindungi dengan `zeroize::ZeroizeOnDrop`.
//! Material rahasia dibersihkan otomatis dari RAM ketika keluar dari scope.

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Kunci privat identitas pengguna (X25519, 32-byte).
/// Otomatis di-zeroize saat keluar dari scope memori.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct IdentityPrivateKey(pub(crate) [u8; 32]);

impl IdentityPrivateKey {
    /// Membuat instance dari byte array 32-byte mentah.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Mengakses byte mentah kunci privat.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Kunci publik identitas pengguna (X25519, 32-byte).
/// Bersifat publik dan aman untuk dibagikan atau diserialisasi.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct IdentityPublicKey(pub [u8; 32]);

impl IdentityPublicKey {
    /// Membuat instance dari byte array 32-byte mentah.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Mengakses byte mentah kunci publik.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Mengonversi kunci publik ke representasi heksadesimal huruf kecil.
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

/// Kunci rahasia bersama hasil Diffie-Hellman X25519 (32-byte).
/// Otomatis di-zeroize saat keluar dari scope memori.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SharedSecret(pub(crate) [u8; 32]);

impl SharedSecret {
    pub(crate) fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Mengakses byte mentah kunci rahasia bersama.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Kunci sesi simetris untuk enkripsi percakapan per-room (32-byte).
/// Diderivasi melalui HKDF-SHA256.
/// Otomatis di-zeroize saat keluar dari scope memori.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct RoomSessionKey(pub(crate) [u8; 32]);

impl RoomSessionKey {
    /// Membuat kunci sesi dari byte array 32-byte.
    /// `pub(crate)` agar injeksi kunci arbitrer tidak bisa dari luar crate.
    pub(crate) fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Mengakses byte kunci sesi.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Kunci enkripsi acak untuk berkas media lampiran (32-byte).
/// Dibuat acak per berkas.
/// Otomatis di-zeroize saat keluar dari scope memori.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MediaFileKey(pub(crate) [u8; 32]);

impl MediaFileKey {
    /// Membuat kunci media dari byte array 32-byte.
    pub(crate) fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Mengakses byte kunci media.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
