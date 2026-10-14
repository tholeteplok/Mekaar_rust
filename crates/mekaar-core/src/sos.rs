//! # mekaar-core: SOS Panic Response Engine
//!
//! Mengendalikan siklus hidup protokol darurat SOS:
//! - Siklus: Idle → Armed (hold-to-confirm 2s) → Broadcasting (GPS/WebRTC) → Acknowledged → Resolved
//! - Inactivity Watchdog: Sensor diam selama 2 menit otomatis menutup siaran.
//! - Silent Panic: Pemicuan via Duress PIN (PIN Paksaan).

use crate::events::{CoreEvent, EventBus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Alasan sesi darurat SOS ditutup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SosEndReason {
    /// Ditutup manual oleh pemilik perangkat.
    Manual,
    /// Timeout timer darurat otomatis.
    Timer,
    /// Inactivity Watchdog: Tidak ada interaksi atau gerakan sensor selama batas waktu (2 menit).
    Inactivity,
}

/// Status darurat SOS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SosState {
    /// Kondisi normal tanpa darurat.
    Idle,

    /// Tombol fisik/UI 72px sedang ditekan tahan (progress 0.0 s.d 1.0).
    Armed { progress: f32 },

    /// SOS Aktif: Siaran GPS live, audio, dan WebRTC video terbuka ke Guardian.
    Broadcasting {
        started_at: u64,
        gps_enabled: bool,
        mic_enabled: bool,
        video_enabled: bool,
    },

    /// Sinyal telah diterima dan direspons oleh Guardian sah.
    Acknowledged { guardian_id: Uuid },

    /// Situasi darurat telah selesai.
    Resolved {
        ended_at: u64,
        reason: SosEndReason,
    },
}

/// Engine pengendali status panik darurat SOS.
#[derive(Clone)]
pub struct SosPanicEngine {
    state: Arc<RwLock<SosState>>,
    last_interaction: Arc<RwLock<u64>>,
    event_bus: EventBus,
}

impl SosPanicEngine {
    /// Inisialisasi SOS Panic Engine dengan status awal Idle.
    pub fn new(event_bus: EventBus) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            state: Arc::new(RwLock::new(SosState::Idle)),
            last_interaction: Arc::new(RwLock::new(now)),
            event_bus,
        }
    }

    /// Mendapatkan status darurat saat ini.
    pub async fn current_state(&self) -> SosState {
        self.state.read().await.clone()
    }

    /// Mencatat interaksi sentuhan atau sensor akselerometer (mereset inactivity watchdog).
    pub async fn record_interaction(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let mut last = self.last_interaction.write().await;
        *last = now;
    }

    /// Memperbarui progres hold-to-confirm tombol SOS (0.0 s.d 1.0).
    pub async fn arm_progress(&self, progress: f32) -> SosState {
        let clamped = progress.clamp(0.0, 1.0);
        let new_state = if clamped >= 1.0 {
            // Cukup 2 detik tahan: otomatis trigger broadcasting
            self.trigger_panic(true, true, true).await
        } else if clamped > 0.0 {
            SosState::Armed { progress: clamped }
        } else {
            SosState::Idle
        };

        if let SosState::Armed { .. } = &new_state {
            let mut state = self.state.write().await;
            *state = new_state.clone();
            self.event_bus.emit(CoreEvent::SosStateChanged(new_state.clone()));
        }

        new_state
    }

    /// Memicu siaran darurat langsung (misal tombol ditahan 2 detik penuh).
    pub async fn trigger_panic(
        &self,
        gps_enabled: bool,
        mic_enabled: bool,
        video_enabled: bool,
    ) -> SosState {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let new_state = SosState::Broadcasting {
            started_at: now,
            gps_enabled,
            mic_enabled,
            video_enabled,
        };

        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
            let mut last = self.last_interaction.write().await;
            *last = now;
        }

        self.event_bus.emit(CoreEvent::SosStateChanged(new_state.clone()));
        new_state
    }

    /// Memicu silent SOS (misal input Duress PIN saat berada di bawah ancaman).
    pub async fn trigger_duress_silent(&self) -> SosState {
        self.trigger_panic(true, true, false).await
    }

    /// Guardian mengakui (acknowledge) sinyal darurat korban.
    pub async fn acknowledge(&self, guardian_id: Uuid) -> Result<SosState, String> {
        let mut state = self.state.write().await;
        if let SosState::Broadcasting { .. } = *state {
            let new_state = SosState::Acknowledged { guardian_id };
            *state = new_state.clone();
            self.event_bus.emit(CoreEvent::SosStateChanged(new_state.clone()));
            Ok(new_state)
        } else {
            Err("Hanya dapat mengakui saat sesi sedang Broadcasting".to_string())
        }
    }

    /// Menyelesaikan krisis darurat.
    pub async fn resolve(&self, reason: SosEndReason) -> SosState {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let new_state = SosState::Resolved {
            ended_at: now,
            reason,
        };

        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
        }

        self.event_bus.emit(CoreEvent::SosStateChanged(new_state.clone()));
        new_state
    }

    /// Mengecek apakah Inactivity Watchdog terpicu (tidak ada gerakan selama `timeout_ms`).
    pub async fn check_inactivity_watchdog(&self, timeout_ms: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let last = *self.last_interaction.read().await;
        let is_broadcasting = matches!(*self.state.read().await, SosState::Broadcasting { .. });

        if is_broadcasting && now >= last + timeout_ms {
            self.resolve(SosEndReason::Inactivity).await;
            true
        } else {
            false
        }
    }
}
