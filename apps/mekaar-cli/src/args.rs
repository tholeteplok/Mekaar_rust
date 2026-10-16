//! # mekaar-cli: Command-Line Argument Parser
//!
//! Definisi subcommands dan opsi CLI menggunakan `clap`.

use clap::{Parser, Subcommand};
use uuid::Uuid;

fn validate_ws_url(s: &str) -> Result<String, String> {
    if s.len() > 256 {
        return Err("server_url terlalu panjang".to_string());
    }
    // Izinkan ws loopback (dev) atau wss (prod). Tolak file://, http://evil, dll.
    if s.starts_with("wss://") {
        return Ok(s.to_string());
    }
    if s.starts_with("ws://127.0.0.1") || s.starts_with("ws://localhost") {
        return Ok(s.to_string());
    }
    Err("server_url harus wss:// atau ws:// loopback".to_string())
}

fn validate_pin(s: &str) -> Result<String, String> {
    if s.len() >= 6 && s.chars().all(|c| c.is_ascii_digit()) {
        Ok(s.to_string())
    } else {
        Err("PIN harus minimal 6 digit numerik".to_string())
    }
}

fn validate_lat(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| "lat tidak valid".to_string())?;
    if v.is_finite() && (-90.0..=90.0).contains(&v) {
        Ok(v)
    } else {
        Err("lat harus -90..90".to_string())
    }
}

fn validate_lon(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| "lon tidak valid".to_string())?;
    if v.is_finite() && (-180.0..=180.0).contains(&v) {
        Ok(v)
    } else {
        Err("lon harus -180..180".to_string())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "mekaar-cli",
    version,
    about = "🦀 MEKAAR Headless Diagnostic & Testing Tool",
    long_about = "Binary diagnostik headless untuk simulasi perpesanan E2EE, pengujian transport layer, dan verifikasi panic SOS tanpa antarmuka grafis."
)]
pub struct Cli {
    /// URL WebSocket Relay Server (contoh: ws://127.0.0.1:8085/ws atau ws://100.101.96.120:8085/ws)
    #[arg(long, global = true, default_value = "ws://127.0.0.1:8085/ws", value_parser = validate_ws_url)]
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

        /// PIN proteksi master vault lokal (wajib, min 6 digit; tanpa default backdoor)
        #[arg(long, value_parser = validate_pin)]
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

        /// PIN proteksi master vault lokal (wajib, min 6 digit)
        #[arg(long, value_parser = validate_pin)]
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

        /// PIN proteksi master vault lokal (wajib, min 6 digit)
        #[arg(long, value_parser = validate_pin)]
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
        #[arg(long, default_value_t = -6.2088, value_parser = validate_lat)]
        lat: f64,

        /// Titik garis bujur GPS darurat (longitude)
        #[arg(long, default_value_t = 106.8456, value_parser = validate_lon)]
        lon: f64,

        /// Akurasi koordinat (dalam meter)
        #[arg(long, default_value_t = 4.5)]
        accuracy: f32,
    },

    /// Menjalankan simulasi otomatis penuh E2E (Alice <-> Bob via Server relay)
    SimulatePair {
        /// URL relay server eksternal (jika tidak dispesifikkan, server lokal in-process akan dijalankan otomatis)
        #[arg(long, value_parser = validate_ws_url)]
        custom_server: Option<String>,
    },
}
