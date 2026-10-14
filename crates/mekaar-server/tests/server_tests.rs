//! Integration Tests: mekaar-server Realtime Relay, Offline Store-and-Forward, and SOS Dispatch

use futures_util::{SinkExt, StreamExt};
use mekaar_proto::{
    decode_postcard, encode_postcard, ClientCommand, MessageEnvelope, MessageType, ServerCommand,
};
use mekaar_server::create_router;
use mekaar_server::db::Database;
use mekaar_server::hub::Hub;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

/// Helper untuk meluncurkan server pengujian pada ephemeral port lokal.
async fn spawn_test_server() -> (SocketAddr, Arc<Hub>) {
    let db = Database::open_in_memory().expect("open test db");
    let hub = Arc::new(Hub::new(Arc::new(db)));
    let app = create_router(hub.clone());

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind ephemeral");
    let addr = listener.local_addr().expect("local addr");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (addr, hub)
}

#[tokio::test]
async fn test_rest_endpoints() {
    let (addr, _hub) = spawn_test_server().await;
    let client = reqwest::Client::new();
    let base_url = format!("http://{}", addr);

    // 1. GET /health
    let resp = client
        .get(format!("{}/health", base_url))
        .send()
        .await
        .expect("health req");
    assert_eq!(resp.status(), 200);
    let health_body: serde_json::Value = resp.json().await.expect("health json");
    assert_eq!(health_body["status"], "ok");

    // 2. POST /keys & GET /keys/{user_id}
    let user_id = Uuid::new_v4();
    let public_key = [0x7A; 32];
    let key_resp = client
        .post(format!("{}/keys", base_url))
        .json(&serde_json::json!({
            "user_id": user_id,
            "public_key": public_key
        }))
        .send()
        .await
        .expect("post keys");
    assert_eq!(key_resp.status(), 200);

    let get_key_resp = client
        .get(format!("{}/keys/{}", base_url, user_id))
        .send()
        .await
        .expect("get keys");
    assert_eq!(get_key_resp.status(), 200);
    let key_data: serde_json::Value = get_key_resp.json().await.unwrap();
    assert_eq!(key_data["user_id"], user_id.to_string());

    // 3. POST /backups & GET /backups/{user_id}
    let backup_json = r#"{"v":2,"kdf":"argon2id","salt":"...","ct":"..."}"#;
    let bkp_resp = client
        .post(format!("{}/backups", base_url))
        .json(&serde_json::json!({
            "user_id": user_id,
            "backup_json": backup_json
        }))
        .send()
        .await
        .expect("post backup");
    assert_eq!(bkp_resp.status(), 200);

    let get_bkp_resp = client
        .get(format!("{}/backups/{}", base_url, user_id))
        .send()
        .await
        .expect("get backup");
    assert_eq!(get_bkp_resp.status(), 200);
    let bkp_data: serde_json::Value = get_bkp_resp.json().await.unwrap();
    assert_eq!(bkp_data["backup_json"], backup_json);

    // 4. POST /guardians & GET /guardians/{victim_id}
    let guardian_id = Uuid::new_v4();
    let add_g_resp = client
        .post(format!("{}/guardians", base_url))
        .json(&serde_json::json!({
            "victim_id": user_id,
            "guardian_id": guardian_id
        }))
        .send()
        .await
        .expect("add guardian");
    assert_eq!(add_g_resp.status(), 200);

    let get_g_resp = client
        .get(format!("{}/guardians/{}", base_url, user_id))
        .send()
        .await
        .expect("get guardians");
    assert_eq!(get_g_resp.status(), 200);
    let g_data: serde_json::Value = get_g_resp.json().await.unwrap();
    let guardians = g_data["guardians"].as_array().unwrap();
    assert_eq!(guardians.len(), 1);
    assert_eq!(guardians[0], guardian_id.to_string());
}

#[tokio::test]
async fn test_websocket_realtime_and_offline_relay() {
    let (addr, _hub) = spawn_test_server().await;
    let ws_url = format!("ws://{}/ws", addr);

    let alice_id = Uuid::new_v4();
    let bob_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();

    // ── Bagian 1: Realtime Relay (Alice & Bob Keduanya Online) ──────────

    // Hubungkan Alice
    let (mut alice_ws, _) = connect_async(&ws_url).await.expect("alice connect");
    let auth_alice = ClientCommand::AuthSubscribe {
        user_id: alice_id,
        public_key: [0x11; 32],
    };
    alice_ws
        .send(Message::Binary(encode_postcard(&auth_alice).unwrap().into()))
        .await
        .unwrap();

    // Terima SubscribedAck
    let msg = alice_ws.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = msg {
        let ack: ServerCommand = decode_postcard(&bin).unwrap();
        assert_eq!(ack, ServerCommand::SubscribedAck { user_id: alice_id });
    } else {
        panic!("Diharapkan binary SubscribedAck");
    }

    // Hubungkan Bob
    let (mut bob_ws, _) = connect_async(&ws_url).await.expect("bob connect");
    let auth_bob = ClientCommand::AuthSubscribe {
        user_id: bob_id,
        public_key: [0x22; 32],
    };
    bob_ws
        .send(Message::Binary(encode_postcard(&auth_bob).unwrap().into()))
        .await
        .unwrap();

    // Terima SubscribedAck untuk Bob
    let msg_bob_ack = bob_ws.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = msg_bob_ack {
        let ack: ServerCommand = decode_postcard(&bin).unwrap();
        assert_eq!(ack, ServerCommand::SubscribedAck { user_id: bob_id });
    }

    // Alice mengirim pesan terenkripsi ke Bob (keduanya online)
    let envelope_1 = MessageEnvelope::new(
        alice_id,
        bob_id,
        room_id,
        [0xAA; 24],
        b"Ciphertext Pesan 1 (Online)".to_vec(),
        1700000000000,
        MessageType::Text,
    );

    alice_ws
        .send(Message::Binary(
            encode_postcard(&ClientCommand::SendEnvelope(envelope_1.clone())).unwrap().into(),
        ))
        .await
        .unwrap();

    // Bob harus menerima pesan seketika via WebSocket
    let bob_incoming = bob_ws.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = bob_incoming {
        let cmd: ServerCommand = decode_postcard(&bin).unwrap();
        assert_eq!(cmd, ServerCommand::IncomingEnvelope(envelope_1.clone()));
    } else {
        panic!("Bob harus menerima IncomingEnvelope");
    }

    // Alice menerima tanda terima DeliveryReceipt dari server
    let alice_receipt = alice_ws.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = alice_receipt {
        let cmd: ServerCommand = decode_postcard(&bin).unwrap();
        match cmd {
            ServerCommand::DeliveryReceipt { message_id, .. } => {
                assert_eq!(message_id, envelope_1.message_id);
            }
            _ => panic!("Alice harus menerima DeliveryReceipt"),
        }
    }

    // ── Bagian 2: Offline Store-and-Forward (Bob Disconnect) ───────────

    // Bob disconnect
    drop(bob_ws);
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Alice mengirim pesan kedua saat Bob offline
    let envelope_2 = MessageEnvelope::new(
        alice_id,
        bob_id,
        room_id,
        [0xBB; 24],
        b"Ciphertext Pesan 2 (Offline Store-and-Forward)".to_vec(),
        1700000001000,
        MessageType::Text,
    );

    alice_ws
        .send(Message::Binary(
            encode_postcard(&ClientCommand::SendEnvelope(envelope_2.clone())).unwrap().into(),
        ))
        .await
        .unwrap();

    // Alice tetap menerima tanda terima bahwa server telah menerima pesan
    let receipt_2 = alice_ws.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = receipt_2 {
        let cmd: ServerCommand = decode_postcard(&bin).unwrap();
        match cmd {
            ServerCommand::DeliveryReceipt { message_id, .. } => {
                assert_eq!(message_id, envelope_2.message_id);
            }
            _ => panic!("Diharapkan DeliveryReceipt untuk pesan 2"),
        }
    }

    // Bob reconnect kembali online
    let (mut bob_reconnect, _) = connect_async(&ws_url).await.expect("bob reconnect");
    bob_reconnect
        .send(Message::Binary(encode_postcard(&auth_bob).unwrap().into()))
        .await
        .unwrap();

    // 1. Terima SubscribedAck
    let msg_reconnect_ack = bob_reconnect.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = msg_reconnect_ack {
        let ack: ServerCommand = decode_postcard(&bin).unwrap();
        assert_eq!(ack, ServerCommand::SubscribedAck { user_id: bob_id });
    }

    // 2. Server harus otomatis mengalirkan pesan kedua yang tertunda saat Bob offline!
    let flushed_msg = bob_reconnect.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = flushed_msg {
        let cmd: ServerCommand = decode_postcard(&bin).unwrap();
        assert_eq!(cmd, ServerCommand::IncomingEnvelope(envelope_2.clone()));
    } else {
        panic!("Bob harus menerima pesan offline yang di-flush!");
    }
}

#[tokio::test]
async fn test_sos_panic_broadcast_to_guardian() {
    let (addr, hub) = spawn_test_server().await;
    let ws_url = format!("ws://{}/ws", addr);

    let victim_id = Uuid::new_v4();
    let guardian_id = Uuid::new_v4();

    // Tautkan Guardian di database
    hub.db().add_guardian(&victim_id, &guardian_id).unwrap();

    // Guardian terkoneksi ke WebSocket
    let (mut guardian_ws, _) = connect_async(&ws_url).await.expect("guardian connect");
    let auth_guardian = ClientCommand::AuthSubscribe {
        user_id: guardian_id,
        public_key: [0x99; 32],
    };
    guardian_ws
        .send(Message::Binary(encode_postcard(&auth_guardian).unwrap().into()))
        .await
        .unwrap();

    let _ = guardian_ws.next().await.unwrap().unwrap(); // skip SubscribedAck

    // Korban terkoneksi dan memicu sinyal SOS
    let (mut victim_ws, _) = connect_async(&ws_url).await.expect("victim connect");
    let auth_victim = ClientCommand::AuthSubscribe {
        user_id: victim_id,
        public_key: [0x88; 32],
    };
    victim_ws
        .send(Message::Binary(encode_postcard(&auth_victim).unwrap().into()))
        .await
        .unwrap();
    let _ = victim_ws.next().await.unwrap().unwrap(); // skip SubscribedAck

    // Pemicu SOS
    let sos_trigger = ClientCommand::SosTrigger {
        user_id: victim_id,
        latitude: -6.175392,
        longitude: 106.827153,
        accuracy: 2.1,
        timestamp_ms: 1700000009999,
    };
    victim_ws
        .send(Message::Binary(encode_postcard(&sos_trigger).unwrap().into()))
        .await
        .unwrap();

    // Guardian harus seketika menerima siaran krisis GuardianSosBroadcast!
    let bcast_msg = guardian_ws.next().await.unwrap().unwrap();
    if let Message::Binary(bin) = bcast_msg {
        let cmd: ServerCommand = decode_postcard(&bin).unwrap();
        assert_eq!(
            cmd,
            ServerCommand::GuardianSosBroadcast {
                victim_id,
                latitude: -6.175392,
                longitude: 106.827153,
                accuracy: 2.1,
                timestamp_ms: 1700000009999,
            }
        );
    } else {
        panic!("Guardian harus menerima sinyal siaran darurat SOS!");
    }
}
