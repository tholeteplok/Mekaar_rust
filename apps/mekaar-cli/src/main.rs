//! # mekaar-cli — Headless Diagnostic & Testing Tool
//!
//! Binary CLI untuk:
//! - Simulasi dua perangkat saling kirim pesan E2EE via relay server (`simulate-pair`)
//! - Obrolan interaktif dua terminal E2EE (`chat`)
//! - Daemon headless background (`daemon`)
//! - Pengiriman cepat satu pesan (`send`)
//! - Pemicu panik darurat SOS (`sos`)

use anyhow::{Context, Result};
use clap::Parser;
use mekaar_cli::args::{Cli, Commands};
use mekaar_cli::{interactive, simulation};
use mekaar_core::events::EventBus;
use mekaar_core::ChatEngine;
use mekaar_crypto::{compute_fingerprint, generate_identity};
use mekaar_net::{MekaarClient, ReconnectPolicy};
use mekaar_proto::{ClientCommand, ServerCommand};
use mekaar_storage::EncryptedDatabase;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::SimulatePair { custom_server } => {
            let server_target = custom_server.or_else(|| {
                if cli.server_url != "ws://127.0.0.1:8085/ws" {
                    Some(cli.server_url.clone())
                } else {
                    None
                }
            });
            simulation::run_simulation(server_target).await?;
        }

        Commands::Chat {
            as_user,
            peer_id,
            pin,
            db_path,
        } => {
            interactive::run_interactive_chat(&cli.server_url, as_user, peer_id, &pin, db_path)
                .await?;
        }

        Commands::Daemon {
            user_id,
            pin,
            db_path,
        } => {
            let my_id = user_id.unwrap_or_else(Uuid::new_v4);
            let (priv_key, pub_key) = generate_identity();

            // P0: tolak path traversal; hanya izinkan filename sederhana di CWD atau path absolut kanonis.
            let effective_db = match db_path {
                Some(p) => {
                    if p.contains("..") {
                        anyhow::bail!("db_path mengandung '..' (path traversal ditolak)");
                    }
                    p
                }
                None => format!("daemon_{}.db", my_id),
            };
            let db = EncryptedDatabase::open(&effective_db, &pin)
                .context("Gagal membuka database SQLCipher")?;
            let engine = Arc::new(ChatEngine::new(
                my_id,
                priv_key,
                pub_key,
                db.clone(),
                EventBus::default(),
            ));

            println!("🦀 Menjalankan mekaar-cli daemon...");
            println!("👤 User ID:     {}", my_id);
            println!("🔑 Fingerprint: {}", compute_fingerprint(&pub_key));
            println!("🌐 Server:      {}", cli.server_url);

            let mut client = MekaarClient::connect(
                &cli.server_url,
                my_id,
                *pub_key.as_bytes(),
                ReconnectPolicy::infinite(),
            )
            .await?;

            println!("🟢 Daemon aktif. Menunggu event masuk (tekan Ctrl+C untuk berhenti)...");

            while let Some(cmd) = client.recv().await {
                match cmd {
                    ServerCommand::IncomingEnvelope(env) => {
                        println!("📥 Menerima pesan terenkripsi dari {}", env.sender_id);
                        // P0: hanya ACK setelah dekripsi + verifikasi berhasil (fail-closed).
                        match engine.receive_envelope(env.clone()).await {
                            Ok(_) => {
                                let _ = client.send_command(ClientCommand::DeliveryAck {
                                    message_id: env.message_id,
                                    sender_id: env.sender_id,
                                });
                            }
                            Err(e) => {
                                eprintln!("⚠️ Envelope ditolak tanpa ACK: {}", e);
                            }
                        }
                    }
                    ServerCommand::GuardianSosBroadcast {
                        victim_id,
                        latitude,
                        longitude,
                        ..
                    } => {
                        println!(
                            "🚨 [SOS BROADCAST] Korban {} di ({}, {})",
                            victim_id, latitude, longitude
                        );
                    }
                    ServerCommand::DeliveryReceipt { message_id, .. } => {
                        let _ = engine.mark_delivered(&message_id).await;
                        println!("✓ Pesan {} terkonfirmasi diterima", message_id);
                    }
                    _ => {}
                }
            }
        }

        Commands::Send {
            from: _from,
            to: _to,
            message,
            view_once: _view_once,
            pin: _pin,
            db_path,
        } => {
            if message.is_empty() || message.len() > 8000 {
                anyhow::bail!("message harus 1..8000 byte");
            }
            if db_path.as_deref().is_some_and(|p| p.contains("..")) {
                anyhow::bail!("db_path mengandung '..' (ditolak)");
            }
            // P0: jangan gunakan kunci nol. Send satu-shot butuh kunci peer asli;
            // tolak fail-closed dengan pesan jelas (bukan E2EE palsu).
            anyhow::bail!(
                "Perintah send satu-shot membutuhkan kunci publik peer asli di kontak (kunci nol ditolak). Gunakan 'chat' dengan kontak terverifikasi."
            );
        }

        Commands::Sos {
            user_id,
            lat,
            lon,
            accuracy,
        } => {
            if !accuracy.is_finite() || !(0.0..=100_000.0).contains(&accuracy) {
                anyhow::bail!("accuracy harus 0..100000 meter");
            }
            let my_id = user_id.unwrap_or_else(Uuid::new_v4);
            let (_priv, pub_key) = generate_identity();

            let client = MekaarClient::connect(
                &cli.server_url,
                my_id,
                *pub_key.as_bytes(),
                ReconnectPolicy::default(),
            )
            .await?;

            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            client.send_command(ClientCommand::SosTrigger {
                user_id: my_id,
                latitude: lat,
                longitude: lon,
                accuracy,
                timestamp_ms: now,
            })?;

            println!("🚨 Sinyal SOS darurat berhasil dipancarkan ke server untuk korban {}", my_id);
            println!("   Koordinat: ({}, {}), Akurasi: {}m", lat, lon, accuracy);
        }
    }

    Ok(())
}
