//! # mekaar-cli: Automated E2E Pair Simulation Runner
//!
//! Menjalankan pengujian diagnostik otomatis end-to-end tanpa antarmuka grafis:
//! 1. Inisialisasi in-process relay server (jika tidak ada custom server)
//! 2. Pembuatan identitas kriptografi Alice & Bob (`mekaar-crypto`)
//! 3. Database lokal terenkripsi SQLCipher mandiri (`mekaar-storage`)
//! 4. Koneksi WebSocket client duplex (`mekaar-net`)
//! 5. Pertukaran pesan E2EE Aegis Protocol
//! 6. Konfirmasi tanda terima (DeliveryAck & DeliveryReceipt)
//! 7. Pembersihan pesan sekali lihat (View-Once RAM/disk scrubbing)
//! 8. Transmisi sinyal panik darurat SOS ke perangkat Guardian
//! 9. Pertukaran sinyal WebRTC SDP Offer/Answer relay

use anyhow::{bail, Context, Result};
use mekaar_core::events::EventBus;
use mekaar_core::ChatEngine;
use mekaar_crypto::{compute_fingerprint, generate_identity};
use mekaar_net::{make_answer_signal, make_offer_signal, MekaarClient, ReconnectPolicy};
use mekaar_proto::{ClientCommand, MessageType, ServerCommand};
use mekaar_server::create_router;
use mekaar_server::db::Database as ServerDb;
use mekaar_server::hub::Hub;
use mekaar_storage::{Contact, EncryptedDatabase};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use uuid::Uuid;

/// Menjalankan simulasi lengkap pengujian Alice <-> Bob.
pub async fn run_simulation(custom_server: Option<String>) -> Result<()> {
    println!("\n========================================================");
    println!("🦀 MEKAAR Native Engine — Automated E2E Pair Simulation");
    println!("========================================================\n");

    let start_total = Instant::now();

    // ── Langkah 1: Persiapan Server Relay ───────────────────────────────
    let (server_url, server_hub) = if let Some(url) = custom_server {
        println!("🌐 Menggunakan Relay Server Eksternal: {}", url);
        (url, None)
    } else {
        let db = Arc::new(ServerDb::open_in_memory().context("Gagal buka in-memory database server")?);
        let hub = Arc::new(Hub::new(db));
        let router = create_router(hub.clone());

        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .context("Gagal bind listener port acak")?;
        let addr = listener.local_addr()?;
        let url = format!("ws://{}/ws", addr);

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        println!("⚡ In-Process Relay Server berjalan pada: {}", url);
        (url, Some(hub))
    };

    // ── Langkah 2: Pembuatan Identitas Alice & Bob ───────────────────────
    let (alice_priv, alice_pub) = generate_identity();
    let (bob_priv, bob_pub) = generate_identity();
    let alice_id = Uuid::new_v4();
    let bob_id = Uuid::new_v4();

    println!("👤 Alice ID: {} (Fingerprint: {})", alice_id, compute_fingerprint(&alice_pub));
    println!("👤 Bob ID:   {} (Fingerprint: {})", bob_id, compute_fingerprint(&bob_pub));

    // ── Langkah 3: Setup Storage SQLCipher Lokal Terisolasi ──────────────
    let alice_dir = tempfile::tempdir().context("Gagal buat tempdir Alice")?;
    let alice_db_file = alice_dir.path().join("alice.db");
    let alice_db = EncryptedDatabase::open(alice_db_file.to_str().unwrap(), "alice-secret-pin-123456")
        .context("Gagal buka SQLCipher Alice")?;

    let bob_dir = tempfile::tempdir().context("Gagal buat tempdir Bob")?;
    let bob_db_file = bob_dir.path().join("bob.db");
    let bob_db = EncryptedDatabase::open(bob_db_file.to_str().unwrap(), "bob-secret-pin-654321")
        .context("Gagal buka SQLCipher Bob")?;

    // Daftarkan kontak Bob di buku kontak Alice
    alice_db.upsert_contact(&Contact {
        id: bob_id,
        username: "bob".to_string(),
        display_name: "Bob".to_string(),
        public_key: *bob_pub.as_bytes(),
        safety_number: compute_fingerprint(&bob_pub),
        status: "active".to_string(),
        updated_at: 1720000000,
    })?;

    // Daftarkan kontak Alice di buku kontak Bob
    bob_db.upsert_contact(&Contact {
        id: alice_id,
        username: "alice".to_string(),
        display_name: "Alice".to_string(),
        public_key: *alice_pub.as_bytes(),
        safety_number: compute_fingerprint(&alice_pub),
        status: "active".to_string(),
        updated_at: 1720000000,
    })?;

    let alice_engine = Arc::new(ChatEngine::new(
        alice_id,
        alice_priv,
        alice_pub,
        alice_db.clone(),
        EventBus::default(),
    ));

    let bob_engine = Arc::new(ChatEngine::new(
        bob_id,
        bob_priv,
        bob_pub,
        bob_db.clone(),
        EventBus::default(),
    ));

    // Daftarkan Bob sebagai Guardian sah Alice di server (untuk tes SOS)
    if let Some(hub) = &server_hub {
        hub.db()
            .add_guardian(&alice_id, &bob_id)
            .context("Gagal tautkan relasi Guardian di server")?;
        hub.db()
            .upsert_public_key(&alice_id, alice_pub.as_bytes())?;
        hub.db()
            .upsert_public_key(&bob_id, bob_pub.as_bytes())?;
        println!("🛡️  Tautan Guardian terdaftar: Bob adalah Guardian resmi Alice.");
    }

    // ── Langkah 4: Koneksi WebSocket Client ──────────────────────────────
    let conn_start = Instant::now();
    let mut alice_client = MekaarClient::connect(
        &server_url,
        alice_id,
        *alice_pub.as_bytes(),
        ReconnectPolicy::default(),
    )
    .await
    .context("Koneksi Alice gagal")?;

    let mut bob_client = MekaarClient::connect(
        &server_url,
        bob_id,
        *bob_pub.as_bytes(),
        ReconnectPolicy::default(),
    )
    .await
    .context("Koneksi Bob gagal")?;

    // Tunggu SubscribedAck dari server
    let ack_alice = tokio::time::timeout(Duration::from_secs(3), alice_client.recv())
        .await
        .context("Timeout menunggu SubscribedAck Alice")?
        .context("Channel Alice tertutup")?;
    let ack_bob = tokio::time::timeout(Duration::from_secs(3), bob_client.recv())
        .await
        .context("Timeout menunggu SubscribedAck Bob")?
        .context("Channel Bob tertutup")?;

    if ack_alice != (ServerCommand::SubscribedAck { user_id: alice_id }) {
        bail!("Respon registrasi Alice salah: {:?}", ack_alice);
    }
    if ack_bob != (ServerCommand::SubscribedAck { user_id: bob_id }) {
        bail!("Respon registrasi Bob salah: {:?}", ack_bob);
    }
    let conn_latency = conn_start.elapsed();
    println!("✅ WebSocket Handshake sukses (Waktu koneksi: {:.2?})", conn_latency);

    // ── Langkah 5: Pengujian Direct E2EE Chat ────────────────────────────
    println!("\n--- [Uji 1] Direct E2EE Chat & Delivery ACK ---");
    let room_id = Uuid::new_v4();
    let msg_text = "Halo Bob, ini adalah pesan terenkripsi E2EE via Aegis Protocol!";

    let msg_start = Instant::now();
    let (_local_msg, envelope) = alice_engine
        .send_message(
            room_id,
            bob_id,
            msg_text,
            MessageType::Text,
            false,
            None,
        )
        .await
        .context("Alice gagal mengenkripsi pesan")?;

    alice_client
        .send_command(ClientCommand::SendEnvelope(envelope.clone()))
        .context("Alice gagal mengirim envelope")?;

    // Bob menerima pesan
    let incoming = tokio::time::timeout(Duration::from_secs(3), bob_client.recv())
        .await
        .context("Timeout Bob menunggu pesan masuk")?
        .context("Channel Bob tertutup")?;

    let received_envelope = match incoming {
        ServerCommand::IncomingEnvelope(env) => env,
        other => bail!("Bob menerima perintah tak terduga: {:?}", other),
    };

    let decrypted = bob_engine
        .receive_envelope(received_envelope.clone())
        .await
        .context("Bob gagal mendekripsi ciphertext")?;

    if decrypted.content != msg_text {
        bail!(
            "Pesan terdekripsi tidak cocok! Ekspektasi: '{}', Dapat: '{}'",
            msg_text,
            decrypted.content
        );
    }
    let e2ee_latency = msg_start.elapsed();
    println!("🔒 Pesan terenkripsi & terdekripsi utuh: \"{}\"", decrypted.content);
    println!("⏱️  Latensi E2EE pengiriman & dekripsi: {:.2?}", e2ee_latency);

    // Bob mengirimkan DeliveryAck
    bob_client
        .send_command(ClientCommand::DeliveryAck {
            message_id: received_envelope.message_id,
            sender_id: alice_id,
        })
        .context("Bob gagal kirim DeliveryAck")?;

    // Alice menerima DeliveryReceipt
    let receipt = tokio::time::timeout(Duration::from_secs(3), alice_client.recv())
        .await
        .context("Timeout Alice menunggu DeliveryReceipt")?
        .context("Channel Alice tertutup")?;

    match receipt {
        ServerCommand::DeliveryReceipt { message_id, .. } => {
            if message_id != envelope.message_id {
                bail!("ID DeliveryReceipt tidak cocok");
            }
            alice_engine.mark_delivered(&message_id).await?;
            println!("📬 DeliveryReceipt terkonfirmasi: Status pesan diperbarui ke 'Delivered'");
        }
        other => bail!("Alice menerima bukan DeliveryReceipt: {:?}", other),
    }

    // ── Langkah 6: Pengujian Pesan Sekali Lihat (View-Once) ───────────────
    println!("\n--- [Uji 2] Pesan Sekali Lihat (View-Once) & Memory Scrubbing ---");
    let secret_view_once = "KODE_RAHASIA_VIEW_ONCE_9988";
    let (_vo_local, vo_envelope) = alice_engine
        .send_message(
            room_id,
            bob_id,
            secret_view_once,
            MessageType::Text,
            true, // view-once aktif!
            None,
        )
        .await
        .context("Alice gagal mengenkripsi view-once")?;

    alice_client
        .send_command(ClientCommand::SendEnvelope(vo_envelope.clone()))?;

    let incoming_vo = tokio::time::timeout(Duration::from_secs(3), bob_client.recv())
        .await
        .context("Timeout Bob menunggu pesan view-once")?
        .context("Channel Bob tertutup")?;

    let vo_env = match incoming_vo {
        ServerCommand::IncomingEnvelope(env) => env,
        other => bail!("Ekspektasi view-once envelope, dapat: {:?}", other),
    };

    let vo_decrypted = bob_engine
        .receive_envelope(vo_env)
        .await
        .context("Bob gagal dekripsi view-once")?;

    assert!(vo_decrypted.is_view_once);
    println!("👁️  Bob membuka pesan sekali lihat: \"{}\"", vo_decrypted.content);

    // Buka dan lakukan sanitasi memori/disk
    bob_db.mark_as_viewed(&vo_decrypted.id)?;

    // Verifikasi pesan telah terhapus/dikosongkan di database Bob
    let sanitized_msg = bob_db.get_message(&vo_decrypted.id)?;
    assert!(
        sanitized_msg.is_some() && sanitized_msg.as_ref().unwrap().is_opened,
        "Pesan view-once tidak tersanitasi!"
    );
    assert!(
        sanitized_msg.as_ref().unwrap().content.contains("telah dibuka"),
        "Konten view-once belum dioverwrite!"
    );
    println!("🧹 Sanitasi memori & disk sukses: Pesan sekali lihat telah di-scrub!");

    // ── Langkah 7: Pengujian Sinyal Darurat SOS Panic ────────────────────
    println!("\n--- [Uji 3] Sinyal Darurat SOS Panic Dispatcher ---");
    let sos_start = Instant::now();
    let emergency_lat = -6.2088;
    let emergency_lon = 106.8456;

    alice_client
        .send_command(ClientCommand::SosTrigger {
            user_id: alice_id,
            latitude: emergency_lat,
            longitude: emergency_lon,
            accuracy: 4.2,
            timestamp_ms: 1720000000,
        })
        .context("Alice gagal picu SOS")?;

    // Bob (sebagai Guardian) menerima siaran SOS
    let mut sos_alert = None;
    for _ in 0..5 {
        if let Ok(Some(cmd)) = tokio::time::timeout(Duration::from_secs(2), bob_client.recv()).await {
            if matches!(cmd, ServerCommand::GuardianSosBroadcast { .. }) {
                sos_alert = Some(cmd);
                break;
            }
        }
    }
    let sos_alert = sos_alert.context("Bob tidak menerima siaran GuardianSosBroadcast")?;

    match sos_alert {
        ServerCommand::GuardianSosBroadcast {
            victim_id,
            latitude,
            longitude,
            ..
        } => {
            if victim_id != alice_id {
                bail!("Victim ID pada siaran SOS tidak cocok");
            }
            assert_eq!(latitude, emergency_lat);
            assert_eq!(longitude, emergency_lon);
            println!(
                "🚨 Bob (Guardian) MENERIMA SOS ALERT! Korban: {}, Titik GPS: ({}, {})",
                victim_id, latitude, longitude
            );
        }
        other => bail!("Ekspektasi GuardianSosBroadcast, dapat: {:?}", other),
    }
    let sos_latency = sos_start.elapsed();
    println!("⚡ Latensi siaran SOS ke Guardian: {:.2?}", sos_latency);

    // Alice menyelesaikan status krisis
    alice_client
        .send_command(ClientCommand::SosResolve {
            user_id: alice_id,
            reason: "Situasi aman terkendali".to_string(),
        })
        .context("Alice gagal selesaikan SOS")?;
    println!("🕊️  Status krisis SOS ditutup secara normal.");

    // ── Langkah 8: Pengujian WebRTC Signaling Relay ──────────────────────
    println!("\n--- [Uji 4] WebRTC P2P Signaling Relay ---");
    let sdp_offer = "v=0\r\no=alice 123456 IN IP4 127.0.0.1\r\ns=SOS Emergency Stream";
    let offer_cmd = make_offer_signal(bob_id, sdp_offer);
    alice_client.send_command(offer_cmd)?;

    let mut relay_offer = None;
    for _ in 0..5 {
        if let Ok(Some(cmd)) = tokio::time::timeout(Duration::from_secs(2), bob_client.recv()).await {
            if matches!(cmd, ServerCommand::WebRtcRelay { .. }) {
                relay_offer = Some(cmd);
                break;
            }
        }
    }
    let relay_offer = relay_offer.context("Bob tidak menerima WebRtcRelay Offer")?;

    match relay_offer {
        ServerCommand::WebRtcRelay {
            from_user_id,
            signal_type,
            payload,
        } => {
            assert_eq!(from_user_id, alice_id);
            assert_eq!(signal_type, "offer");
            assert!(payload.contains("SOS Emergency Stream"));
            println!("📡 Bob menerima WebRTC SDP Offer dari Alice");
        }
        other => bail!("Ekspektasi WebRtcRelay Offer, dapat: {:?}", other),
    }

    let sdp_answer = "v=0\r\no=bob 654321 IN IP4 127.0.0.1\r\ns=SOS Answer Stream";
    let answer_cmd = make_answer_signal(alice_id, sdp_answer);
    bob_client.send_command(answer_cmd)?;

    let mut relay_answer = None;
    for _ in 0..5 {
        if let Ok(Some(cmd)) = tokio::time::timeout(Duration::from_secs(2), alice_client.recv()).await {
            if matches!(cmd, ServerCommand::WebRtcRelay { .. }) {
                relay_answer = Some(cmd);
                break;
            }
        }
    }
    let relay_answer = relay_answer.context("Alice tidak menerima WebRtcRelay Answer")?;

    match relay_answer {
        ServerCommand::WebRtcRelay {
            from_user_id,
            signal_type,
            payload,
        } => {
            assert_eq!(from_user_id, bob_id);
            assert_eq!(signal_type, "answer");
            assert!(payload.contains("SOS Answer Stream"));
            println!("📡 Alice menerima WebRTC SDP Answer dari Bob");
        }
        other => bail!("Ekspektasi WebRtcRelay Answer, dapat: {:?}", other),
    }

    // ── Penutupan Sesi Bersih ───────────────────────────────────────────
    alice_client.close();
    bob_client.close();

    let total_elapsed = start_total.elapsed();
    println!("\n========================================================");
    println!("🎉 SIMULASI E2E SUKSES PENUH!");
    println!("========================================================");
    println!("- Direct E2EE Chat:        LULUS ✅");
    println!("- Delivery Confirmation:   LULUS ✅");
    println!("- View-Once Sanitization:  LULUS ✅");
    println!("- SOS Panic Dispatcher:    LULUS ✅");
    println!("- WebRTC Signaling Relay:  LULUS ✅");
    println!("- Total Waktu Eksekusi:    {:.2?}", total_elapsed);
    println!("========================================================\n");

    Ok(())
}
