//! # mekaar-net — Transport & Media Streaming
//!
//! Layer jaringan dan komunikasi MEKAAR:
//! - **WebSocket Client (`MekaarClient`):** Transport asinkron dua arah via `tokio-tungstenite`
//! - **Auto-Reconnection (`ReconnectPolicy`):** Exponential backoff selaras `reconnect_policy.dart` Flutter
//! - **WebRTC Signaling:** Pertukaran SDP Offer/Answer dan ICE Candidate untuk live SOS audio/video
//! - **Codec Negotiation:** Pengiriman paket biner Postcard hemat kuota dan fallback JSON

pub mod client;
pub mod error;
pub mod reconnect;
pub mod signaling;

pub use client::{MekaarClient, MekaarSender};
pub use error::NetError;
pub use reconnect::ReconnectPolicy;
pub use signaling::{
    make_answer_signal, make_candidate_signal, make_hangup_signal, make_offer_signal,
    IceCandidatePayload, SdpPayload,
};

/// Status koneksi ke relay server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// Belum terkoneksi.
    Disconnected,
    /// Sedang mencoba koneksi.
    Connecting,
    /// Terkoneksi dan siap kirim/terima pesan.
    Connected,
    /// Reconnecting setelah koneksi terputus.
    Reconnecting { attempt: u32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_states_exist() {
        let state = ConnectionState::Disconnected;
        assert_eq!(state, ConnectionState::Disconnected);
    }
}
