//! Integration Tests: Aegis Protocol E2EE Full Lifecycle

use mekaar_crypto::*;

#[test]
fn test_alice_bob_e2ee_chat_exchange() {
    // 1. Inisialisasi identitas Alice dan Bob
    let (alice_priv, alice_pub) = generate_identity();
    let (bob_priv, bob_pub) = generate_identity();

    // 2. Verifikasi Safety Number (Fingerprint)
    let alice_fp = compute_fingerprint(&alice_pub);
    let bob_fp = compute_fingerprint(&bob_pub);
    assert_ne!(alice_fp, bob_fp);
    assert_eq!(alice_fp.split(' ').count(), 16);

    // 3. Negosiasi kunci room terpisah (TOFU model via relay directory)
    let room_id = "550e8400-e29b-41d4-a716-446655440000";
    let alice_room_key = derive_room_key(&alice_priv, &bob_pub, room_id).expect("alice derive");
    let bob_room_key = derive_room_key(&bob_priv, &alice_pub, room_id).expect("bob derive");

    assert_eq!(alice_room_key.as_bytes(), bob_room_key.as_bytes());

    // 4. Alice mengirim pesan rahasia
    let secret_message = b"SOS LOKASI DARURAT: Lat -6.2088, Lng 106.8456, butuh bantuan!";
    let (nonce, ciphertext) = encrypt_message(&alice_room_key, secret_message).expect("alice encrypt");

    // 5. Bob menerima dan mendekripsi pesan
    let decrypted_by_bob = decrypt_message(&bob_room_key, &nonce, &ciphertext).expect("bob decrypt");
    assert_eq!(decrypted_by_bob, secret_message);

    // 6. Fail-closed test: eve mencoba memodifikasi ciphertext di perjalanan
    let mut tampered_ciphertext = ciphertext.clone();
    tampered_ciphertext[5] ^= 0xFF; // membalik 8 bit

    let tampered_result = decrypt_message(&bob_room_key, &nonce, &tampered_ciphertext);
    assert_eq!(tampered_result, Err(CryptoError::DecryptionFailed));

    // 7. Fail-closed test: eve mencoba mendekripsi menggunakan kunci room yang salah
    let eve_wrong_key = derive_room_key(&alice_priv, &bob_pub, "wrong-room-id").expect("eve key");
    let eve_decrypt_result = decrypt_message(&eve_wrong_key, &nonce, &ciphertext);
    assert_eq!(eve_decrypt_result, Err(CryptoError::DecryptionFailed));
}

#[test]
fn test_rfc7748_x25519_test_vectors() {
    // Vektor uji resmi RFC 7748 Bagian 6.1 (Curve25519)
    let alice_private_bytes: [u8; 32] = [
        0x77, 0x07, 0x6d, 0x0a, 0x73, 0x18, 0xa5, 0x7d, 0x3c, 0x16, 0xc1, 0x72, 0x51, 0xb2, 0x66,
        0x45, 0xdf, 0x4c, 0x2f, 0x87, 0xeb, 0xc0, 0x99, 0x2a, 0xb1, 0x77, 0xfb, 0xa5, 0x1d, 0xb9,
        0x2c, 0x2a,
    ];

    let expected_alice_public: [u8; 32] = [
        0x85, 0x20, 0xf0, 0x09, 0x89, 0x30, 0xa7, 0x54, 0x74, 0x8b, 0x7d, 0xdc, 0xb4, 0x3e, 0xf7,
        0x5a, 0x0d, 0xbf, 0x3a, 0x0d, 0x26, 0x38, 0x1a, 0xf4, 0xeb, 0xa4, 0xa9, 0x8e, 0xaa, 0x9b,
        0x4e, 0x6a,
    ];

    let bob_private_bytes: [u8; 32] = [
        0x5d, 0xab, 0x08, 0x7e, 0x62, 0x4a, 0x8a, 0x4b, 0x79, 0xe1, 0x7f, 0x8b, 0x83, 0x80, 0x0e,
        0xe6, 0x6f, 0x3b, 0xb1, 0x29, 0x26, 0x18, 0xb6, 0xfd, 0x1c, 0x2f, 0x8b, 0x27, 0xff, 0x88,
        0xe0, 0xeb,
    ];

    let expected_bob_public: [u8; 32] = [
        0xde, 0x9e, 0xdb, 0x7d, 0x7b, 0x7d, 0xc1, 0xb4, 0xd3, 0x5b, 0x61, 0xc2, 0xec, 0xe4, 0x35,
        0x37, 0x3f, 0x83, 0x43, 0xc8, 0x5b, 0x78, 0x67, 0x4d, 0xad, 0xfc, 0x7e, 0x14, 0x6f, 0x88,
        0x2b, 0x4f,
    ];

    let expected_shared_secret: [u8; 32] = [
        0x4a, 0x5d, 0x9d, 0x5b, 0xa4, 0xce, 0x2d, 0xe1, 0x72, 0x8e, 0x3b, 0xf4, 0x80, 0x35, 0x0f,
        0x25, 0xe0, 0x7e, 0x21, 0xc9, 0x47, 0xd1, 0x9e, 0x33, 0x76, 0xf0, 0x9b, 0x3c, 0x1e, 0x16,
        0x17, 0x42,
    ];

    let (alice_priv, alice_pub) = identity_from_seed(alice_private_bytes);
    let (bob_priv, bob_pub) = identity_from_seed(bob_private_bytes);

    assert_eq!(alice_pub.as_bytes(), &expected_alice_public);
    assert_eq!(bob_pub.as_bytes(), &expected_bob_public);

    let alice_shared = compute_shared_secret(&alice_priv, &bob_pub).expect("alice dh");
    let bob_shared = compute_shared_secret(&bob_priv, &alice_pub).expect("bob dh");

    assert_eq!(alice_shared.as_bytes(), &expected_shared_secret);
    assert_eq!(bob_shared.as_bytes(), &expected_shared_secret);
}

#[test]
fn test_argon2id_pin_backup_cycle() {
    let (priv_key, pub_key) = generate_identity();
    let pin = "992817";

    // Backup
    let backup = wrap_private_key(&priv_key, pin).expect("backup success");

    // Restore dengan PIN yang benar
    let restored = unwrap_private_key(&backup, pin).expect("restore success");
    assert_eq!(restored.as_bytes(), priv_key.as_bytes());

    // Restore dengan PIN salah
    let fail = unwrap_private_key(&backup, "000000");
    assert!(fail.is_err());

    // Cek bahwa kunci hasil restore menghasilkan public key identik
    let secret = x25519_dalek::StaticSecret::from(*restored.as_bytes());
    let derived_pub = x25519_dalek::PublicKey::from(&secret);
    assert_eq!(derived_pub.as_bytes(), pub_key.as_bytes());
}
