//! # mekaar-core: Chat Engine
//!
//! Mengendalikan seluruh siklus pengiriman dan penerimaan pesan obrolan E2EE:
//! - Derivasi kunci room per-percakapan dengan prinsip *Fail-Closed*.
//! - Enkripsi simetris XChaCha20-Poly1305 dan pengemasan ke wire `MessageEnvelope`.
//! - Dekripsi pesan masuk dan persistensi aman ke `EncryptedDatabase`.
//! - Pembaruan status pengiriman (Queued → Sent → Delivered → Read).

use crate::error::CoreError;
use crate::events::{CoreEvent, EventBus};
use mekaar_crypto::{
    decrypt_message_with_aad, derive_room_key, encrypt_message_with_aad, IdentityPrivateKey,
    IdentityPublicKey, MAX_MESSAGE_BYTES,
};
use mekaar_proto::{MessageEnvelope, MessageType};
use mekaar_storage::{DeliveryStatus, EncryptedDatabase, LocalMessage};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// AAD yang mengikat konteks pesan: domain || room || sender || recipient || message_id || timestamp.
/// Mencegah replay lintas-room dan swap routing oleh relay aktif.
fn build_message_aad(
    room_id: &Uuid,
    sender_id: &Uuid,
    recipient_id: &Uuid,
    message_id: &Uuid,
    timestamp_ms: u64,
) -> Vec<u8> {
    let mut aad = Vec::with_capacity(16 + 16 * 4 + 8);
    aad.extend_from_slice(b"mekaar-aegis-v1");
    aad.extend_from_slice(room_id.as_bytes());
    aad.extend_from_slice(sender_id.as_bytes());
    aad.extend_from_slice(recipient_id.as_bytes());
    aad.extend_from_slice(message_id.as_bytes());
    aad.extend_from_slice(&timestamp_ms.to_be_bytes());
    aad
}

/// Skew maksimum timestamp yang diterima (5 menit ke depan).
const MAX_FUTURE_SKEW_MS: u64 = 5 * 60 * 1000;

/// Engine pengendali perpesanan E2EE lokal.
#[derive(Clone)]
pub struct ChatEngine {
    my_user_id: Uuid,
    identity_priv: Arc<RwLock<IdentityPrivateKey>>,
    identity_pub: IdentityPublicKey,
    storage: EncryptedDatabase,
    event_bus: EventBus,
}

impl ChatEngine {
    /// Membuat instance ChatEngine baru untuk pengguna tertentu.
    pub fn new(
        my_user_id: Uuid,
        identity_priv: IdentityPrivateKey,
        identity_pub: IdentityPublicKey,
        storage: EncryptedDatabase,
        event_bus: EventBus,
    ) -> Self {
        Self {
            my_user_id,
            identity_priv: Arc::new(RwLock::new(identity_priv)),
            identity_pub,
            storage,
            event_bus,
        }
    }

    /// Mendapatkan ID pengguna saat ini.
    pub fn my_user_id(&self) -> &Uuid {
        &self.my_user_id
    }

    /// Mendapatkan public key identitas pengguna saat ini.
    pub fn my_public_key(&self) -> &IdentityPublicKey {
        &self.identity_pub
    }

    /// Mengirim pesan baru:
    /// 1. Ambil public key lawan dari buku kontak lokal.
    /// 2. Derivasi RoomSessionKey via HKDF-SHA256 (`mekaar-room:{room_id}`).
    /// 3. Enkripsi teks menggunakan XChaCha20-Poly1305 AEAD.
    /// 4. Simpan salinan lokal di SQLCipher dengan status `Queued`.
    /// 5. Kemas ke `MessageEnvelope` biner untuk ditransmisikan via jaringan.
    pub async fn send_message(
        &self,
        room_id: Uuid,
        recipient_id: Uuid,
        content: &str,
        msg_type: MessageType,
        is_view_once: bool,
        disappearing_duration_ms: Option<u64>,
    ) -> Result<(LocalMessage, MessageEnvelope), CoreError> {
        if content.is_empty() || content.len() > MAX_MESSAGE_BYTES {
            return Err(CoreError::InvalidState(
                "Konten pesan harus 1..64KiB".to_string(),
            ));
        }
        // 1. Ambil public key penerima
        let contact = self
            .storage
            .get_contact(&recipient_id)?
            .ok_or(CoreError::PeerNotFound(recipient_id))?;
        if contact.public_key == [0u8; 32] {
            return Err(CoreError::MissingPublicKey(recipient_id));
        }

        let peer_pub = IdentityPublicKey::from_bytes(contact.public_key);

        // 2. Derivasi kunci room
        let priv_key = self.identity_priv.read().await;
        let room_key = derive_room_key(&priv_key, &peer_pub, &room_id.to_string())?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let auto_delete_at = match disappearing_duration_ms {
            Some(d) => Some(now.checked_add(d).ok_or_else(|| {
                CoreError::InvalidState("disappearing timer overflow".to_string())
            })?),
            None => None,
        };
        let message_id = Uuid::new_v4();

        // 3. Enkripsi konten plaintext dengan AAD konteks
        let aad = build_message_aad(&room_id, &self.my_user_id, &recipient_id, &message_id, now);
        let (nonce, ciphertext) =
            encrypt_message_with_aad(&room_key, content.as_bytes(), &aad)?;

        // 4. Buat dan simpan model lokal
        let local_msg = LocalMessage {
            id: message_id,
            room_id,
            sender_id: self.my_user_id,
            recipient_id,
            content: content.to_string(),
            media_url: None,
            msg_type,
            is_view_once,
            is_opened: false,
            opened_at: None,
            reply_to_id: None,
            is_deleted: false,
            is_silent_deleted: false,
            auto_delete_at,
            edited_at: None,
            reactions: HashMap::new(),
            is_encrypted: true,
            created_at: now,
            updated_at: now,
            status: DeliveryStatus::Queued,
        };

        self.storage.insert_message(&local_msg)?;

        // 5. Kemas ke MessageEnvelope wire
        let mut envelope = MessageEnvelope::new(
            self.my_user_id,
            recipient_id,
            room_id,
            nonce,
            ciphertext,
            now,
            msg_type,
        );
        envelope.message_id = message_id;
        envelope.is_view_once = is_view_once;
        envelope.expires_at = auto_delete_at;

        self.event_bus
            .emit(CoreEvent::MessageStatusUpdated {
                message_id,
                status: DeliveryStatus::Queued,
            });

        Ok((local_msg, envelope))
    }

    /// Menerima dan memproses envelope pesan masuk dari wire:
    /// 1. Ambil public key pengirim.
    /// 2. Derivasi RoomSessionKey.
    /// 3. Dekripsi ciphertext XChaCha20-Poly1305.
    /// 4. Simpan plaintext hasil dekripsi ke SQLCipher lokal.
    /// 5. Emit event `CoreEvent::MessageReceived`.
    pub async fn receive_envelope(
        &self,
        envelope: MessageEnvelope,
    ) -> Result<LocalMessage, CoreError> {
        let sender_id = envelope.sender_id;

        // Fail-closed: tolak pesan dari diri sendiri (spoof / loop).
        if sender_id == self.my_user_id {
            return Err(CoreError::InvalidState(
                "Envelope dari diri sendiri ditolak".to_string(),
            ));
        }
        // Anti-replay: tolak message_id yang sudah diproses.
        if self.storage.get_message(&envelope.message_id)?.is_some() {
            return Err(CoreError::InvalidState(
                "Duplikat message_id (replay) ditolak".to_string(),
            ));
        }
        // Freshness: tolak timestamp masa depan di luar skew.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        if envelope.timestamp_ms > now.saturating_add(MAX_FUTURE_SKEW_MS) {
            return Err(CoreError::InvalidState(
                "Timestamp envelope di masa depan".to_string(),
            ));
        }
        if envelope.ciphertext.is_empty()
            || envelope.ciphertext.len() > MAX_MESSAGE_BYTES + 16 + 64
        {
            return Err(CoreError::InvalidState(
                "Ciphertext di luar batas".to_string(),
            ));
        }

        // 1. Ambil public key pengirim
        let contact = self
            .storage
            .get_contact(&sender_id)?
            .ok_or(CoreError::PeerNotFound(sender_id))?;
        if contact.public_key == [0u8; 32] {
            return Err(CoreError::MissingPublicKey(sender_id));
        }

        let peer_pub = IdentityPublicKey::from_bytes(contact.public_key);

        // 2. Derivasi kunci room
        let priv_key = self.identity_priv.read().await;
        let room_key = derive_room_key(&priv_key, &peer_pub, &envelope.room_id.to_string())?;

        // 3. Dekripsi ciphertext dengan verifikasi AAD (fail-closed UTF-8)
        let aad = build_message_aad(
            &envelope.room_id,
            &envelope.sender_id,
            &envelope.recipient_id,
            &envelope.message_id,
            envelope.timestamp_ms,
        );
        let decrypted_bytes =
            decrypt_message_with_aad(&room_key, &envelope.nonce, &envelope.ciphertext, &aad)?;
        let decrypted_text = String::from_utf8(decrypted_bytes).map_err(|_| {
            CoreError::InvalidState("Plaintext hasil dekripsi bukan UTF-8 valid".to_string())
        })?;

        // 4. Simpan ke database lokal
        let local_msg = LocalMessage {
            id: envelope.message_id,
            room_id: envelope.room_id,
            sender_id: envelope.sender_id,
            recipient_id: self.my_user_id,
            content: decrypted_text,
            media_url: None,
            msg_type: envelope.msg_type,
            is_view_once: envelope.is_view_once,
            is_opened: false,
            opened_at: None,
            reply_to_id: None,
            is_deleted: false,
            is_silent_deleted: false,
            auto_delete_at: envelope.expires_at,
            edited_at: None,
            reactions: HashMap::new(),
            is_encrypted: true,
            created_at: envelope.timestamp_ms,
            updated_at: now,
            status: DeliveryStatus::Delivered,
        };

        self.storage.insert_message(&local_msg)?;

        // 5. Emit event ke UI
        self.event_bus
            .emit(CoreEvent::MessageReceived(local_msg.clone()));

        Ok(local_msg)
    }

    /// Memperbarui status pesan lokal menjadi Delivered.
    pub async fn mark_delivered(&self, message_id: &Uuid) -> Result<(), CoreError> {
        self.storage
            .update_message_status(message_id, DeliveryStatus::Delivered)?;

        self.event_bus
            .emit(CoreEvent::MessageStatusUpdated {
                message_id: *message_id,
                status: DeliveryStatus::Delivered,
            });

        Ok(())
    }

    /// Memperbarui status pesan lokal menjadi Read.
    pub async fn mark_read(&self, message_id: &Uuid) -> Result<(), CoreError> {
        self.storage
            .update_message_status(message_id, DeliveryStatus::Read)?;

        self.event_bus
            .emit(CoreEvent::MessageStatusUpdated {
                message_id: *message_id,
                status: DeliveryStatus::Read,
            });

        Ok(())
    }
}
