//! # mekaar-proto — Wire Protocol & Serialisasi Biner
//!
//! Spesifikasi wire protocol dan schema komunikasi jaringan MEKAAR:
//! - **MessageEnvelope:** Paket ciphertext terenkripsi E2EE antar-pengguna
//! - **ClientCommand & ServerCommand:** Protokol frame WebSocket untuk relay server
//! - **Postcard Codec:** Serialisasi biner hemat transmisi darurat hingga 70%
//! - **JSON Codec:** Interoperabilitas dan log diagnostik

pub mod codec;
pub mod command;
pub mod envelope;
pub mod error;

pub use codec::{decode_json, decode_postcard, encode_json, encode_postcard};
pub use command::{ClientCommand, ServerCommand};
pub use envelope::{MessageEnvelope, MessageType};
pub use error::ProtoError;
