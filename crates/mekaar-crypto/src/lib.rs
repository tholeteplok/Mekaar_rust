//! # mekaar-crypto — Aegis Protocol E2EE Core
//!
//! Pustaka kriptografi inti MEKAAR yang mengimplementasikan protokol **Aegis**:
//! - **Asymmetric Key Exchange:** X25519 Curve25519 (`x25519-dalek`)
//! - **Symmetric AEAD Encryption:** XChaCha20-Poly1305 (`chacha20poly1305`)
//! - **Key Derivation (KDF):** HKDF-SHA256 (`hkdf` + `sha2`)
//! - **Master PIN KDF:** Argon2id (`argon2`)
//! - **Memory Safety:** Deterministic zeroization (`zeroize::ZeroizeOnDrop`)
//!
//! ## Prinsip Fail-Closed
//! Seluruh operasi kriptografi mengembalikan `Result<T, CryptoError>`.
//! Dilarang keras mengirim data dalam plaintext jika negosiasi kunci atau verifikasi MAC gagal.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod backup;
pub mod cipher;
pub mod error;
pub mod identity;
pub mod kdf;
pub mod types;

// Re-exports untuk kemudahan penggunaan
pub use backup::{unwrap_private_key, wrap_private_key, KeyBackup};
pub use cipher::{
    decrypt_concatenated, decrypt_media, decrypt_message, decrypt_message_with_aad,
    encrypt_concatenated, encrypt_media, encrypt_message, encrypt_message_with_aad,
    generate_media_key, generate_nonce, MAC_TAG_LEN, MAX_MEDIA_BYTES, MAX_MESSAGE_BYTES, NONCE_LEN,
};
pub use error::CryptoError;
pub use identity::{compute_fingerprint, generate_identity, identity_from_seed};
pub use kdf::{compute_shared_secret, derive_pin_key, derive_room_key};
pub use types::{
    IdentityPrivateKey, IdentityPublicKey, MediaFileKey, RoomSessionKey, SharedSecret,
};
