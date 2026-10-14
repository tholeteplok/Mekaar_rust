//! # mekaar-crypto: Symmetric AEAD Cipher
//!
//! Enkripsi dan dekripsi simetris menggunakan **XChaCha20-Poly1305**.
//! - Nonce: 24-byte acak dari CSPRNG OS.
//! - Tag MAC: 16-byte Poly1305.
//! - Prinsip Fail-Closed: manipulasi sekecil apapun pada ciphertext atau tag
//!   menghasilkan `CryptoError::DecryptionFailed`.

use crate::error::CryptoError;
use crate::types::{MediaFileKey, RoomSessionKey};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use rand::rngs::OsRng;
use rand::RngCore;

/// Panjang nonce XChaCha20 (24 bytes).
pub const NONCE_LEN: usize = 24;
/// Panjang tag MAC Poly1305 (16 bytes).
pub const MAC_TAG_LEN: usize = 16;

/// Menghasilkan 24-byte nonce acak aman dari OS CSPRNG.
pub fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Menghasilkan kunci acak baru untuk file media (256-bit).
pub fn generate_media_key() -> MediaFileKey {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    MediaFileKey::from_bytes(key_bytes)
}

/// Mengenkrpsi pesan plaintext menggunakan kunci sesi room.
///
/// Mengembalikan pasangan: `(nonce, ciphertext_dengan_tag)`.
pub fn encrypt_message(
    key: &RoomSessionKey,
    plaintext: &[u8],
) -> Result<([u8; NONCE_LEN], Vec<u8>), CryptoError> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_bytes()));
    let nonce = generate_nonce();
    let x_nonce = XNonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(x_nonce, plaintext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    Ok((nonce, ciphertext))
}

/// Mendekripsi pesan ciphertext menggunakan kunci sesi room dan nonce.
///
/// Jika data telah di-tamper atau tag MAC salah, mengembalikan `CryptoError::DecryptionFailed`.
pub fn decrypt_message(
    key: &RoomSessionKey,
    nonce: &[u8; NONCE_LEN],
    ciphertext_with_tag: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_bytes()));
    let x_nonce = XNonce::from_slice(nonce);

    cipher
        .decrypt(x_nonce, ciphertext_with_tag)
        .map_err(|_| CryptoError::DecryptionFailed)
}

/// Mengenkrpsi data dan menggabungkannya dalam format sekuensial tunggal:
/// `[nonce (24 byte)] || [ciphertext] || [mac tag (16 byte)]`.
///
/// Format ini kompatibel langsung dengan fungsi `box.concatenation()` pada Flutter.
pub fn encrypt_concatenated(key_bytes: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let nonce = generate_nonce();
    let x_nonce = XNonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(x_nonce, plaintext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Mendekripsi data dari format sekuensial tunggal:
/// `[nonce (24 byte)] || [ciphertext] || [mac tag (16 byte)]`.
pub fn decrypt_concatenated(key_bytes: &[u8; 32], concatenated: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if concatenated.len() < NONCE_LEN + MAC_TAG_LEN {
        return Err(CryptoError::InvalidFormat(
            "Panjang paket lebih kecil dari nonce + MAC tag".to_string(),
        ));
    }

    let (nonce, ciphertext) = concatenated.split_at(NONCE_LEN);
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let x_nonce = XNonce::from_slice(nonce);

    cipher
        .decrypt(x_nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)
}

/// Mengenkripsi berkas media menggunakan `MediaFileKey`.
pub fn encrypt_media(key: &MediaFileKey, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    encrypt_concatenated(key.as_bytes(), data)
}

/// Mendekripsi berkas media menggunakan `MediaFileKey`.
pub fn decrypt_media(key: &MediaFileKey, encrypted_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    decrypt_concatenated(key.as_bytes(), encrypted_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = RoomSessionKey::from_bytes([7u8; 32]);
        let plaintext = b"Halo ini pesan rahasia MEKAAR!";

        let (nonce, ct) = encrypt_message(&key, plaintext).expect("encrypt");
        let decrypted = decrypt_message(&key, &nonce, &ct).expect("decrypt");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_tamper_fails_closed() {
        let key = RoomSessionKey::from_bytes([7u8; 32]);
        let plaintext = b"Pesan otentik";

        let (nonce, mut ct) = encrypt_message(&key, plaintext).expect("encrypt");
        // Tamper 1 bit di ciphertext
        if let Some(first) = ct.first_mut() {
            *first ^= 0x01;
        }

        let result = decrypt_message(&key, &nonce, &ct);
        assert_eq!(result, Err(CryptoError::DecryptionFailed));
    }

    #[test]
    fn test_concatenated_roundtrip() {
        let key_bytes = [9u8; 32];
        let plaintext = b"Data terdistribusi dengan format Flutter";

        let payload = encrypt_concatenated(&key_bytes, plaintext).expect("concat encrypt");
        assert!(payload.len() >= NONCE_LEN + plaintext.len() + MAC_TAG_LEN);

        let recovered = decrypt_concatenated(&key_bytes, &payload).expect("concat decrypt");
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn test_media_encryption_roundtrip() {
        let media_key = generate_media_key();
        let image_bytes = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46]; // JPEG header mock

        let encrypted = encrypt_media(&media_key, &image_bytes).expect("media encrypt");
        let decrypted = decrypt_media(&media_key, &encrypted).expect("media decrypt");

        assert_eq!(decrypted, image_bytes);
    }
}
