# PRODUCT REQUIREMENT DOCUMENT (PRD)
## MEKAAR Suite: Full Rust Architecture (Tauri v2 Client + Standalone Self-Hosted Relay Server)

- **Dokumen:** Product Requirement Document (PRD)
- **Nama Produk:** MEKAAR Suite (Full-Rust Ecosystem)
- **Versi Dokumen:** 2.0.0 (Updated: Full-Rust + Standalone Server Architecture)
- **Status:** Approved for Implementation
- **Strategi Mesin:**
  - **Development Machine (Laptop):** Build environment, coding UI Tauri v2, kompilasi binary client & server.
  - **Production Server (teplok-pc 24/7):** Host mandiri binary `mekaar-server` (Axum/Tokio) via private mesh Tailscale.
- **Target Klien:** Mobile (Android & iOS via Tauri v2 Mobile) & Desktop (Linux, macOS, Windows via Tauri v2 Desktop).

---

## 1. EXECUTIVE SUMMARY & ARSITEKTUR INFRASTRUKTUR

### 1.1 Visi Produk
Mentransformasi ekosistem **MEKAAR** menjadi **100% Rust** secara menyeluruh:
1. **Frontend / Klien:** Menggunakan **Tauri v2** (Rust core backend + Webview UI native ringan) untuk menghasilkan aplikasi seluler dan desktop yang hemat memori (<30 MB RAM), ukuran binary ringkas (<15 MB), dan bebas jeda Garbage Collection.
2. **Backend Server Mandiri (`mekaar-server`):** Menggantikan ketergantungan BaaS Supabase dengan microservice Rust mandiri berbasis **Axum & Tokio**. Server bertindak murni sebagai *Blind Relay* (Zero-Knowledge) yang berjalan 24/7 di **teplok-pc** melalui jaringan privat Tailscale.

### 1.2 Pembagian Peran Lingkungan (Dev vs Prod)

```text
┌─────────────────────────────────────────────────────────────┐
│                   LAPTOP PENGEMBANG (DEV)                   │
│  - Editor & Compiler (Rust toolchain, Node/Web tooling)     │
│  - Build Tauri v2 Client (Android APK, iOS, Desktop)       │
│  - Cross-compile binary release `mekaar-server` (x86_64)   │
└──────────────────────────────┬──────────────────────────────┘
                               │ Deploy via scp / Tailscale
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 TEPLOK-PC (24/7 SERVER MANDIRI)             │
│  - Host: Linux x86_64 (Intel Xeon 4 Core, RAM 3.8 GB)       │
│  - Port: Private Mesh Tailscale (e.g. 100.101.96.120:8085)  │
│  - Layanan: `mekaar-server.service` (Systemd User Service)  │
│  - Beban Kerja: 15-40 MB RAM (Tanpa beban build/compile)    │
│  - Database: SQLite lokal terindeks (WAL mode) / PostgreSQL │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. MODULAR CARGO WORKSPACE

```text
mekaar-suite/
├── Cargo.toml                  # Workspace manifest
├── crates/
│   ├── mekaar-crypto/          # Protokol Aegis (X25519, ChaCha20-Poly1305, HKDF, Zeroize)
│   ├── mekaar-proto/           # Schema pesan biner, wire protocol, serialisasi Postcard/JSON
│   ├── mekaar-core/            # State machine chat, vault lokal, engine Guardian & SOS
│   ├── mekaar-storage/         # SQLite lokal terenkripsi (SQLCipher / Rusqlite)
│   ├── mekaar-net/             # Transport layer: WebRTC signaling, WebSocket client, TLS
│   └── mekaar-server/          # Standalone Backend Relay (Axum, WebSocket Hub, SQLite/WAL)
└── apps/
    ├── mekaar-tauri/           # Klien Utama Tauri v2 (Android, iOS, Desktop)
    │   ├── src-tauri/          # Rust backend Tauri (interfacing dengan mekaar-core)
    │   └── ui/                 # Antarmuka Tri-Register (Svelte / HTML5 / Vanilla JS ringan)
    └── mekaar-cli/             # Headless diagnostic & testing tool
```

---

## 3. RUANG LINGKUP & FITUR PRODUK (PRODUCT FEATURES)

### 3.1 Pilar 1: Aegis Protocol E2EE (Kriptografi Klien)
* **FR-1.1 (Keypair Identitas Klien):** Kunci privat asimetris dibuat lokal via Curve25519 (`x25519-dalek`). Kunci privat dilindungi di RAM via `zeroize::ZeroizeOnDrop`.
* **FR-1.2 (Enkripsi Simetris End-to-End):** Cipher XChaCha20-Poly1305 mengenkripsi seluruh isi pesan, lokasi, dan metadata lampiran.
* **FR-1.3 (Enkripsi Media Terdistribusi):** File media dienkripsi dengan kunci acak per file sebelum diunggah; kunci dekripsi dikirimkan di dalam envelope pesan terenkripsi.
* **FR-1.4 (Prinsip Fail-Closed):** Jika derivasi kunci gagal atau terjadi kegagalan otentikasi MAC, sistem menolak pengiriman. Dilarang keras mengirim pesan dalam teks terbuka (*plaintext*).

### 3.2 Pilar 2: Fitur Anti-Jejak (Privacy Hardening)
* **FR-2.1 (View-Once Media RAM Buffer):** Media sekali lihat didekripsi murni di RAM sementara. Begitu antarmuka ditutup, buffer memori di-overwrite nol dan dihapus tanpa pernah menyentuh storage disk publik.
* **FR-2.2 (Disappearing Message Worker):** Background task asinkron (`tokio::time::sleep`) menghapus pesan lokal dari database SQLCipher saat masa berlaku habis (1 Jam, 1 Hari, 7 Hari).
* **FR-2.3 (Private Vault Cloaking):** Chat rahasia dilindungi oleh PIN 6 digit yang diderivasi via Argon2id. Chat tersembunyi sepenuhnya dari daftar obrolan utama tanpa tanda visual.

### 3.3 Pilar 3: Guardian Network & Safety Route
* **FR-3.1 (Akses Berbasis Kebutuhan / Zero-Surveillance):** Guardian **tidak memiliki akses** GPS atau mikrofon pada kondisi normal.
* **FR-3.2 (Safety Route / Hangout Mode):** Pengguna dapat membagikan telemetri rute kepulangan sementara dengan token terikat waktu (*time-bounded token*).
* **FR-3.3 (Dynamic Role Swapping):** Hubungan saling mengawasi dapat dialihkan dua arah secara simetris atas konfirmasi timbal balik.

### 3.4 Pilar 4: SOS Panic Response System
* **FR-4.1 (Pemicu Darurat):** Tombol fisik UI 72px (tahan 2 detik dengan haptic feedback) atau deteksi guncangan sensorik akselerometer darurat.
* **FR-4.2 (Transmisi Sinyal Prioritas):** Sinyal SOS dikirim langsung ke server relay dengan jalur prioritas tinggi (*high-priority WebSocket frame*), seketika menyiagakan seluruh Guardian aktif.
* **FR-4.3 (Live WebRTC Audio/Video Stream):** Membuka koneksi WebRTC P2P instan untuk menyiarkan feed kamera dan mikrofon langsung ke layar Guardian.
* **FR-4.4 (Live GPS Tracking):** Pembaruan titik koordinat presisi setiap 3–5 detik yang dipetakan pada OpenStreetMap.
* **FR-4.5 (Device Lost Protocol):** Perintah darurat dari Guardian terverifikasi untuk mengunci perangkat sasaran dari jarak jauh dan membunyikan alarm sirene desibel tinggi.

---

## 4. SPESIFIKASI SERVER MANDIRI (`mekaar-server` DI TEPLOK-PC)

Server mandiri di **teplok-pc** dirancang beroperasi tanpa intervensi manusia, dengan spesifikasi teknis berikut:

### 4.1 Tanggung Jawab & Arsitektur Zero-Knowledge
Server **tidak memiliki kunci dekripsi** pengguna dan bertindak sebagai *blind relay*:
1. **Public Key Directory:** Menyimpan `user_id` $\leftrightarrow$ `IdentityPublicKey` X25519.
2. **WebSocket Realtime Hub:** Meneruskan paket ciphertext terenkripsi antar-koneksi aktif secara instan.
3. **Offline Store-and-Forward Outbox:** Jika penerima offline, ciphertext disimpan sementara di database SQLite lokal server (WAL mode) hingga penerima online.
4. **WebRTC Signaling Relay:** Menfasilitasi pertukaran SDP offer/answer dan ICE candidate untuk inisiasi panggilan dan live stream SOS.
5. **Guardian Event Dispatcher:** Meneruskan sinyal notifikasi krisis ke perangkat Guardian.

### 4.2 Spesifikasi Sumber Daya & Layanan
* **Framework:** Axum (Hyper/Tokio) + `tokio-tungstenite`.
* **Database:** SQLite lokal dengan WAL mode (`PRAGMA journal_mode=WAL`) via `rusqlite` / `sqlx-sqlite` (ringan, tanpa perlu overhead daemon PostgreSQL berat).
* **Alamat Jaringan:** `http://100.101.96.120:8085` (Private Tailscale Mesh saja; tidak diekspos ke internet publik).
* **Systemd Service:** `~/.config/systemd/user/mekaar-server.service`.
* **Batas Konsumsi RAM:** $< 40$ MB saat beban aktif ribuan frame WebSocket.

---

## 5. SPESIFIKASI KLIEN TAURI V2

* **Arsitektur:**
  - **Backend Klien (Rust / `src-tauri`):** Menjalankan `mekaar-core`, `mekaar-crypto`, penyimpanan lokal SQLCipher, dan pipeline WebRTC.
  - **Frontend UI (Webview):** Menggunakan Svelte atau HTML5/CSS modern untuk mereplikasi arsitektur visual **Tri-Register**:
    1. *Clean Core 🤍:* Navigasi, pengaturan, daftar kontak, dan status Guardian.
    2. *Colorful Room 🎨:* Ruang obrolan dengan 11 preset tema ekspresif.
    3. *Focused SOS 🚨:* Kanvas merah pekat darurat tanpa ornamen bermain saat mode panik aktif.
* **Target Build (dari Laptop):**
  - Android APK / AAB (`cargo tauri android build`)
  - Linux binary (`cargo tauri build`)
  - Windows & macOS binary

---

## 6. PROJECT BENCHMARK & REFERENSI SERUPA

1. **[signalapp/libsignal (Rust)](https://github.com/signalapp/libsignal)**
   - *Arsitektur Kriptografi Resmi Signal Messenger.*
   - **Referensi untuk:** Manajemen state ratchet, memory safety kunci privat, dan abstraksi protokol E2EE.
2. **[matrix-org/matrix-rust-sdk](https://github.com/matrix-org/matrix-rust-sdk)**
   - *SDK resmi Matrix Protocol berbasis Rust.*
   - **Referensi untuk:** Arsitektur modular *crypto store*, event queue offline, dan sinkronisasi client-server tanpa jeda.
3. **[webrtc-rs/webrtc](https://github.com/webrtc-rs/webrtc)**
   - *Implementasi WebRTC murni Rust.*
   - **Referensi untuk:** Pembuatan audio/video pipeline darurat pada SOS tanpa dependensi C++ besar.
4. **[tauri-apps/tauri (v2.0)](https://github.com/tauri-apps/tauri)**
   - *Framework aplikasi multi-platform berbasis Rust.*
   - **Referensi untuk:** Build cross-platform seluler (Android/iOS) dan desktop dengan overhead memori minimal.
5. **[tokio-rs/axum](https://github.com/tokio-rs/axum)**
   - *Framework web mikro berkinerja tinggi berbasis Tokio & Tower.*
   - **Referensi untuk:** Implementasi server relay WebSocket berkecepatan tinggi dengan jejak RAM sangat kecil.

---

## 7. RENCANA KERJA & TAHAPAN IMPLEMENTASI (ROADMAP)

```text
[Fase 1: Kripto & Wire Protocol di Laptop]
    └── Implementasi mekaar-crypto (X25519, ChaCha20, Zeroize) & mekaar-proto (Postcard/JSON)
[Fase 2: Server Mandiri di Laptop & Deploy ke teplok-pc]
    └── Bikin mekaar-server (Axum, WebSocket Hub, SQLite WAL)
    └── Kompilasi binary release x86_64 -> kirim ke teplok-pc -> pasang systemd service
[Fase 3: Storage & Core Engine di Laptop]
    └── Bikin mekaar-storage (SQLCipher) & mekaar-core (State machine chat, SOS, Guardian)
[Fase 4: Headless CLI & End-to-End Test]
    └── Bikin mekaar-cli -> Uji komunikasi chat E2EE 2 perangkat via teplok-pc relay
[Fase 5: Klien UI Tauri v2 (Mobile & Desktop)]
    └── Bangun UI Tri-Register di Tauri v2 -> Hubungkan command/event ke mekaar-core
    └── Build APK Android & binary Desktop
```

---

*Dokumen PRD v2.0 ini menjadi spesifikasi final resmi untuk pembangunan sistem MEKAAR Full-Rust.*
