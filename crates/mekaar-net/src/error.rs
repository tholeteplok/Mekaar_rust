//! # mekaar-net: Error Definitions

/// Error yang dapat terjadi pada transport layer jaringan MEKAAR.
#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("Koneksi WebSocket gagal: {0}")]
    WebSocketConnection(String),

    #[error("Koneksi terputus dari server")]
    Disconnected,

    #[error("Timeout koneksi setelah {0}ms")]
    Timeout(u64),

    #[error("Gagal serialisasi/deserialisasi payload: {0}")]
    Serialization(String),

    #[error("I/O Error: {0}")]
    Io(String),
}
