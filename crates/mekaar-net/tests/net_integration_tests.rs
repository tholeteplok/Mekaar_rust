//! # mekaar-net: Integration Tests
//!
//! Pengujian integrasi transport layer:
//! - Koneksi WebSocket duplex client ke server Axum
//! - Handshake registrasi otomatis
//! - Transmisi pesan biner Postcard
//! - Pengiriman sinyal WebRTC Offer/Answer via relay
//! - Pertukaran DeliveryReceipt

use mekaar_crypto::generate_identity;
use mekaar_net::{make_offer_signal, MekaarClient, ReconnectPolicy};
use mekaar_proto::{ClientCommand, MessageEnvelope, MessageType, ServerCommand};
use mekaar_server::create_router;
use mekaar_server::db::Database as ServerDb;
use mekaar_server::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use uuid::Uuid;

async fn spawn_test_server() -> (String, Arc<Hub>) {
    let db = Arc::new(ServerDb::open_in_memory().expect("Gagal buka in-memory DB"));
    let hub = Arc::new(Hub::new(db));
    let app = create_router(hub.clone());

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Gagal bind listener port acak");
    let addr = listener.local_addr().expect("Gagal ambil local addr");
    let ws_url = format!("ws://{}/ws", addr);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (ws_url, hub)
}

#[tokio::test]
async fn test_client_connect_and_exchange_messages() {
    let (ws_url, _hub) = spawn_test_server().await;

    // 1. Inisialisasi Alice dan Bob
    let (_alice_priv, alice_pub) = generate_identity();
    let (_bob_priv, bob_pub) = generate_identity();
    let alice_id = Uuid::new_v4();
    let bob_id = Uuid::new_v4();

    // 2. Hubungkan Alice ke Server via MekaarClient
    let mut alice_client = MekaarClient::connect(
        &ws_url,
        alice_id,
        *alice_pub.as_bytes(),
        ReconnectPolicy::default(),
    )
    .await
    .expect("Alice gagal terkoneksi");

    // Verifikasi Alice menerima SubscribedAck
    let ack = tokio::time::timeout(Duration::from_secs(2), alice_client.recv())
        .await
        .expect("Timeout menunggu SubscribedAck Alice")
        .expect("Channel tertutup");

    assert_eq!(ack, ServerCommand::SubscribedAck { user_id: alice_id });
    assert!(alice_client.is_connected());

    // 3. Hubungkan Bob ke Server via MekaarClient
    let mut bob_client = MekaarClient::connect(
        &ws_url,
        bob_id,
        *bob_pub.as_bytes(),
        ReconnectPolicy::default(),
    )
    .await
    .expect("Bob gagal terkoneksi");

    let ack_bob = tokio::time::timeout(Duration::from_secs(2), bob_client.recv())
        .await
        .expect("Timeout menunggu SubscribedAck Bob")
        .expect("Channel tertutup");

    assert_eq!(ack_bob, ServerCommand::SubscribedAck { user_id: bob_id });

    // 4. Alice mengirim MessageEnvelope ke Bob
    let envelope = MessageEnvelope::new(
        alice_id,
        bob_id,
        Uuid::new_v4(),
        [7u8; 24],
        b"ciphertext_e2ee_terenkripsi_aegis".to_vec(),
        1720000000,
        MessageType::Text,
    );

    alice_client
        .send_command(ClientCommand::SendEnvelope(envelope.clone()))
        .expect("Alice gagal kirim envelope");

    // 5. Bob menerima IncomingEnvelope
    let received = tokio::time::timeout(Duration::from_secs(2), bob_client.recv())
        .await
        .expect("Timeout Bob menerima envelope")
        .expect("Channel tertutup");

    match received {
        ServerCommand::IncomingEnvelope(env) => {
            assert_eq!(env.message_id, envelope.message_id);
            assert_eq!(env.sender_id, alice_id);
            assert_eq!(env.ciphertext, envelope.ciphertext);
        }
        other => panic!("Harusnya menerima IncomingEnvelope, tapi dapat: {:?}", other),
    }

    // 6. Bob mengirim DeliveryAck kembali ke Alice
    bob_client
        .send_command(ClientCommand::DeliveryAck {
            message_id: envelope.message_id,
            sender_id: alice_id,
        })
        .expect("Bob gagal kirim DeliveryAck");

    // Alice menerima DeliveryReceipt
    let receipt = tokio::time::timeout(Duration::from_secs(2), alice_client.recv())
        .await
        .expect("Timeout Alice menerima DeliveryReceipt")
        .expect("Channel tertutup");

    match receipt {
        ServerCommand::DeliveryReceipt { message_id, .. } => {
            assert_eq!(message_id, envelope.message_id);
        }
        other => panic!("Harusnya menerima DeliveryReceipt, tapi dapat: {:?}", other),
    }

    // 7. Uji WebRTC Signaling (Offer dari Alice ke Bob)
    let offer_cmd = make_offer_signal(bob_id, "v=0\r\no=alice 123456 IN IP4 127.0.0.1");
    alice_client
        .send_command(offer_cmd)
        .expect("Alice gagal kirim sinyal WebRTC");

    let signal_relayed = tokio::time::timeout(Duration::from_secs(2), bob_client.recv())
        .await
        .expect("Timeout Bob menerima WebRtcRelay")
        .expect("Channel tertutup");

    match signal_relayed {
        ServerCommand::WebRtcRelay {
            from_user_id,
            signal_type,
            payload,
        } => {
            assert_eq!(from_user_id, alice_id);
            assert_eq!(signal_type, "offer");
            assert!(payload.contains("v=0"));
        }
        other => panic!("Harusnya menerima WebRtcRelay, tapi dapat: {:?}", other),
    }

    alice_client.close();
    bob_client.close();
}
