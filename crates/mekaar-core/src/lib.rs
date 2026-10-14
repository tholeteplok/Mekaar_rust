//! # mekaar-core — Core Engine & Logic State Machines
//!
//! Logika bisnis inti dan mesin state untuk MEKAAR:
//! - **Chat Engine:** Manajemen siklus pengiriman, penerimaan, dan enkripsi E2EE per-room.
//! - **Disappearing Messages Worker:** Background worker otomatis pembersih pesan kedaluwarsa.
//! - **Guardian State Machine:** Kontrol hak akses berbasis kebutuhan (Zero-Surveillance) dan Dynamic Role Swapping.
//! - **SOS Panic Engine:** Siklus panik terstruktur (Idle → Armed → Broadcasting → Resolved) dengan Inactivity Watchdog.
//! - **Private Vault Cloaking:** Verifikasi PIN Master, deteksi Duress PIN (silent SOS), dan penyembunyian chat rahasia.
//! - **Reactive Event Bus:** Penyiaran event asinkron ke antarmuka klien.

pub mod chat;
pub mod disappearing;
pub mod error;
pub mod events;
pub mod guardian;
pub mod sos;
pub mod vault;

pub use chat::ChatEngine;
pub use disappearing::{spawn_disappearing_worker, DisappearingWorkerHandle};
pub use error::CoreError;
pub use events::{CoreEvent, EventBus};
pub use guardian::{GuardianManager, GuardianRole, GuardianState};
pub use sos::{SosEndReason, SosPanicEngine, SosState};
pub use vault::{AuthOutcome, VaultManager};
