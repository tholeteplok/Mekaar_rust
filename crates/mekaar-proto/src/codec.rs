//! # mekaar-proto: Serialization Codec (Postcard & JSON)
//!
//! Abstraksi codec untuk:
//! - **Postcard:** Serialisasi biner efisien yang menghemat kuota transmisi darurat hingga 70%.
//! - **JSON:** Format human-readable untuk logging, debug, dan interoperabilitas.

use crate::error::ProtoError;
use serde::{Deserialize, Serialize};

/// Meng-encode data struktur Rust ke dalam format biner Postcard.
pub fn encode_postcard<T: Serialize>(value: &T) -> Result<Vec<u8>, ProtoError> {
    postcard::to_allocvec(value).map_err(|e| ProtoError::Postcard(e.to_string()))
}

/// Men-decode byte biner Postcard kembali ke tipe struktur Rust.
pub fn decode_postcard<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, ProtoError> {
    postcard::from_bytes(bytes).map_err(|e| ProtoError::Postcard(e.to_string()))
}

/// Meng-encode data struktur Rust ke dalam format JSON String.
pub fn encode_json<T: Serialize>(value: &T) -> Result<String, ProtoError> {
    serde_json::to_string(value).map_err(|e| ProtoError::Json(e.to_string()))
}

/// Men-decode string JSON kembali ke tipe struktur Rust.
pub fn decode_json<'a, T: Deserialize<'a>>(json_str: &'a str) -> Result<T, ProtoError> {
    serde_json::from_str(json_str).map_err(|e| ProtoError::Json(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::{MessageEnvelope, MessageType};
    use uuid::Uuid;

    #[test]
    fn test_codec_roundtrips() {
        let envelope = MessageEnvelope::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            [8u8; 24],
            vec![10, 20, 30, 40, 50],
            1700000000000,
            MessageType::Text,
        );

        // Test Postcard
        let postcard_bytes = encode_postcard(&envelope).expect("encode postcard");
        let decoded_postcard: MessageEnvelope =
            decode_postcard(&postcard_bytes).expect("decode postcard");
        assert_eq!(envelope, decoded_postcard);

        // Test JSON
        let json_str = encode_json(&envelope).expect("encode json");
        let decoded_json: MessageEnvelope = decode_json(&json_str).expect("decode json");
        assert_eq!(envelope, decoded_json);

        // Verifikasi efisiensi ukuran biner Postcard vs JSON
        assert!(postcard_bytes.len() < json_str.len());
    }
}
