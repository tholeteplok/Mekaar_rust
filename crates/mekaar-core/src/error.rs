//! # mekaar-core: Error Types
//!
//! Error yang mungkin terjadi dalam alur logika bisnis dan state machine MEKAAR.

use mekaar_crypto::CryptoError;
use mekaar_proto::ProtoError;
use mekaar_storage::StorageError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Kriptografi error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("Penyimpanan lokal error: {0}")]
    Storage(#[from] StorageError),

    #[error("Protokol wire error: {0}")]
    Proto(#[from] ProtoError),

    #[error("Kontak atau lawan bicara {0} tidak ditemukan")]
    PeerNotFound(Uuid),

    #[error("Kunci publik untuk lawan bicara {0} belum tersedia")]
    MissingPublicKey(Uuid),

    #[error("Ruang obrolan {0} tidak ditemukan")]
    RoomNotFound(Uuid),

    #[error("Operasi tidak valid pada status saat ini: {0}")]
    InvalidState(String),

    #[error("Private Vault terkunci: Masukkan PIN untuk membuka obrolan rahasia")]
    VaultLocked,

    #[error("Duress PIN terdeteksi: Membuka mode aman dan memicu silent SOS")]
    DuressDetected,

    #[error("Akun terkunci sementara karena {0}x kesalahan PIN berturut-turut. Tunggu hingga {1}")]
    PinLocked(u32, u64),
}
