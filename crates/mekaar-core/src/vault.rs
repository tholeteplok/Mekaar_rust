//! # mekaar-core: Private Vault & Duress PIN Manager
//!
//! Pengelola keamanan akses lokal:
//! - Verifikasi Master PIN (Argon2id).
//! - Deteksi **Duress PIN (PIN Paksaan)**: membuka tampilan normal tetapi memicu silent SOS di latar belakang.
//! - Proteksi brute force: 5x gagal berturut-turut mengunci verifikasi selama 30 menit.
//! - Private Vault Cloaking: menyembunyikan chat rahasia dari daftar room utama.

use crate::error::CoreError;
use mekaar_crypto::derive_pin_key;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroize;

/// Perbandingan constant-time untuk hash PIN.
fn ct_eq_bytes(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Hasil verifikasi PIN akses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthOutcome {
    /// PIN Benar: Aplikasi / Vault terbuka normal.
    Unlocked,
    /// Duress PIN Terdeteksi: Aplikasi tampak terbuka, namun silent SOS seketika dipicu.
    Duress,
}

/// Pengelola keamanan Vault & PIN lokal.
#[derive(Clone)]
pub struct VaultManager {
    master_hash: Arc<RwLock<Option<[u8; 32]>>>,
    duress_hash: Arc<RwLock<Option<[u8; 32]>>>,
    salt: [u8; 16],
    failed_attempts: Arc<RwLock<u32>>,
    locked_until: Arc<RwLock<u64>>,
    is_vault_unlocked: Arc<RwLock<bool>>,
}

impl VaultManager {
    /// Inisialisasi VaultManager dengan salt 16-byte konstan per-perangkat.
    pub fn new(salt: [u8; 16]) -> Self {
        Self {
            master_hash: Arc::new(RwLock::new(None)),
            duress_hash: Arc::new(RwLock::new(None)),
            salt,
            failed_attempts: Arc::new(RwLock::new(0)),
            locked_until: Arc::new(RwLock::new(0)),
            is_vault_unlocked: Arc::new(RwLock::new(false)),
        }
    }

    /// Menetapkan Master PIN baru (diderivasi via Argon2id).
    pub async fn set_master_pin(&self, pin: &str) -> Result<(), CoreError> {
        // Tolak jika sama dengan duress aktif (hindari silent-SOS tiap login benar).
        if let Some(duress) = *self.duress_hash.read().await {
            let candidate = derive_pin_key(pin, &self.salt)?;
            let equal = ct_eq_bytes(&candidate, &duress);
            let mut tmp = candidate;
            tmp.zeroize();
            if equal {
                return Err(CoreError::InvalidState(
                    "Master PIN tidak boleh sama dengan Duress PIN".to_string(),
                ));
            }
        }
        let key = derive_pin_key(pin, &self.salt)?;
        let mut master = self.master_hash.write().await;
        *master = Some(key);
        Ok(())
    }

    /// Menetapkan Duress PIN baru (PIN Paksaan).
    pub async fn set_duress_pin(&self, pin: &str) -> Result<(), CoreError> {
        if let Some(master) = *self.master_hash.read().await {
            let candidate = derive_pin_key(pin, &self.salt)?;
            let equal = ct_eq_bytes(&candidate, &master);
            let mut tmp = candidate;
            tmp.zeroize();
            if equal {
                return Err(CoreError::InvalidState(
                    "Duress PIN tidak boleh sama dengan Master PIN".to_string(),
                ));
            }
        }
        let key = derive_pin_key(pin, &self.salt)?;
        let mut duress = self.duress_hash.write().await;
        *duress = Some(key);
        Ok(())
    }

    /// Status apakah ruang rahasia Private Vault saat ini sedang terbuka.
    pub async fn is_vault_unlocked(&self) -> bool {
        *self.is_vault_unlocked.read().await
    }

    /// Mengunci kembali seluruh ruang rahasia Private Vault.
    pub async fn lock_vault(&self) {
        let mut unlocked = self.is_vault_unlocked.write().await;
        *unlocked = false;
    }

    /// Verifikasi input PIN pengguna dengan proteksi lockout 30 menit setelah 5x gagal.
    pub async fn verify_pin(&self, input_pin: &str) -> Result<AuthOutcome, CoreError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // 1. Cek status lockout
        let locked_until = *self.locked_until.read().await;
        if now < locked_until {
            let attempts = *self.failed_attempts.read().await;
            return Err(CoreError::PinLocked(attempts, locked_until));
        }

        let mut input_hash = derive_pin_key(input_pin, &self.salt)?;

        // 2. Cek apakah cocok dengan Duress PIN (constant-time)
        if let Some(duress_key) = *self.duress_hash.read().await {
            if ct_eq_bytes(&input_hash, &duress_key) {
                // Reset failed attempts
                *self.failed_attempts.write().await = 0;
                *self.is_vault_unlocked.write().await = true;
                input_hash.zeroize();
                return Ok(AuthOutcome::Duress);
            }
        }

        // 3. Cek apakah cocok dengan Master PIN (constant-time)
        if let Some(master_key) = *self.master_hash.read().await {
            if ct_eq_bytes(&input_hash, &master_key) {
                *self.failed_attempts.write().await = 0;
                *self.is_vault_unlocked.write().await = true;
                input_hash.zeroize();
                return Ok(AuthOutcome::Unlocked);
            }
        }
        input_hash.zeroize();

        // 4. Input PIN salah: tingkatkan failed attempts
        let mut attempts = self.failed_attempts.write().await;
        *attempts += 1;

        if *attempts >= 5 {
            // Kunci selama 30 menit (1.800.000 ms)
            let lock_time = now + 1_800_000;
            *self.locked_until.write().await = lock_time;
            return Err(CoreError::PinLocked(*attempts, lock_time));
        }

        Err(CoreError::InvalidState(format!(
            "PIN tidak cocok (Percobaan {}/5)",
            *attempts
        )))
    }
}
