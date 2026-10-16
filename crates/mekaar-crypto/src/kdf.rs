//! # mekaar-crypto: Key Derivation Functions (KDF)
//!
//! - Pertukaran kunci Diffie-Hellman X25519.
//! - Derivasi kunci sesi per-room via HKDF-SHA256 (`info: b"mekaar-room:{room_id}"`).
//! - Derivasi kunci master PIN via Argon2id (m=12MB, t=2, p=1).

use crate::error::CryptoError;
use crate::types::{IdentityPrivateKey, IdentityPublicKey, RoomSessionKey, SharedSecret};
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use sha2::Sha256;
use x25519_dalek::{PublicKey, StaticSecret};

/// Menghitung Diffie-Hellman shared secret dari private key lokal dan public key lawan bicara.
///
/// Fail-closed: menolak peer all-zero dan hasil shared all-zero (titik orde-rendah / kunci rusak).
pub fn compute_shared_secret(
    my_priv: &IdentityPrivateKey,
    peer_pub: &IdentityPublicKey,
) -> Result<SharedSecret, CryptoError> {
    if peer_pub.as_bytes() == &[0u8; 32] {
        return Err(CryptoError::InvalidPeerKey);
    }
    let secret = StaticSecret::from(*my_priv.as_bytes());
    let peer = PublicKey::from(*peer_pub.as_bytes());
    let shared = secret.diffie_hellman(&peer);

    let bytes = shared.to_bytes();
    if bytes == [0u8; 32] {
        return Err(CryptoError::InvalidPeerKey);
    }

    Ok(SharedSecret::from_bytes(bytes))
}

/// Menurunkan kunci sesi room 32-byte menggunakan HKDF-SHA256.
///
/// Salt diikat ke kedua public key (domain separation) dan `info` memakai
/// room_id yang dikanonikalisasi (trim + lowercase). Mencegah kunci lemah
/// akibat salt kosong dan tabrakan `"Room-A"` vs `"room-a "`.
pub fn derive_room_key(
    my_priv: &IdentityPrivateKey,
    peer_pub: &IdentityPublicKey,
    room_id: &str,
) -> Result<RoomSessionKey, CryptoError> {
    let room_canon = room_id.trim().to_lowercase();
    if room_canon.is_empty() || room_canon.len() > 128 {
        return Err(CryptoError::InvalidFormat(
            "room_id harus 1..128 karakter".to_string(),
        ));
    }
    let shared = compute_shared_secret(my_priv, peer_pub)?;

    // Salt = SHA256("mekaar-aegis-v1" || min(pubA,pubB) || max(pubA,pubB))
    // Mengikat identitas kedua pihak agar pasangan berbeda tidak berbagi salt.
    use sha2::Digest;
    let static_secret = StaticSecret::from(*my_priv.as_bytes());
    let my_pub = PublicKey::from(&static_secret);
    let (first, second) = if my_pub.as_bytes() <= peer_pub.as_bytes() {
        (my_pub.as_bytes(), peer_pub.as_bytes())
    } else {
        (peer_pub.as_bytes(), my_pub.as_bytes())
    };
    let mut hasher = Sha256::new();
    hasher.update(b"mekaar-aegis-v1");
    hasher.update(first);
    hasher.update(second);
    let salt = hasher.finalize();

    let info = format!("mekaar-room-v1:{}", room_canon);

    let hk = Hkdf::<Sha256>::new(Some(&salt), shared.as_bytes());
    let mut okm = [0u8; 32];
    hk.expand(info.as_bytes(), &mut okm)
        .map_err(|_| CryptoError::KeyDerivationFailed)?;

    Ok(RoomSessionKey::from_bytes(okm))
}

/// Menurunkan kunci simetris 32-byte dari PIN 6-digit menggunakan Argon2id.
///
/// Parameter (hardened):
/// - Memory: 19,456 KiB (~19 MB, minimum OWASP)
/// - Iterasi (time cost): 2
/// - Paralelisme: 1
/// - Panjang Output: 32 bytes
/// - Salt: 16 bytes
///
/// Fail-closed: menolak PIN kosong / non-6-digit.
pub fn derive_pin_key(pin: &str, salt: &[u8; 16]) -> Result<[u8; 32], CryptoError> {
    if pin.len() < 6 || !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err(CryptoError::InvalidFormat(
            "PIN harus minimal 6 digit numerik".to_string(),
        ));
    }
    let params = Params::new(19456, 2, 1, Some(32))
        .map_err(|_| CryptoError::KeyDerivationFailed)?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut out = [0u8; 32];
    argon2
        .hash_password_into(pin.as_bytes(), salt, &mut out)
        .map_err(|_| CryptoError::KeyDerivationFailed)?;

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::generate_identity;

    #[test]
    fn test_ecdh_symmetry() {
        let (alice_priv, alice_pub) = generate_identity();
        let (bob_priv, bob_pub) = generate_identity();

        let alice_shared = compute_shared_secret(&alice_priv, &bob_pub).expect("alice dh");
        let bob_shared = compute_shared_secret(&bob_priv, &alice_pub).expect("bob dh");

        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
    }

    #[test]
    fn test_derive_room_key_symmetry() {
        let (alice_priv, alice_pub) = generate_identity();
        let (bob_priv, bob_pub) = generate_identity();
        let room_id = "test-room-12345";

        let alice_room_key = derive_room_key(&alice_priv, &bob_pub, room_id).expect("alice derive");
        let bob_room_key = derive_room_key(&bob_priv, &alice_pub, room_id).expect("bob derive");

        assert_eq!(alice_room_key.as_bytes(), bob_room_key.as_bytes());
    }

    #[test]
    fn test_derive_room_key_different_rooms() {
        let (alice_priv, _alice_pub) = generate_identity();
        let (_bob_priv, bob_pub) = generate_identity();

        let key_1 = derive_room_key(&alice_priv, &bob_pub, "room-A").unwrap();
        let key_2 = derive_room_key(&alice_priv, &bob_pub, "room-B").unwrap();

        assert_ne!(key_1.as_bytes(), key_2.as_bytes());
    }

    #[test]
    fn test_derive_pin_key_determinism() {
        let pin = "123456";
        let salt = [42u8; 16];

        let key1 = derive_pin_key(pin, &salt).expect("kdf 1");
        let key2 = derive_pin_key(pin, &salt).expect("kdf 2");

        assert_eq!(key1, key2);

        // Salt berbeda harus menghasilkan kunci berbeda
        let salt_diff = [43u8; 16];
        let key3 = derive_pin_key(pin, &salt_diff).expect("kdf 3");
        assert_ne!(key1, key3);
    }
}
