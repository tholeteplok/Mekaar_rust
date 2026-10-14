//! # mekaar-net: WebSocket Client & Transport Layer
//!
//! Klien WebSocket asinkron ke Relay Server di teplok-pc / localhost.
//! Menangani:
//! - Handshake registrasi otomatis (`ClientCommand::AuthSubscribe`)
//! - Pumping paket biner Postcard (efisiensi kuota) dan fallback JSON
//! - Auto-reconnection dengan exponential backoff (`ReconnectPolicy`)
//! - Heartbeat ping/pong periodik (30 detik)

use crate::error::NetError;
use crate::reconnect::ReconnectPolicy;
use futures_util::{SinkExt, StreamExt};
use mekaar_proto::{
    decode_json, decode_postcard, encode_json, encode_postcard, ClientCommand, ServerCommand,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Pengirim perintah yang dapat di-clone ke berbagai task/thread.
#[derive(Clone)]
pub struct MekaarSender {
    tx: mpsc::UnboundedSender<ClientCommand>,
}

impl MekaarSender {
    /// Mengirimkan perintah ke antrean relay server.
    pub fn send(&self, cmd: ClientCommand) -> Result<(), NetError> {
        self.tx
            .send(cmd)
            .map_err(|_| NetError::Disconnected)
    }
}

/// Klien transportasi WebSocket MEKAAR.
pub struct MekaarClient {
    user_id: Uuid,
    public_key: [u8; 32],
    server_url: String,
    sender: MekaarSender,
    rx_incoming: mpsc::UnboundedReceiver<ServerCommand>,
    is_connected: Arc<AtomicBool>,
    supervisor_handle: tokio::task::JoinHandle<()>,
}

impl MekaarClient {
    /// Menghubungkan klien ke server relay dengan kebijakan reconnect tertentu.
    pub async fn connect(
        server_url: &str,
        user_id: Uuid,
        public_key: [u8; 32],
        policy: ReconnectPolicy,
    ) -> Result<Self, NetError> {
        let (tx_to_server, rx_from_client) = mpsc::unbounded_channel::<ClientCommand>();
        let (tx_to_client, rx_incoming) = mpsc::unbounded_channel::<ServerCommand>();
        let is_connected = Arc::new(AtomicBool::new(false));

        let sender = MekaarSender {
            tx: tx_to_server.clone(),
        };

        let server_url_clone = server_url.to_string();
        let is_conn_clone = is_connected.clone();

        // Coba koneksi pertama kali untuk memvalidasi ketersediaan awal
        let initial_ws = match connect_async(server_url).await {
            Ok((ws, _)) => {
                info!("Terhubung ke WebSocket server: {}", server_url);
                is_conn_clone.store(true, Ordering::SeqCst);
                Some(ws)
            }
            Err(e) => {
                warn!(
                    "Koneksi awal ke {} gagal: {}. Memulai reconnect supervisor...",
                    server_url, e
                );
                None
            }
        };

        // Jalankan supervisor task di latar belakang
        let supervisor_handle = tokio::spawn(async move {
            run_client_supervisor(
                server_url_clone,
                user_id,
                public_key,
                policy,
                initial_ws,
                rx_from_client,
                tx_to_client,
                is_conn_clone,
            )
            .await;
        });

        Ok(Self {
            user_id,
            public_key,
            server_url: server_url.to_string(),
            sender,
            rx_incoming,
            is_connected,
            supervisor_handle,
        })
    }

    /// ID pengguna terdaftar.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Public key identitas pengguna.
    pub fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }

    /// URL relay server target.
    pub fn server_url(&self) -> &str {
        &self.server_url
    }

    /// Status koneksi aktif.
    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }

    /// Mengambil sender cloneable untuk mengirim perintah ke server.
    pub fn sender(&self) -> MekaarSender {
        self.sender.clone()
    }

    /// Mengirimkan ClientCommand ke relay server.
    pub fn send_command(&self, cmd: ClientCommand) -> Result<(), NetError> {
        self.sender.send(cmd)
    }

    /// Menerima perintah ServerCommand berikutnya dari relay server.
    pub async fn recv(&mut self) -> Option<ServerCommand> {
        self.rx_incoming.recv().await
    }

    /// Menutup sesi klien dan menghentikan supervisor latar belakang.
    pub fn close(&self) {
        self.supervisor_handle.abort();
        self.is_connected.store(false, Ordering::SeqCst);
    }
}

impl Drop for MekaarClient {
    fn drop(&mut self) {
        self.supervisor_handle.abort();
    }
}

/// Loop supervisor yang menjaga koneksi WebSocket tetap hidup dan menangani auto-reconnect.
async fn run_client_supervisor(
    server_url: String,
    user_id: Uuid,
    public_key: [u8; 32],
    policy: ReconnectPolicy,
    mut initial_ws: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    mut rx_from_client: mpsc::UnboundedReceiver<ClientCommand>,
    tx_to_client: mpsc::UnboundedSender<ServerCommand>,
    is_connected: Arc<AtomicBool>,
) {
    let mut attempt = 0u32;

    loop {
        let ws_stream = if let Some(ws) = initial_ws.take() {
            ws
        } else {
            attempt += 1;
            if !policy.should_retry(attempt) {
                error!("Maksimal percobaan reconnect ({:?}) tercapai. Menghentikan transport client.", policy.max_attempts);
                break;
            }

            let delay = policy.delay_for_attempt(attempt);
            debug!("Menunggu {:?} sebelum mencoba reconnect (percobaan {})...", delay, attempt);
            tokio::time::sleep(delay).await;

            match connect_async(&server_url).await {
                Ok((ws, _)) => {
                    info!("Berhasil terkoneksi kembali ke relay server!");
                    attempt = 0;
                    is_connected.store(true, Ordering::SeqCst);
                    ws
                }
                Err(e) => {
                    warn!("Gagal menyambung ke server {}: {}", server_url, e);
                    continue;
                }
            }
        };

        // Pisahkan split reader dan writer
        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        // 1. Kirimkan AuthSubscribe handshake
        let auth_cmd = ClientCommand::AuthSubscribe {
            user_id,
            public_key,
        };
        let auth_bytes = encode_postcard(&auth_cmd).unwrap_or_else(|_| encode_json(&auth_cmd).unwrap().into_bytes());
        if let Err(e) = ws_sink.send(Message::Binary(auth_bytes.into())).await {
            warn!("Gagal mengirim AuthSubscribe: {}", e);
            is_connected.store(false, Ordering::SeqCst);
            continue;
        }

        // 2. Loop pump dua arah dengan keepalive
        let mut ping_interval = tokio::time::interval(Duration::from_secs(30));
        // Konsumsi tick pertama agar tidak langsung memicu ping seketika
        ping_interval.tick().await;
        let mut connection_alive = true;

        while connection_alive {
            tokio::select! {
                // Outgoing command dari klien
                Some(cmd) = rx_from_client.recv() => {
                    let msg = match encode_postcard(&cmd) {
                        Ok(b) => Message::Binary(b.into()),
                        Err(_) => match encode_json(&cmd) {
                            Ok(s) => Message::Text(s.into()),
                            Err(e) => {
                                warn!("Gagal serialisasi ClientCommand: {}", e);
                                continue;
                            }
                        }
                    };

                    if let Err(e) = ws_sink.send(msg).await {
                        warn!("Gagal mengirim pesan ke socket: {}", e);
                        connection_alive = false;
                    }
                }

                // Heartbeat ping berkala
                _ = ping_interval.tick() => {
                    if let Err(e) = ws_sink.send(Message::Ping(vec![].into())).await {
                        debug!("Gagal mengirim ping keepalive: {}", e);
                        connection_alive = false;
                    }
                }

                // Incoming frame dari server
                incoming = ws_stream.next() => {
                    match incoming {
                        Some(Ok(msg)) => {
                            let server_cmd: Option<ServerCommand> = match msg {
                                Message::Binary(bytes) => {
                                    decode_postcard::<ServerCommand>(&bytes)
                                        .or_else(|_| decode_json::<ServerCommand>(&String::from_utf8_lossy(&bytes)))
                                        .ok()
                                }
                                Message::Text(text) => {
                                    decode_json::<ServerCommand>(&text).ok()
                                }
                                Message::Ping(payload) => {
                                    let _ = ws_sink.send(Message::Pong(payload)).await;
                                    None
                                }
                                Message::Pong(_) => None,
                                Message::Close(_) => {
                                    info!("Server menutup koneksi WebSocket");
                                    connection_alive = false;
                                    None
                                }
                                _ => None,
                            };

                            if let Some(cmd) = server_cmd {
                                // Pong adalah sinyal keepalive internal, tidak perlu diteruskan ke aplikasi
                                if matches!(cmd, ServerCommand::Pong) {
                                    continue;
                                }

                                if tx_to_client.send(cmd).is_err() {
                                    info!("Receiver channel klien telah di-drop. Menghentikan supervisor.");
                                    return;
                                }
                            }
                        }
                        Some(Err(e)) => {
                            warn!("Error membaca frame WebSocket: {}", e);
                            connection_alive = false;
                        }
                        None => {
                            warn!("WebSocket stream EOF (terputus)");
                            connection_alive = false;
                        }
                    }
                }
            }
        }

        is_connected.store(false, Ordering::SeqCst);
        warn!("Koneksi WebSocket terputus. Memulai siklus reconnect...");
    }
}
