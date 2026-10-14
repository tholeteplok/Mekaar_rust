# DOKUMEN RANCANGAN ARSITEKTUR: MEKAAR (RUST REWRITE)

- **Dokumen:** Architectural Design Document (ADD)
- **Nama Proyek:** MEKAAR Native Engine & Suite
- **Versi:** 1.0.0-Draft
- **Target Bahasa:** Rust (Edition 2024 / 1.85+)
- **Paradigma:** Memory-Safe, Concurrency-Safe, Zero-Cost Abstraction, Fail-Closed Security

---

## 1. PENDAHULUAN & TUJUAN ARSITEKTUR

Proyek **MEKAAR** dirancang sebagai sistem perpesanan instan independen yang menempatkan **keselamatan personal (*personal-safety*)** dan **privasi mutlak (*privacy-first*)** sebagai prioritas tanpa kompromi. Membangun ulang sistem inti (*core engine*) menggunakan bahasa pemrograman **Rust** memberikan keunggulan teknis signifikan:

1. **Keamanan Memori Tanpa Garbage Collector (Memory Safety without GC):**
   - Menghilangkan celah kerentanan *buffer overflow*, *use-after-free*, dan *data race* saat memproses paket media maupun kunci kriptografi.
   - Pemanfaatan trait `zeroize::ZeroizeOnDrop` menjamin pembersihan memori kunci privat secara deterministik segera setelah tidak digunakan.
2. **Performa Real-time & Latensi Rendah:**
   - Tidak ada *jank* atau lonjakan latensi akibat *garbage collection pause*, sangat krusial untuk pemrosesan video dan audio darurat SOS WebRTC.
3. **Efisiensi Sumber Daya Ekstrem:**
   - Konsumsi RAM latar belakang (*background daemon*) di bawah 25 MB, memungkinkan berjalan stabil pada perangkat bersumber daya terbatas dan server ringan 24/7.
4. **Modul Portabel Multi-Platform (Single Core, Multi-Frontend):**
   - Satu pustaka inti (`mekaar-core`) dapat dikompilasi secara *cross-platform* ke Android, iOS, Linux, Windows, macOS, maupun WebAssembly (Wasm).

---

## 2. STRUKTUR WORKSPACE CARGO

Sistem dipecah ke dalam arsitektur Cargo Workspace multi-crate yang modular dan terisolasi:

```text
mekaar-rust/
├── Cargo.toml                  # Workspace manifest
├── crates/
│   ├── mekaar-crypto/          # Protokol Aegis E2EE (X25519, ChaCha20-Poly1305, HKDF, Zeroize)
│   ├── mekaar-proto/           # Schema pesan, serialisasi biner, format wire protocol
│   ├── mekaar-core/            # State machine percakapan, vault sesi, engine Guardian & SOS
│   ├── mekaar-storage/         # Database lokal terenkripsi (SQLCipher / Rusqlite / Sled)
│   ├── mekaar-net/             # Transport layer: WebRTC signaling, WebSocket, TLS, Push
│   ├── mekaar-server/          # Backend relay mandiri (Axum, PostgreSQL via SQLx, Redis)
│   └── mekaar-client-ffi/      # UniFFI / C-ABI bridge untuk integrasi antarmuka klien
└── apps/
    ├── mekaar-cli/             # Klien headless terminal (uji fungsional, simulasi daemon)
    └── mekaar-desktop/         # Klien native desktop (Tauri v2 / Slint)
```

---

## 3. SPESIFIKASI MODUL CRATE

### 3.1. `mekaar-crypto` (Aegis Protocol Core)
Bertanggung jawab atas seluruh operasi kriptografi tingkat tinggi dengan prinsip *fail-closed*.

* **Primitif Kriptografi:**
  * **Pertukaran Kunci Asimetris:** `x25519-dalek` (Curve25519 Diffie-Hellman).
  * **Enkripsi Simetris:** `chacha20poly1305` (XChaCha20-Poly1305 AEAD).
  * **Derivasi Kunci Sesi:** `hkdf` dipadukan dengan `sha2` (HKDF-SHA256).
  * **Pembersihan Memori:** `zeroize` (`#[derive(Zeroize, ZeroizeOnDrop)]`).
  * **Derivasi Master PIN:** `argon2` untuk enkripsi lokal *key backup* dan *private vault*.

* **Struktur Tipe Data Memori Aman:**
  ```rust
  use zeroize::{Zeroize, ZeroizeOnDrop};

  #[derive(Zeroize, ZeroizeOnDrop)]
  pub struct IdentityPrivateKey([u8; 32]);

  #[derive(Clone, Copy, PartialEq, Eq, Debug)]
  pub struct IdentityPublicKey([u8; 32]);

  #[derive(Zeroize, ZeroizeOnDrop)]
  pub struct RoomSessionKey([u8; 32]);

  #[derive(Debug, thiserror::Error)]
  pub enum CryptoError {
      #[error("Kunci lawan bicara belum terverifikasi atau rusak")]
      InvalidPeerKey,
      #[error("Gagal melakukan dekripsi (otentikasi MAC gagal)")]
      DecryptionFailed,
      #[error("Kegagalan derivasi kunci sesi")]
      KeyDerivationFailed,
  }
  ```

* **Kebijakan Keamanan Fail-Closed:**
  Operasi enkripsi/dekripsi selalu mengembalikan `Result<T, CryptoError>`. Sistem menolak transmisi paket dalam teks terbuka (*plaintext*) apabila negosiasi kunci gagal.

---

### 3.2. `mekaar-proto` (Wire Protocol & Serialisasi)
* Mendukung serialisasi efisien menggunakan **Postcard** atau **Protocol Buffers (`prost`)** guna menghemat kuota transmisi darurat hingga 70% dibanding format JSON.
* Tetap menyediakan serialisasi JSON untuk interoperabilitas database eksternal.
* Struktur Paket:
  ```rust
  use serde::{Deserialize, Serialize};
  use uuid::Uuid;

  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct MessageEnvelope {
      pub version: u8,
      pub sender_id: Uuid,
      pub recipient_id: Uuid,
      pub room_id: Uuid,
      pub nonce: [u8; 24],
      pub ciphertext: Vec<u8>,
      pub timestamp_ms: u64,
      pub is_view_once: bool,
      pub expires_at: Option<u64>,
  }
  ```

---

### 3.3. `mekaar-core` (Logic & State Machine)
* **Chat Engine:** Mengelola siklus pesan (*sent*, *delivered*, *read*), *disappearing message timer* berbasis *asynchronous worker* (`tokio::time::sleep`), dan mekanisme *offline queue*.
* **Guardian State Machine:**
  Menerapkan kontrol hak akses berbasis kondisi:
  ```rust
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub enum GuardianState {
      /// Kondisi normal: Tidak ada izin lokasi atau audio yang dapat diakses.
      Dormant,
      /// Izin sementara aktif terjadwal (perjalanan pulang / hangout).
      HangoutSharing { expires_at: u64 },
      /// Tombol SOS terpicu: Siaran darurat real-time dibuka penuh.
      EmergencyBroadcast,
  }
  ```
* **SOS Panic Engine:**
  Mengendalikan status darurat terstruktur:
  `Idle` $\rightarrow$ `Armed (Pulsing)` $\rightarrow$ `Broadcasting (GPS + WebRTC)` $\rightarrow$ `Acknowledged` $\rightarrow$ `Resolved`.

---

### 3.4. `mekaar-storage` (Database Lokal Terenkripsi)
* Menggantikan penyimpanan lokal bawaan platform dengan pustaka **`rusqlite`** yang dikompilasi bersama ekstensi **SQLCipher** (AES-256-CBC full-database encryption).
* Data yang disimpan:
  * Riwayat pesan E2EE lokal.
  * Kunci privat identitas yang di-wrap oleh hash Argon2 dari PIN 6 digit.
  * Antrean pesan keluar saat offline (*offline outbox*).
  * Cache media terenkripsi.

---

### 3.5. `mekaar-net` (Transport & Media Streaming)
* **Signaling Transport:** Koneksi WebSocket asinkron menggunakan `tokio-tungstenite`.
* **Mesin WebRTC:**
  * Memanfaatkan **`webrtc-rs`** (implementasi WebRTC murni Rust tanpa *wrapper* C++).
  * Menangani resolusi ICE, STUN, dan *TURN traversal* otomatis.
  * Jalur audio/video: integrasi *codec* Opus dan H.264 packetizer untuk transmisi video darurat berkecepatan tinggi.

---

### 3.6. `mekaar-server` (Backend Microservices Mandiri)
Opsi backend mandiri selain Supabase:
* **Web Framework:** `Axum` berbasis ekosistem `Tokio` dan `Hyper`.
* **Database Driver:** `SQLx` untuk PostgreSQL (dengan verifikasi query waktu kompilasi).
* **Pub/Sub Realtime:** Redis / Valkey via `fred-rs`.
* **Prinsip Zero-Knowledge:** Server hanya berfungsi sebagai perute (*relay*) buta tanpa akses terhadap kunci enkripsi maupun isi percakapan.

---

## 4. INTEGRASI ANTARMUKA (FRONTEND)

Terdapat dua jalur pendekatan implementasi UI:

### Opsi 1: Hybrid Core (Rekomendasi untuk Ekosistem Mobile)
* **Core & Kripto:** Pustaka Rust murni (`crates/mekaar-core`).
* **Frontend:** Flutter atau Native (Swift/Kotlin) yang dihubungkan melalui **`flutter_rust_bridge` (FRB v2)** atau **UniFFI**.
* **Keuntungan:** Tetap mempertahankan arsitektur desain visual *Tri-Register* (Clean Core, Colorful Room, Focused SOS) yang telah teruji, sementara seluruh beban komputasi kritis, kriptografi, dan penyimpanan diamankan oleh Rust.

### Opsi 2: Full-Rust Native App
* **Tauri v2:** Kerangka kerja lintas platform (Desktop & Mobile) dengan backend Rust dan antarmuka berbasis webview native (Svelte/React/HTML5). Ukuran instalasi sangat ringkas (<10 MB).
* **Slint:** Kerangka kerja GUI deklaratif native murni yang sangat hemat memori (RAM < 15 MB).

---

## 5. TAHAPAN IMPLEMENTASI (ROADMAP)

| Fase | Fokus Pekerjaan | Deliverable |
|---|---|---|
| **Fase 1** | Kriptografi & Protokol | Crate `mekaar-crypto` & `mekaar-proto` dengan pengujian vektor uji (KAT). |
| **Fase 2** | Penyimpanan Terenkripsi | Crate `mekaar-storage` (SQLCipher) dan state machine `mekaar-core`. |
| **Fase 3** | Jaringan & Streaming | Crate `mekaar-net` (signaling WebSocket & pipeline WebRTC). |
| **Fase 4** | Uji Coba Headless (PoC) | Binary `mekaar-cli` untuk komunikasi E2EE antar dua terminal. |
| **Fase 5** | FFI & Antarmuka | Binding FFI dan penyambungan ke antarmuka aplikasi. |

---

*Dokumen ini diterbitkan sebagai cetak biru resmi untuk perancangan ulang sistem MEKAAR ke dalam ekosistem Rust.*
