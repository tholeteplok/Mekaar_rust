//! # mekaar-crypto: PIN-based Key Wrapping & Backup
//!
//! Mengenkripsi kunci privat identitas pengguna dengan master PIN 6 digit via Argon2id
//! sehingga dapat disimpan sebagai backup di server tanpa membocorkan kunci (Zero-Knowledge).

use crate::cipher::{decrypt_concatenated, encrypt_concatenated};
use crate::error::CryptoError;
use crate::kdf::derive_pin_key;
use crate::types::IdentityPrivateKey;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Struktur data backup kunci privat yang terenkripsi aman.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KeyBackup {
    /// Versi format backup (v2 = Argon2id).
    pub version: u8,
    /// Identifier algoritma KDF yang dipakai.
    pub kdf: String,
    /// Salt 16-byte acak untuk Argon2id.
    pub salt: [u8; 16],
    /// Kunci privat yang terenkripsi XChaCha20-Poly1305:
    /// `nonce (24 bytes) || ciphertext (32 bytes) || mac tag (16 bytes)` = 72 bytes.
    pub encrypted_private_key: Vec<u8>,
}

/// Membungkus (wrap) kunci privat identitas menggunakan master PIN.
pub fn wrap_private_key(priv_key: &IdentityPrivateKey, pin: &str) -> Result<KeyBackup, CryptoError> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let mut wrap_key = derive_pin_key(pin, &salt)?;
    let encrypted = encrypt_concatenated(&wrap_key, priv_key.as_bytes())?;
    wrap_key.zeroize();

    Ok(KeyBackup {
        version: 2,
        kdf: "argon2id".to_string(),
        salt,
        encrypted_private_key: encrypted,
    })
}

/// Membuka (unwrap) kunci privat identitas dari data backup menggunakan master PIN.
///
/// Jika PIN salah atau data rusak, mengembalikan `CryptoError::InvalidPinOrCorruptedBackup`.
pub fn unwrap_private_key(backup: &KeyBackup, pin: &str) -> Result<IdentityPrivateKey, CryptoError> {
    if backup.kdf != "argon2id" {
        return Err(CryptoError::InvalidFormat(format!(
            "KDF '{}' tidak didukung (hanya 'argon2id')",
            backup.kdf
        )));
    }

    let mut wrap_key = derive_pin_key(pin, &backup.salt)?;
    let decrypted = decrypt_concatenated(&wrap_key, &backup.encrypted_private_key)
        .map_err(|_| CryptoError::InvalidPinOrCorruptedBackup)?;
    wrap_key.zeroize();

    if decrypted.len() != 32 {
        return Err(CryptoError::InvalidFormat(
            "Panjang kunci privat hasil unwrap tidak 32-byte".to_string(),
        ));
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&decrypted);

    Ok(IdentityPrivateKey::from_bytes(key_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::generate_identity;

    #[test]
    fn test_wrap_and_unwrap_success() {
        let (priv_key, pub_key) = generate_identity();
        let pin = "849201";

        let backup = wrap_private_key(&priv_key, pin).expect("wrap");
        assert_eq!(backup.version, 2);
        assert_eq!(backup.kdf, "argon2id");
        assert_eq!(backup.encrypted_private_key.len(), 24 + 32 + 16);

        let restored_priv = unwrap_private_key(&backup, pin).expect("unwrap");
        assert_eq!(restored_priv.as_bytes(), priv_key.as_bytes());

        // Verifikasi bahwa public key yang dihasilkan dari private key restore tetap cocok
        let secret = x25519_dalek::StaticSecret::from(*restored_priv.as_bytes());
        let expected_pub = x25519_dalek::PublicKey::from(&secret);
        assert_eq!(expected_pub.as_bytes(), pub_key.as_bytes());
    }

    #[test]
    fn test_unwrap_wrong_pin_fails() {
        let (priv_key, _) = generate_identity();
        let pin = "123456";
        let wrong_pin = "654321";

        let backup = wrap_private_key(&priv_key, pin).expect("wrap");
        let result = unwrap_private_key(&backup, wrong_pin);

        assert_eq!(result.err(), Some(CryptoError::InvalidPinOrCorruptedBackup));
    }
}
