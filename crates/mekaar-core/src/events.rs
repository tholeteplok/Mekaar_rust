//! # mekaar-core: Reactive Event Bus
//!
//! Saluran penyiar event asinkron internal untuk komunikasi antara core engine
//! dan antarmuka UI (Tauri v2 IPC / CLI).

use crate::guardian::GuardianState;
use crate::sos::SosState;
use mekaar_storage::{DeliveryStatus, LocalMessage};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Seluruh variasi kejadian sistem yang dapat dipancarkan ke antarmuka pengguna.
#[derive(Clone, Debug, PartialEq)]
pub enum CoreEvent {
    /// Pesan baru diterima dan telah didekripsi ke database lokal.
    MessageReceived(LocalMessage),

    /// Status pengiriman pesan lokal diperbarui.
    MessageStatusUpdated {
        message_id: Uuid,
        status: DeliveryStatus,
    },

    /// Pekerja pesan menghilang telah membersihkan sejumlah pesan kedaluwarsa.
    DisappearingMessagesPurged {
        count: usize,
    },

    /// Transisi status darurat SOS.
    SosStateChanged(SosState),

    /// Transisi status perizinan Guardian (Zero-Surveillance).
    GuardianStateChanged(GuardianState),

    /// Ruang rahasia Private Vault berhasil dibuka (atau Duress terdeteksi).
    VaultUnlocked {
        is_duress: bool,
    },
}

/// Event bus berbasis `tokio::sync::broadcast` yang aman digunakan multi-thread.
#[derive(Clone)]
pub struct EventBus {
    sender: Arc<broadcast::Sender<CoreEvent>>,
}

impl EventBus {
    /// Membuat event bus baru dengan kapasitas antrean tertentu (default 100).
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender: Arc::new(sender),
        }
    }

    /// Memancarkan event ke seluruh subscriber yang mendengarkan.
    pub fn emit(&self, event: CoreEvent) {
        // Abaikan error jika saat ini belum ada listener aktif
        let _ = self.sender.send(event);
    }

    /// Membuat listener receiver baru untuk menerima aliran event.
    pub fn subscribe(&self) -> broadcast::Receiver<CoreEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(128)
    }
}
