//! Integration Tests: mekaar-core E2EE Chat, Guardian, SOS Engine, and Vault

use mekaar_core::*;
use mekaar_crypto::generate_identity;
use mekaar_proto::MessageType;
use mekaar_storage::{Contact, EncryptedDatabase};
use std::time::Duration;
use uuid::Uuid;

#[tokio::test]
async fn test_chat_engine_alice_bob_e2ee_flow() {
    let alice_id = Uuid::new_v4();
    let bob_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();

    let (alice_priv, alice_pub) = generate_identity();
    let (bob_priv, bob_pub) = generate_identity();

    let alice_db = EncryptedDatabase::open_in_memory("alice_key_123").unwrap();
    let bob_db = EncryptedDatabase::open_in_memory("bob_key_456").unwrap();

    let alice_bus = EventBus::default();
    let bob_bus = EventBus::default();

    // 1. Daftarkan kontak masing-masing
    alice_db
        .upsert_contact(&Contact {
            id: bob_id,
            username: "bob".to_string(),
            display_name: "Bob".to_string(),
            public_key: *bob_pub.as_bytes(),
            safety_number: "BOB-SAFE-123".to_string(),
            status: "active".to_string(),
            updated_at: 1000,
        })
        .unwrap();

    bob_db
        .upsert_contact(&Contact {
            id: alice_id,
            username: "alice".to_string(),
            display_name: "Alice".to_string(),
            public_key: *alice_pub.as_bytes(),
            safety_number: "ALICE-SAFE-456".to_string(),
            status: "active".to_string(),
            updated_at: 1000,
        })
        .unwrap();

    let alice_engine = ChatEngine::new(alice_id, alice_priv, alice_pub, alice_db.clone(), alice_bus);
    let bob_engine = ChatEngine::new(bob_id, bob_priv, bob_pub, bob_db.clone(), bob_bus.clone());

    let mut bob_events = bob_bus.subscribe();

    // 2. Alice mengirim pesan ke Bob
    let message_text = "Halo Bob, ini pesan terenkripsi E2EE dari Mekaar Core!";
    let (local_msg, envelope) = alice_engine
        .send_message(room_id, bob_id, message_text, MessageType::Text, false, None)
        .await
        .expect("alice send");

    assert_eq!(local_msg.content, message_text);
    assert_eq!(envelope.recipient_id, bob_id);

    // 3. Bob menerima envelope dari wire
    let received_by_bob = bob_engine
        .receive_envelope(envelope)
        .await
        .expect("bob receive");

    assert_eq!(received_by_bob.content, message_text);

    // 4. Verifikasi event bus Bob menerima notifikasi
    let event = bob_events.recv().await.expect("recv event");
    if let CoreEvent::MessageReceived(msg) = event {
        assert_eq!(msg.content, message_text);
        assert_eq!(msg.sender_id, alice_id);
    } else {
        panic!("Diharapkan event MessageReceived");
    }

    // 5. Update status pengiriman
    alice_engine.mark_delivered(&local_msg.id).await.unwrap();
    alice_engine.mark_read(&local_msg.id).await.unwrap();

    let updated_local = alice_db.get_message(&local_msg.id).unwrap().unwrap();
    assert_eq!(updated_local.status, mekaar_storage::DeliveryStatus::Read);
}

#[tokio::test]
async fn test_disappearing_messages_worker() {
    let db = EncryptedDatabase::open_in_memory("test_db_pass").unwrap();
    let bus = EventBus::default();
    let mut rx = bus.subscribe();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Masukkan pesan kedaluwarsa (auto_delete_at = now - 50ms)
    let expired_msg = mekaar_storage::LocalMessage {
        id: Uuid::new_v4(),
        room_id: Uuid::new_v4(),
        sender_id: Uuid::new_v4(),
        recipient_id: Uuid::new_v4(),
        content: "Pesan segera terhapus".to_string(),
        media_url: None,
        msg_type: MessageType::Text,
        is_view_once: false,
        is_opened: false,
        opened_at: None,
        reply_to_id: None,
        is_deleted: false,
        is_silent_deleted: false,
        auto_delete_at: Some(now - 50),
        edited_at: None,
        reactions: std::collections::HashMap::new(),
        is_encrypted: true,
        created_at: now - 100,
        updated_at: now - 100,
        status: mekaar_storage::DeliveryStatus::Read,
    };
    db.insert_message(&expired_msg).unwrap();

    // Jalankan worker dengan interval 30ms
    let handle = spawn_disappearing_worker(db.clone(), bus.clone(), Duration::from_millis(30));

    // Tunggu event pembersihan
    tokio::select! {
        Ok(CoreEvent::DisappearingMessagesPurged { count }) = rx.recv() => {
            assert!(count >= 1);
        }
        _ = tokio::time::sleep(Duration::from_millis(500)) => {
            panic!("Timeout menunggu disappearing worker membersihkan pesan");
        }
    }

    handle.stop();
}

#[tokio::test]
async fn test_guardian_state_transitions_and_role_swap() {
    let bus = EventBus::default();
    let manager = GuardianManager::new(bus.clone());

    // 1. Status awal adalah Dormant (Zero-Surveillance)
    assert_eq!(manager.current_state().await, GuardianState::Dormant);
    assert_eq!(manager.current_role().await, GuardianRole::Owner);

    // 2. Aktifkan rute perjalanan sementara (Hangout)
    let hangout = manager.start_hangout(3600000, "trip-token-abc".into()).await;
    match hangout {
        GuardianState::HangoutSharing { token, .. } => assert_eq!(token, "trip-token-abc"),
        _ => panic!("Harus HangoutSharing"),
    }

    // 3. Selesaikan hangout
    assert_eq!(manager.end_hangout().await, GuardianState::Dormant);

    // 4. Pemicuan SOS darurat
    assert_eq!(manager.trigger_emergency().await, GuardianState::EmergencyBroadcast);
    assert_eq!(manager.resolve_emergency().await, GuardianState::Dormant);

    // 5. Dynamic Role Swapping
    let swapped = manager.swap_role().await;
    assert_eq!(swapped, GuardianRole::Guardian);
    assert_eq!(manager.current_role().await, GuardianRole::Guardian);
}

#[tokio::test]
async fn test_sos_panic_engine_lifecycle_and_watchdog() {
    let bus = EventBus::default();
    let engine = SosPanicEngine::new(bus.clone());

    assert_eq!(engine.current_state().await, SosState::Idle);

    // 1. Arm progress
    let armed = engine.arm_progress(0.5).await;
    match armed {
        SosState::Armed { progress } => assert_eq!(progress, 0.5),
        _ => panic!("Harus Armed"),
    }

    // 2. Full trigger (hold 2s)
    let broadcasting = engine.trigger_panic(true, true, true).await;
    match broadcasting {
        SosState::Broadcasting { gps_enabled, mic_enabled, video_enabled, .. } => {
            assert!(gps_enabled && mic_enabled && video_enabled);
        }
        _ => panic!("Harus Broadcasting"),
    }

    // 3. Guardian acknowledge
    let guardian_id = Uuid::new_v4();
    let acked = engine.acknowledge(guardian_id).await.unwrap();
    assert_eq!(acked, SosState::Acknowledged { guardian_id });

    // 4. Resolve manual
    let resolved = engine.resolve(SosEndReason::Manual).await;
    match resolved {
        SosState::Resolved { reason, .. } => assert_eq!(reason, SosEndReason::Manual),
        _ => panic!("Harus Resolved"),
    }
}

#[tokio::test]
async fn test_vault_manager_normal_duress_and_lockout() {
    let salt = [0x33; 16];
    let vault = VaultManager::new(salt);

    let master_pin = "123456";
    let duress_pin = "999999";

    vault.set_master_pin(master_pin).await.unwrap();
    vault.set_duress_pin(duress_pin).await.unwrap();

    // 1. Verifikasi PIN Master benar
    let res_master = vault.verify_pin(master_pin).await.unwrap();
    assert_eq!(res_master, AuthOutcome::Unlocked);
    assert!(vault.is_vault_unlocked().await);

    vault.lock_vault().await;
    assert!(!vault.is_vault_unlocked().await);

    // 2. Verifikasi Duress PIN (PIN Paksaan)
    let res_duress = vault.verify_pin(duress_pin).await.unwrap();
    assert_eq!(res_duress, AuthOutcome::Duress);

    // 3. Uji proteksi brute force (5x gagal)
    vault.lock_vault().await;
    for _ in 0..4 {
        let err = vault.verify_pin("000000").await;
        assert!(err.is_err());
    }

    // Percobaan ke-5 harus menghasilkan error Lockout
    let locked_err = vault.verify_pin("000000").await;
    match locked_err {
        Err(CoreError::PinLocked(attempts, ..)) => {
            assert_eq!(attempts, 5);
        }
        _ => panic!("Harus PinLocked pada percobaan ke-5"),
    }
}
