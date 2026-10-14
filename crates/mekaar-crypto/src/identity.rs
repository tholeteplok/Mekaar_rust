//! # mekaar-crypto: Identity & Fingerprinting
//!
//! Manajemen keypair identitas X25519 dan kalkulasi Safety Number / Fingerprint.

use crate::types::{IdentityPrivateKey, IdentityPublicKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey, StaticSecret};

/// Menghasilkan pasangan kunci identitas baru (X25519) menggunakan CSPRNG OS.
pub fn generate_identity() -> (IdentityPrivateKey, IdentityPublicKey) {
    let secret = StaticSecret::random_from_rng(&mut OsRng);
    let public = PublicKey::from(&secret);

    let priv_key = IdentityPrivateKey::from_bytes(secret.to_bytes());
    let pub_key = IdentityPublicKey::from_bytes(public.to_bytes());

    (priv_key, pub_key)
}

/// Menghasilkan keypair identitas dari 32-byte seed deterministik.
pub fn identity_from_seed(seed: [u8; 32]) -> (IdentityPrivateKey, IdentityPublicKey) {
    let secret = StaticSecret::from(seed);
    let public = PublicKey::from(&secret);

    let priv_key = IdentityPrivateKey::from_bytes(secret.to_bytes());
    let pub_key = IdentityPublicKey::from_bytes(public.to_bytes());

    (priv_key, pub_key)
}

/// Menghitung sidik jari (Safety Number) dari kunci publik identitas.
///
/// Algoritma:
/// 1. Hash SHA-256 dari 32-byte kunci publik (menghasilkan 32 byte / 64 digit hex).
/// 2. Format menjadi 16 kelompok hex 4-karakter kapital yang dipisahkan spasi.
///
/// Contoh: `E1A3 89F0 4B2C 9D1A 3F0E 5A8C 7B2D 4E6F 8A1B 2C3D 4E5F 6A7B 8C9D 0E1F 2A3B 4C5D`
///
/// Format ini 100% kompatibel dengan fungsi `getPublicKeyFingerprint` di Flutter prototype.
pub fn compute_fingerprint(pub_key: &IdentityPublicKey) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pub_key.as_bytes());
    let hash = hasher.finalize();

    let hex_chars: String = hash.iter().map(|b| format!("{:02X}", b)).collect();

    let mut chunks = Vec::with_capacity(16);
    for chunk in hex_chars.as_bytes().chunks(4) {
        if let Ok(s) = std::str::from_utf8(chunk) {
            chunks.push(s);
        }
    }

    chunks.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_identity_consistency() {
        let (priv_key, pub_key) = generate_identity();
        let secret = StaticSecret::from(*priv_key.as_bytes());
        let expected_public = PublicKey::from(&secret);

        assert_eq!(pub_key.as_bytes(), expected_public.as_bytes());
    }

    #[test]
    fn test_fingerprint_format() {
        let (_, pub_key) = generate_identity();
        let fp = compute_fingerprint(&pub_key);

        let parts: Vec<&str> = fp.split(' ').collect();
        assert_eq!(parts.len(), 16);
        for part in parts {
            assert_eq!(part.len(), 4);
            assert!(part.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_lowercase()));
        }
    }
}
