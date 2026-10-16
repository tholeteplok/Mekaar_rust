//! # mekaar-cli: Interactive Terminal Chat (REPL)
//!
//! Antarmuka obrolan terminal interaktif dua arah dengan enkripsi E2EE real-time.
//! Membaca input baris dari stdin tanpa memblokir stream pesan masuk dari WebSocket.

use anyhow::{Context, Result};
use mekaar_core::events::EventBus;
use mekaar_core::ChatEngine;
use mekaar_crypto::{compute_fingerprint, generate_identity};
use mekaar_net::{MekaarClient, ReconnectPolicy};
use mekaar_proto::{ClientCommand, MessageType, ServerCommand};
use mekaar_storage::EncryptedDatabase;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

/// Menjalankan sesi terminal chat interaktif.
pub async fn run_interactive_chat(
    server_url: &str,
    as_user: Option<Uuid>,
    peer_id: Uuid,
    pin: &str,
    db_path: Option<String>,
) -> Result<()> {
    let my_id = as_user.unwrap_or_else(Uuid::new_v4);
    let (my_priv, my_pub) = generate_identity();

    // Setup database lokal SQLCipher
    let temp_dir;
    let effective_db_path = match db_path {
        Some(path) => {
            if path.contains("..") {
                anyhow::bail!("db_path mengandung '..' (ditolak)");
            }
            path
        }
        None => {
            temp_dir = tempfile::tempdir().context("Gagal buat direktori sementara DB")?;
            temp_dir
                .path()
                .join("chat.db")
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("DB temp path bukan UTF-8"))?
                .to_string()
        }
    };

    let db = EncryptedDatabase::open(&effective_db_path, pin)
        .context("Gagal membuka database SQLCipher lokal")?;

    // P0: jangan tanam kontak kunci-nol (E2EE palsu). Wajib sudah ada kontak terverifikasi.
    if db
        .get_contact(&peer_id)?
        .map(|c| c.public_key == [0u8; 32])
        .unwrap_or(true)
    {
        anyhow::bail!(
            "Kontak peer belum terverifikasi (kunci nol/hilang). Daftarkan kunci publik asli dulu."
        );
    }

    let engine = Arc::new(ChatEngine::new(
        my_id,
        my_priv,
        my_pub,
        db.clone(),
        EventBus::default(),
    ));

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║ 💬 MEKAAR Interactive Terminal Chat (E2EE)             ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!("👤 User Anda:     {}", my_id);
    println!("👥 Lawan Bicara:  {}", peer_id);
    println!("🔑 Fingerprint:   {}", compute_fingerprint(&my_pub));
    println!("🌐 Server:        {}", server_url);
    println!("--------------------------------------------------------");
    println!("💡 Ketik pesan lalu tekan ENTER.");
    println!("   Gunakan '/viewonce <pesan>' untuk pesan sekali lihat.");
    println!("   Gunakan '/sos' untuk memicu siaran panik darurat.");
    println!("   Gunakan '/quit' untuk keluar dari sesi.");
    println!("--------------------------------------------------------\n");

    // Hubungkan transport WebSocket
    let mut client = MekaarClient::connect(
        server_url,
        my_id,
        *my_pub.as_bytes(),
        ReconnectPolicy::default(),
    )
    .await
    .context("Gagal menghubungkan ke relay server")?;

    let sender = client.sender();
    let engine_rx = engine.clone();
    let db_rx = db.clone();
    let room_id = Uuid::new_v4();

    // Background task: Membaca pesan masuk dari WebSocket
    let rx_handle = tokio::spawn(async move {
        while let Some(cmd) = client.recv().await {
            match cmd {
                ServerCommand::SubscribedAck { .. } => {
                    println!("[Sistem] Terhubung dan terautentikasi ke Relay Server! 🟢");
                }
                ServerCommand::IncomingEnvelope(env) => {
                    match engine_rx.receive_envelope(env.clone()).await {
                        Ok(msg) => {
                            if msg.is_view_once {
                                println!("\n[Lawan Bicara]: 👁️  {} (Sekali Lihat)", msg.content);
                                let _ = db_rx.mark_as_viewed(&msg.id);
                            } else {
                                println!("\n[Lawan Bicara]: {} 🔒", msg.content);
                            }
                            let _ = client.send_command(ClientCommand::DeliveryAck {
                                message_id: env.message_id,
                                sender_id: env.sender_id,
                            });
                        }
                        Err(_) => {
                            println!("\n[Lawan Bicara]: [Paket terenkripsi diterima - {} bytes]", env.ciphertext.len());
                            // Tanpa ACK: dekripsi/verifikasi gagal, jangan hapus dari outbox server.
                        }
                    }
                }
                ServerCommand::DeliveryReceipt { message_id, .. } => {
                    let _ = engine_rx.mark_delivered(&message_id).await;
                    println!("    [✓✓ Terkirim ke penerima]");
                }
                ServerCommand::GuardianSosBroadcast {
                    victim_id,
                    latitude,
                    longitude,
                    ..
                } => {
                    println!(
                        "\n🚨🚨 [SOS ALERT] Korban {} memicu darurat di lokasi ({}, {})! 🚨🚨",
                        victim_id, latitude, longitude
                    );
                }
                ServerCommand::Error { code, message } => {
                    println!("[Server Error {code}]: {message}");
                }
                _ => {}
            }
        }
    });

    // Main task: Membaca input dari terminal stdin
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "/quit" {
            println!("\nMenutup sesi chat...");
            break;
        }

        if trimmed == "/sos" {
            println!("[Sistem] Memicu sinyal SOS darurat...");
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            sender.send(ClientCommand::SosTrigger {
                user_id: my_id,
                latitude: -6.2088,
                longitude: 106.8456,
                accuracy: 5.0,
                timestamp_ms: now,
            })?;
            continue;
        }

        let (is_view_once, content) = if let Some(stripped) = trimmed.strip_prefix("/viewonce ") {
            (true, stripped.trim())
        } else {
            (false, trimmed)
        };

        match engine
            .send_message(
                room_id,
                peer_id,
                content,
                MessageType::Text,
                is_view_once,
                None,
            )
            .await
        {
            Ok((_local_msg, envelope)) => {
                sender.send(ClientCommand::SendEnvelope(envelope))?;
                if is_view_once {
                    println!("[Anda]: {} (👁️ Sekali Lihat)", content);
                } else {
                    println!("[Anda]: {}", content);
                }
            }
            Err(e) => {
                println!("[Error Mengirim]: {}", e);
            }
        }
    }

    rx_handle.abort();
    Ok(())
}
