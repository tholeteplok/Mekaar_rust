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
    decrypt_message, derive_room_key, encrypt_message, IdentityPrivateKey, IdentityPublicKey,
};
use mekaar_proto::{MessageEnvelope, MessageType};
use mekaar_storage::{DeliveryStatus, EncryptedDatabase, LocalMessage};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

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
        // 1. Ambil public key penerima
        let contact = self
            .storage
            .get_contact(&recipient_id)?
            .ok_or(CoreError::PeerNotFound(recipient_id))?;

        let peer_pub = IdentityPublicKey::from_bytes(contact.public_key);

        // 2. Derivasi kunci room
        let priv_key = self.identity_priv.read().await;
        let room_key = derive_room_key(&priv_key, &peer_pub, &room_id.to_string())?;

        // 3. Enkripsi konten plaintext
        let (nonce, ciphertext) = encrypt_message(&room_key, content.as_bytes())?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let auto_delete_at = disappearing_duration_ms.map(|d| now + d);
        let message_id = Uuid::new_v4();

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

        // 1. Ambil public key pengirim
        let contact = self
            .storage
            .get_contact(&sender_id)?
            .ok_or(CoreError::PeerNotFound(sender_id))?;

        let peer_pub = IdentityPublicKey::from_bytes(contact.public_key);

        // 2. Derivasi kunci room
        let priv_key = self.identity_priv.read().await;
        let room_key = derive_room_key(&priv_key, &peer_pub, &envelope.room_id.to_string())?;

        // 3. Dekripsi ciphertext
        let decrypted_bytes = decrypt_message(&room_key, &envelope.nonce, &envelope.ciphertext)?;
        let decrypted_text = String::from_utf8_lossy(&decrypted_bytes).to_string();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

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
