//! # mekaar-net: Reconnect Policy
//!
//! Kebijakan auto-reconnect dengan exponential backoff terikat waktu (capped exponential backoff).
//! Mereplikasi perilaku dari `reconnect_policy.dart` pada prototipe Flutter MEKAAR:
//! - Percobaan 1 → 1 detik
//! - Percobaan 2 → 2 detik
//! - Percobaan 3 → 4 detik
//! - Percobaan 4+ → cap 8 detik

use std::time::Duration;

/// Kebijakan reconnect untuk koneksi WebSocket realtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconnectPolicy {
    /// Maksimal percobaan sebelum menyerah. `None` berarti terus mencoba (untuk daemon).
    pub max_attempts: Option<u32>,
    /// Delay awal percobaan pertama (default: 1 detik).
    pub base_delay: Duration,
    /// Batas maksimal delay antar percobaan (default: 8 detik).
    pub max_delay: Duration,
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self {
            max_attempts: Some(5),
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(8),
        }
    }
}

impl ReconnectPolicy {
    /// Membuat kebijakan baru dengan parameter khusus.
    pub fn new(max_attempts: Option<u32>, base_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_attempts,
            base_delay,
            max_delay,
        }
    }

    /// Membuat kebijakan terus mencoba tanpa henti (cocok untuk background daemon).
    pub fn infinite() -> Self {
        Self {
            max_attempts: None,
            ..Default::default()
        }
    }

    /// Menghitung delay untuk percobaan ke-[attempt] (1-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let shift = attempt.saturating_sub(1);
        let factor = 1u64.checked_shl(shift).unwrap_or(u64::MAX);

        let delay_ms = (self.base_delay.as_millis() as u64).saturating_mul(factor);
        let max_ms = self.max_delay.as_millis() as u64;

        Duration::from_millis(delay_ms.min(max_ms))
    }

    /// Memeriksa apakah percobaan ke-[attempt] masih diizinkan.
    pub fn should_retry(&self, attempt: u32) -> bool {
        match self.max_attempts {
            Some(max) => attempt <= max,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff_delays() {
        let policy = ReconnectPolicy::default();

        assert_eq!(policy.delay_for_attempt(1), Duration::from_secs(1));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_secs(2));
        assert_eq!(policy.delay_for_attempt(3), Duration::from_secs(4));
        assert_eq!(policy.delay_for_attempt(4), Duration::from_secs(8));
        assert_eq!(policy.delay_for_attempt(5), Duration::from_secs(8));
        assert_eq!(policy.delay_for_attempt(100), Duration::from_secs(8));
    }

    #[test]
    fn test_retry_limits() {
        let policy = ReconnectPolicy::new(Some(3), Duration::from_secs(1), Duration::from_secs(8));
        assert!(policy.should_retry(1));
        assert!(policy.should_retry(2));
        assert!(policy.should_retry(3));
        assert!(!policy.should_retry(4));

        let infinite = ReconnectPolicy::infinite();
        assert!(infinite.should_retry(9999));
    }
}
