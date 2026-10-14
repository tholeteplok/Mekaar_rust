//! # mekaar-server: Realtime Hub & Dispatcher
//!
//! Mengelola koneksi WebSocket aktif secara konkuren:
//! - Direct sub-millisecond envelope relay antar klien online.
//! - Offline Store-and-Forward ke SQLite WAL jika penerima offline.
//! - Auto-flushing pesan offline saat penerima reconnect.
//! - SOS panic priority dispatcher ke seluruh Guardian terdaftar.
//! - WebRTC SDP/ICE signaling relay.

use crate::db::Database;
use mekaar_proto::{
    decode_postcard, encode_postcard, ClientCommand, MessageEnvelope, MessageType, ServerCommand,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Hub sentral yang mengorkestrasi seluruh perutean pesan real-time dan offline.
#[derive(Clone)]
pub struct Hub {
    peers: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<ServerCommand>>>>,
    db: Arc<Database>,
}

impl Hub {
    /// Membuat instance Hub baru dengan referensi database.
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            db,
        }
    }

    /// Mendapatkan referensi database server.
    pub fn db(&self) -> &Arc<Database> {
        &self.db
    }

    /// Jumlah koneksi WebSocket yang saat ini aktif.
    pub async fn active_connection_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.len()
    }

    /// Mendaftarkan koneksi klien baru, menerbitkan `SubscribedAck`,
    /// dan otomatis mengalirkan (flush) seluruh pesan offline yang tertunda.
    pub async fn connect_peer(
        &self,
        user_id: Uuid,
        public_key: Option<[u8; 32]>,
        tx: mpsc::UnboundedSender<ServerCommand>,
    ) {
        // Simpan kunci publik jika disertakan
        if let Some(pub_key) = public_key {
            if let Err(e) = self.db.upsert_public_key(&user_id, &pub_key) {
                error!("Gagal menyimpan kunci publik saat peer connect: {}", e);
            }
        }

        // Daftarkan pengirim ke peers aktif
        {
            let mut peers = self.peers.write().await;
            peers.insert(user_id, tx.clone());
        }

        info!("Klien {} terhubung ke Hub", user_id);

        // 1. Kirim konfirmasi pendaftaran
        let _ = tx.send(ServerCommand::SubscribedAck { user_id });

        // 2. Flush pesan offline yang tertunda
        match self.db.get_offline_messages(&user_id) {
            Ok(messages) => {
                let count = messages.len();
                if count > 0 {
                    info!("Mengalirkan {} pesan offline tertunda ke klien {}", count, user_id);
                    for (_msg_id, blob) in messages {
                        if let Ok(envelope) = decode_postcard::<MessageEnvelope>(&blob) {
                            if tx.send(ServerCommand::IncomingEnvelope(envelope)).is_err() {
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Gagal mengambil pesan offline untuk {}: {}", user_id, e);
            }
        }
    }

    /// Menghapus koneksi klien saat WebSocket terputus.
    pub async fn disconnect_peer(&self, user_id: &Uuid) {
        let mut peers = self.peers.write().await;
        peers.remove(user_id);
        info!("Klien {} terputus dari Hub", user_id);
    }

    /// Memproses perintah wire yang diterima dari klien via WebSocket.
    pub async fn handle_command(&self, sender_id: Option<Uuid>, cmd: ClientCommand) {
        match cmd {
            ClientCommand::AuthSubscribe { user_id, public_key } => {
                // Di-handle saat handshake awal WebSocket
                if let Err(e) = self.db.upsert_public_key(&user_id, &public_key) {
                    error!("Gagal update public key pada AuthSubscribe: {}", e);
                }
            }

            ClientCommand::SendEnvelope(envelope) => {
                self.relay_envelope(envelope).await;
            }

            ClientCommand::DeliveryAck { message_id, sender_id } => {
                self.handle_delivery_ack(message_id, sender_id).await;
            }

            ClientCommand::SosTrigger {
                user_id,
                latitude,
                longitude,
                accuracy,
                timestamp_ms,
            } => {
                self.handle_sos_trigger(user_id, latitude, longitude, accuracy, timestamp_ms).await;
            }

            ClientCommand::SosResolve { user_id, reason } => {
                info!("SOS untuk user {} telah diselesaikan (Alasan: {})", user_id, reason);
            }

            ClientCommand::WebRtcSignal {
                target_user_id,
                signal_type,
                payload,
            } => {
                if let Some(from_id) = sender_id {
                    self.relay_webrtc(from_id, target_user_id, signal_type, payload).await;
                }
            }

            ClientCommand::Ping => {
                if let Some(uid) = sender_id {
                    let peers = self.peers.read().await;
                    if let Some(tx) = peers.get(&uid) {
                        let _ = tx.send(ServerCommand::Pong);
                    }
                }
            }
        }
    }

    /// Meneruskan envelope pesan terenkripsi.
    /// Jika penerima online: seketika dikirim ke WebSocket penerima.
    /// Jika penerima offline: disimpan ke database SQLite untuk dikirim saat online.
    pub async fn relay_envelope(&self, envelope: MessageEnvelope) {
        let recipient_id = envelope.recipient_id;
        let sender_id = envelope.sender_id;
        let message_id = envelope.message_id;

        let peers = self.peers.read().await;
        if let Some(recipient_tx) = peers.get(&recipient_id) {
            // Penerima sedang ONLINE: kirim langsung
            debug!("Routing envelope {} langsung ke penerima online {}", message_id, recipient_id);
            if recipient_tx.send(ServerCommand::IncomingEnvelope(envelope.clone())).is_ok() {
                // Beri tahu pengirim bahwa pesan telah sampai di antrean server
                if let Some(sender_tx) = peers.get(&sender_id) {
                    let _ = sender_tx.send(ServerCommand::DeliveryReceipt {
                        message_id,
                        timestamp_ms: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                    });
                }
                return;
            }
        }

        // Penerima OFFLINE (atau send gagal): simpan ke SQLite outbox
        debug!("Penerima {} offline, menyimpan envelope {} ke SQLite outbox", recipient_id, message_id);
        match encode_postcard(&envelope) {
            Ok(blob) => {
                if let Err(e) = self.db.store_offline_message(
                    &message_id,
                    &recipient_id,
                    &sender_id,
                    &blob,
                    envelope.expires_at,
                ) {
                    error!("Gagal menyimpan pesan offline {}: {}", message_id, e);
                } else if let Some(sender_tx) = peers.get(&sender_id) {
                    // Konfirmasi ke pengirim bahwa server telah menerima & menyimpan pesan
                    let _ = sender_tx.send(ServerCommand::DeliveryReceipt {
                        message_id,
                        timestamp_ms: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                    });
                }
            }
            Err(e) => {
                error!("Gagal encode envelope {}: {}", message_id, e);
            }
        }
    }

    /// Menangani konfirmasi penerimaan pesan (DeliveryAck).
    async fn handle_delivery_ack(&self, message_id: Uuid, sender_id: Uuid) {
        // 1. Hapus dari database offline jika ada
        if let Err(e) = self.db.delete_offline_message(&message_id) {
            warn!("Gagal menghapus pesan offline {}: {}", message_id, e);
        }

        // 2. Teruskan tanda terima ke pengirim asli jika sedang online
        let peers = self.peers.read().await;
        if let Some(sender_tx) = peers.get(&sender_id) {
            let _ = sender_tx.send(ServerCommand::DeliveryReceipt {
                message_id,
                timestamp_ms: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            });
        }
    }

    /// Menangani pemicu darurat SOS dan menyiarkannya ke seluruh Guardian korban.
    async fn handle_sos_trigger(
        &self,
        victim_id: Uuid,
        latitude: f64,
        longitude: f64,
        accuracy: f32,
        timestamp_ms: u64,
    ) {
        warn!("🚨 SOS TRIGGERED dari korban: {} (Lat: {}, Lng: {})", victim_id, latitude, longitude);

        let guardians = match self.db.get_guardians_for_victim(&victim_id) {
            Ok(g) => g,
            Err(e) => {
                error!("Gagal memuat guardian untuk korban {}: {}", victim_id, e);
                return;
            }
        };

        let broadcast_cmd = ServerCommand::GuardianSosBroadcast {
            victim_id,
            latitude,
            longitude,
            accuracy,
            timestamp_ms,
        };

        let peers = self.peers.read().await;
        for gid in guardians {
            if let Some(gtx) = peers.get(&gid) {
                info!("Menyiarkan sinyal darurat SOS ke Guardian online {}", gid);
                let _ = gtx.send(broadcast_cmd.clone());
            } else {
                debug!("Guardian {} offline saat SOS korban dipicu", gid);
                // Buat pesan sistem offline darurat untuk Guardian
                let urgent_envelope = MessageEnvelope::new(
                    victim_id,
                    gid,
                    victim_id,
                    [0u8; 24],
                    format!("🚨 SOS: {},{}", latitude, longitude).into_bytes(),
                    timestamp_ms,
                    MessageType::System,
                );
                if let Ok(blob) = encode_postcard(&urgent_envelope) {
                    let _ = self.db.store_offline_message(
                        &urgent_envelope.message_id,
                        &gid,
                        &victim_id,
                        &blob,
                        None,
                    );
                }
            }
        }
    }

    /// Meneruskan sinyal WebRTC SDP atau ICE candidate langsung ke pengguna tujuan.
    async fn relay_webrtc(&self, from_id: Uuid, to_id: Uuid, signal_type: String, payload: String) {
        let peers = self.peers.read().await;
        if let Some(to_tx) = peers.get(&to_id) {
            let _ = to_tx.send(ServerCommand::WebRtcRelay {
                from_user_id: from_id,
                signal_type,
                payload,
            });
        } else if let Some(from_tx) = peers.get(&from_id) {
            let _ = from_tx.send(ServerCommand::Error {
                code: 404,
                message: format!("Target user {} sedang offline", to_id),
            });
        }
    }
}
