//! # mekaar-cli: Command-Line Argument Parser
//!
//! Definisi subcommands dan opsi CLI menggunakan `clap`.

use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "mekaar-cli",
    version,
    about = "🦀 MEKAAR Headless Diagnostic & Testing Tool",
    long_about = "Binary diagnostik headless untuk simulasi perpesanan E2EE, pengujian transport layer, dan verifikasi panic SOS tanpa antarmuka grafis."
)]
pub struct Cli {
    /// URL WebSocket Relay Server (contoh: ws://127.0.0.1:8085/ws atau ws://100.101.96.120:8085/ws)
    #[arg(long, global = true, default_value = "ws://127.0.0.1:8085/ws")]
    pub server_url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Menjalankan daemon headless latar belakang (mendengarkan pesan masuk & auto-ack)
    Daemon {
        /// ID pengguna klien (jika tidak dispesifikkan, UUID acak akan dibuat)
        #[arg(long)]
        user_id: Option<Uuid>,

        /// PIN proteksi master vault lokal
        #[arg(long, default_value = "123456")]
        pin: String,

        /// Path file database SQLCipher lokal (default: in-memory / tempfile)
        #[arg(long)]
        db_path: Option<String>,
    },

    /// Memulai sesi obrolan interaktif langsung (REPL E2EE) antar terminal
    Chat {
        /// ID pengguna Anda (default: acak atau identitas tersimpan)
        #[arg(long)]
        as_user: Option<Uuid>,

        /// ID pengguna lawan bicara target
        #[arg(long)]
        peer_id: Uuid,

        /// PIN proteksi master vault lokal
        #[arg(long, default_value = "123456")]
        pin: String,

        /// Path file database SQLCipher lokal
        #[arg(long)]
        db_path: Option<String>,
    },

    /// Mengirimkan satu pesan terenkripsi langsung ke lawan bicara
    Send {
        /// ID pengirim (default: UUID acak)
        #[arg(long)]
        from: Option<Uuid>,

        /// ID penerima
        #[arg(long)]
        to: Uuid,

        /// Isi pesan plaintext yang akan dienkripsi
        #[arg(long)]
        message: String,

        /// Tandai sebagai pesan sekali lihat (View-Once)
        #[arg(long, default_value_t = false)]
        view_once: bool,

        /// PIN proteksi master vault lokal
        #[arg(long, default_value = "123456")]
        pin: String,

        /// Path file database SQLCipher lokal
        #[arg(long)]
        db_path: Option<String>,
    },

    /// Memicu sinyal darurat SOS darurat tinggi ke seluruh Guardian aktif
    Sos {
        /// ID pengguna korban (default: UUID acak)
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Titik garis lintang GPS darurat (latitude)
        #[arg(long, default_value_t = -6.2088)]
        lat: f64,

        /// Titik garis bujur GPS darurat (longitude)
        #[arg(long, default_value_t = 106.8456)]
        lon: f64,

        /// Akurasi koordinat (dalam meter)
        #[arg(long, default_value_t = 4.5)]
        accuracy: f32,
    },

    /// Menjalankan simulasi otomatis penuh E2E (Alice <-> Bob via Server relay)
    SimulatePair {
        /// URL relay server eksternal (jika tidak dispesifikkan, server lokal in-process akan dijalankan otomatis)
        #[arg(long)]
        custom_server: Option<String>,
    },
}
