//! # mekaar-core: Guardian Network & Safety Route State Machine
//!
//! Mengimplementasikan prinsip **Zero-Surveillance** (Anti-Stalkerware):
//! - Guardian TIDAK memiliki izin sensor (GPS/Mikrofon/Kamera) saat kondisi normal (`Dormant`).
//! - Izin hanya terbuka saat rute perjalanan sementara diaktifkan (`HangoutSharing`).
//! - Atau saat situasi panik darurat dipicu (`EmergencyBroadcast`).
//! - Mendukung **Dynamic Role Swapping** (tukar peran saling mengawasi simetris).

use crate::events::{CoreEvent, EventBus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Status kontrol hak akses Guardian.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuardianState {
    /// Kondisi normal: Tidak ada izin lokasi atau audio/kamera yang dapat diakses.
    Dormant,

    /// Izin sementara aktif terjadwal (Safety Route / Hangout mode).
    HangoutSharing {
        expires_at: u64,
        token: String,
    },

    /// Tombol SOS terpicu: Siaran darurat real-time dibuka penuh.
    EmergencyBroadcast,
}

/// Peran dalam relasi Guardian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuardianRole {
    /// Pemilik akun yang dilindungi.
    Owner,
    /// Pihak pelindung/pengawas.
    Guardian,
}

/// State machine pengelola status Guardian.
#[derive(Clone)]
pub struct GuardianManager {
    state: Arc<RwLock<GuardianState>>,
    role: Arc<RwLock<GuardianRole>>,
    event_bus: EventBus,
}

impl GuardianManager {
    /// Inisialisasi manager dengan status awal Dormant (Zero-Surveillance).
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            state: Arc::new(RwLock::new(GuardianState::Dormant)),
            role: Arc::new(RwLock::new(GuardianRole::Owner)),
            event_bus,
        }
    }

    /// Mendapatkan status perizinan saat ini.
    pub async fn current_state(&self) -> GuardianState {
        self.state.read().await.clone()
    }

    /// Mendapatkan peran saat ini (Owner vs Guardian).
    pub async fn current_role(&self) -> GuardianRole {
        *self.role.read().await
    }

    /// Mengaktifkan mode Safety Route / Hangout untuk durasi tertentu.
    pub async fn start_hangout(&self, duration_ms: u64, token: String) -> GuardianState {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let new_state = GuardianState::HangoutSharing {
            expires_at: now + duration_ms,
            token,
        };

        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
        }

        self.event_bus
            .emit(CoreEvent::GuardianStateChanged(new_state.clone()));
        new_state
    }

    /// Mengakhiri mode Safety Route dan kembali ke status Dormant.
    pub async fn end_hangout(&self) -> GuardianState {
        let new_state = GuardianState::Dormant;
        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
        }

        self.event_bus
            .emit(CoreEvent::GuardianStateChanged(new_state.clone()));
        new_state
    }

    /// Mengaktifkan siaran darurat penuh saat SOS dipicu.
    pub async fn trigger_emergency(&self) -> GuardianState {
        let new_state = GuardianState::EmergencyBroadcast;
        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
        }

        self.event_bus
            .emit(CoreEvent::GuardianStateChanged(new_state.clone()));
        new_state
    }

    /// Menyelesaikan situasi darurat dan mengunci kembali sensor ke Dormant.
    pub async fn resolve_emergency(&self) -> GuardianState {
        let new_state = GuardianState::Dormant;
        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
        }

        self.event_bus
            .emit(CoreEvent::GuardianStateChanged(new_state.clone()));
        new_state
    }

    /// Dynamic Role Swapping: Menukar peran antara Owner dan Guardian secara simetris.
    pub async fn swap_role(&self) -> GuardianRole {
        let mut role = self.role.write().await;
        *role = match *role {
            GuardianRole::Owner => GuardianRole::Guardian,
            GuardianRole::Guardian => GuardianRole::Owner,
        };
        *role
    }
}
