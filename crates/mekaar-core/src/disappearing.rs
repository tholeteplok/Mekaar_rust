//! # mekaar-core: Disappearing Message Background Worker
//!
//! Task asinkron latar belakang (`tokio::task`) yang berjalan secara berkala
//! untuk menghapus pesan kedaluwarsa secara permanen dari database lokal terenkripsi.

use crate::events::{CoreEvent, EventBus};
use mekaar_storage::EncryptedDatabase;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

/// Handle untuk mengontrol siklus hidup background worker pesan menghilang.
pub struct DisappearingWorkerHandle {
    is_running: Arc<AtomicBool>,
}

impl DisappearingWorkerHandle {
    /// Menghentikan worker latar belakang.
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}

/// Menjalankan background worker untuk pembersihan otomatis pesan disappearing.
pub fn spawn_disappearing_worker(
    storage: EncryptedDatabase,
    event_bus: EventBus,
    check_interval: Duration,
) -> DisappearingWorkerHandle {
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = is_running.clone();

    tokio::spawn(async move {
        info!("Pekerja background disappearing messages aktif (Interval: {:?})", check_interval);

        while is_running_clone.load(Ordering::SeqCst) {
            tokio::time::sleep(check_interval).await;

            if !is_running_clone.load(Ordering::SeqCst) {
                break;
            }

            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            match storage.purge_expired_messages(now) {
                Ok(count) => {
                    if count > 0 {
                        debug!("Disappearing worker: Berhasil membersihkan {} pesan kedaluwarsa", count);
                        event_bus.emit(CoreEvent::DisappearingMessagesPurged { count });
                    }
                }
                Err(e) => {
                    error!("Gagal membersihkan pesan kedaluwarsa: {}", e);
                }
            }
        }

        info!("Pekerja background disappearing messages telah berhenti");
    });

    DisappearingWorkerHandle { is_running }
}
