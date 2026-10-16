<script lang="ts">
  import Dialog from './Dialog.svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open, onclose }: Props = $props();

  let activeCategory = $state<'all' | 'network' | 'privacy' | 'crypto'>('all');
  let searchQuery = $state('');

  interface GlossaryItem {
    id: string;
    term: string;
    aka: string;
    category: 'network' | 'privacy' | 'crypto';
    icon: string;
    summary: string;
    howItWorks: string;
    benefit: string;
  }

  const glossaryItems: GlossaryItem[] = [
    {
      id: 'mesh',
      term: 'Mekaar Mesh Network',
      aka: 'Jaringan Komunikasi Privat Peer-to-Peer',
      category: 'network',
      icon: '🌐',
      summary: 'Arsitektur jaringan privat antar-node (perangkat pengguna dan server terpercaya) tanpa ketergantungan pada penyedia cloud publik terpusat.',
      howItWorks: 'Setiap node saling terhubung secara terenkripsi melalui terowongan aman (Tailscale/WireGuard), menciptakan rute privat mandiri yang kebal pemblokiran ISP komersial.',
      benefit: 'Kedaulatan data 100% terjaga; percakapan tidak pernah melewati server korporasi periklanan atau pihak ketiga.'
    },
    {
      id: 'e2ee',
      term: 'End-to-End Encryption (E2EE)',
      aka: 'Enkripsi Ujung-ke-Ujung (Aegis Protocol)',
      category: 'crypto',
      icon: '🔐',
      summary: 'Metode pengamanan di mana hanya pengirim dan penerima yang dapat membaca isi pesan atau mendengarkan panggilan suara/video.',
      howItWorks: 'Pesan dienkripsi di ponsel Anda sebelum dikirim dan baru didekripsi di ponsel penerima. Server perantara hanya melihat serangkaian teks acak teracak yang mustahil dibaca.',
      benefit: 'Bahkan jika server Mekaar disadap, pihak penyerang tetap tidak bisa mengintip obrolan Anda.'
    },
    {
      id: 'relay',
      term: 'Relay Calls (Perlindungan IP)',
      aka: 'Pengalihan Panggilan Melalui Node Aman',
      category: 'network',
      icon: '🛡️',
      summary: 'Fitur privasi panggilan yang menyembunyikan alamat IP asli Anda dari lawan bicara saat melakukan panggilan suara atau video.',
      howItWorks: 'Secara default, panggilan internet langsung (P2P) menghubungkan dua perangkat secara langsung sehingga lawan bicara dapat mengetahui alamat IP dan perkiraan lokasi Anda. Dengan Relay Calls, data dialihkan melalui server Mekaar sebagai perisai.',
      benefit: 'Mencegah pelacakan lokasi geografis nyata dan analisis lalu lintas jaringan oleh pihak yang Anda hubungi.'
    },
    {
      id: 'stego',
      term: 'Stego Photo & Media Vault',
      aka: 'Penyembunyian Pesan Rahasia di Balik Foto',
      category: 'privacy',
      icon: '🖼️',
      summary: 'Teknologi Steganografi yang memungkinkan Anda menyembunyikan berkas dokumen atau teks rahasia ke dalam piksel foto biasa.',
      howItWorks: 'Pesan rahasia dienkripsi terlebih dahulu dengan sandi (passkey), lalu bit-bitnya disisipkan secara halus ke nilai warna piksel gambar tanpa mengubah tampilan foto secara kasat mata.',
      benefit: 'Jika ponsel diperiksa secara paksa, foto tersebut hanya terlihat seperti gambar biasa (misalnya foto pemandangan atau dokumen umum).'
    },
    {
      id: 'burn',
      term: 'Burn on Exit & Countdown Wipe',
      aka: 'Pemusnahan Pesan Otomatis 2 Arah',
      category: 'privacy',
      icon: '🔥',
      summary: 'Sistem siklus hidup pesan otomatis yang memusnahkan riwayat percakapan seketika Anda menutup ruang obrolan atau saat penghitung waktu habis.',
      howItWorks: 'Mencakup seluruh pesan di kedua perangkat (pesan Anda dan pesan rekan) dengan menimpa memori (zeroing) sehingga data tidak dapat dipulihkan oleh software forensik.',
      benefit: 'Menghilangkan jejak digital di kedua belah pihak secara serentak; ideal untuk koordinasi lapangan yang sangat sensitif.'
    },
    {
      id: 'duress',
      term: 'Duress PIN (Cloak Mode)',
      aka: 'PIN Paksaan / Mode Samaran Senyap',
      category: 'privacy',
      icon: '🚨',
      summary: 'PIN keamanan khusus yang digunakan saat Anda dipaksa atau diancam oleh pihak lain untuk membuka aplikasi Mekaar.',
      howItWorks: 'Saat PIN Paksaan dimasukkan di layar kunci, aplikasi tetap terbuka seolah normal, namun brankas rahasia otomatis disembunyikan dan aplikasi menyiarkan sinyal Silent SOS diam-diam ke Wali Siaga (Guardian).',
      benefit: 'Melindungi keselamatan fisik pengguna dari ancaman kekerasan sekaligus menjaga data paling rahasia tetap terlindungi.'
    },
    {
      id: 'sas',
      term: 'Aegis Shield & SAS Code',
      aka: 'Short Authentication String (Anti-Penyadapan MITM)',
      category: 'crypto',
      icon: '🤝',
      summary: 'Mekanisme verifikasi visual untuk memastikan jalur enkripsi antara Anda dan kontak tidak disusupi oleh pihak penengah (Man-In-The-Middle).',
      howItWorks: 'Sistem menghasilkan 4 kelompok angka dan 4 ikon emoji unik berdasarkan kunci kriptografi kedua pihak. Anda cukup mencocokkan angka/emoji ini dengan kontak Anda (tatap muka atau telepon).',
      benefit: 'Jaminan mutlak bahwa Anda benar-benar terhubung ke orang yang dimaksud tanpa ada perantara yang menyadap.'
    },
    {
      id: 'panic_wipe',
      term: 'Panic Wipe (Pemusnahan Data)',
      aka: 'Emergency Forensic Zeroing',
      category: 'privacy',
      icon: '💣',
      summary: 'Tombol pemusnahan total yang menghapus basis data lokal, kunci enkripsi, dan riwayat obrolan seketika dalam situasi darurat kritis.',
      howItWorks: 'File basis data SQLCipher ditimpa dengan pola bit acak sebelum dihapus dari penyimpanan fisik perangkat.',
      benefit: 'Memastikan tidak ada bukti atau kunci kriptografi yang dapat diekstraksi jika perangkat disita secara paksa.'
    }
  ];

  const filteredItems = $derived.by(() => {
    return glossaryItems.filter((item) => {
      const matchCat = activeCategory === 'all' || item.category === activeCategory;
      const q = searchQuery.toLowerCase().trim();
      const matchQuery =
        !q ||
        item.term.toLowerCase().includes(q) ||
        item.aka.toLowerCase().includes(q) ||
        item.summary.toLowerCase().includes(q);
      return matchCat && matchQuery;
    });
  });
</script>

<Dialog
  {open}
  title="Kamus Istilah & Panduan Keamanan"
  {onclose}
  maxWidth="640px"
>
  <div class="glossary-dialog-content">
    <!-- Search Input -->
    <div class="search-box-wrap">
      <span class="search-icon">🔍</span>
      <input
        type="text"
        class="search-input-field"
        placeholder="Cari istilah (contoh: Mesh, E2EE, Relay, Stego, Burn)..."
        bind:value={searchQuery}
      />
      {#if searchQuery}
        <button type="button" class="clear-search-btn" onclick={() => (searchQuery = '')}>✕</button>
      {/if}
    </div>

    <!-- Category Filter Pills -->
    <div class="category-pills-row">
      <button
        type="button"
        class="cat-pill-btn {activeCategory === 'all' ? 'cat-active' : ''}"
        onclick={() => (activeCategory = 'all')}
      >
        Semua ({glossaryItems.length})
      </button>
      <button
        type="button"
        class="cat-pill-btn {activeCategory === 'network' ? 'cat-active' : ''}"
        onclick={() => (activeCategory = 'network')}
      >
        🌐 Jaringan & Mesh
      </button>
      <button
        type="button"
        class="cat-pill-btn {activeCategory === 'privacy' ? 'cat-active' : ''}"
        onclick={() => (activeCategory = 'privacy')}
      >
        🛡️ Privasi Ekstrem
      </button>
      <button
        type="button"
        class="cat-pill-btn {activeCategory === 'crypto' ? 'cat-active' : ''}"
        onclick={() => (activeCategory = 'crypto')}
      >
        🔐 Kriptografi & E2EE
      </button>
    </div>

    <!-- Scrollable Glossary List -->
    <div class="glossary-list-scroll">
      {#if filteredItems.length === 0}
        <div class="empty-state-box">
          <span style="font-size: 2rem;">🔍</span>
          <p style="font-size: 0.88rem; color: var(--text-secondary); margin-top: 6px;">
            Tidak ada istilah yang cocok dengan pencarian "<strong>{searchQuery}</strong>"
          </p>
        </div>
      {:else}
        {#each filteredItems as item (item.id)}
          <div class="glossary-card">
            <div class="card-top-row">
              <span class="term-icon">{item.icon}</span>
              <div class="term-title-col">
                <div class="term-main-title">{item.term}</div>
                <div class="term-aka-desc">{item.aka}</div>
              </div>
            </div>

            <p class="term-summary-text">
              {item.summary}
            </p>

            <div class="detail-callout-box">
              <div class="callout-section">
                <span class="callout-label">⚙️ CARA KERJA:</span>
                <span class="callout-value">{item.howItWorks}</span>
              </div>
              <div class="callout-section" style="margin-top: 6px;">
                <span class="callout-label label-green">🛡️ KEUNGGULAN KEAMANAN:</span>
                <span class="callout-value">{item.benefit}</span>
              </div>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Footer -->
    <div class="modal-footer-row">
      <span class="footer-note">
        Mekaar Aegis Shield Protocol • Perlindungan data maksimal
      </span>
      <button type="button" class="btn-understand" onclick={onclose}>
        Paham & Tutup
      </button>
    </div>
  </div>
</Dialog>

<style>
  .glossary-dialog-content {
    display: flex;
    flex-direction: column;
    max-height: 72vh;
  }

  /* Search Box */
  .search-box-wrap {
    display: flex;
    align-items: center;
    background: var(--bg-primary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-pill);
    padding: 6px 14px;
    gap: 8px;
    min-height: var(--touch-target-min, 40px);
    margin-bottom: 10px;
    box-sizing: border-box;
  }

  .search-icon {
    font-size: 0.95rem;
    color: var(--text-secondary);
  }

  .search-input-field {
    flex: 1;
    border: none;
    background: transparent;
    font-family: var(--font-active);
    font-size: 0.86rem;
    color: var(--text-primary);
    outline: none;
  }

  .clear-search-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 0.85rem;
    cursor: pointer;
    padding: 2px 4px;
  }

  /* Category Filter Pills */
  .category-pills-row {
    display: flex;
    gap: 6px;
    overflow-x: auto;
    padding-bottom: 8px;
    margin-bottom: 8px;
  }

  .cat-pill-btn {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 0.78rem;
    font-weight: 600;
    font-family: var(--font-active);
    padding: 4px 14px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    background: var(--bg-card);
    color: var(--text-secondary);
    white-space: nowrap;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .cat-pill-btn.cat-active {
    background: var(--brand-blue-subtle);
    border-color: var(--brand-blue);
    color: var(--brand-blue);
    font-weight: 700;
  }

  /* Glossary Scrollable Feed */
  .glossary-list-scroll {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding-right: 4px;
    margin-bottom: 12px;
  }

  .glossary-card {
    background: var(--bg-card);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-md);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  }

  .card-top-row {
    display: flex;
    align-items: flex-start;
    gap: 10px;
  }

  .term-icon {
    font-size: 1.5rem;
    line-height: 1;
    padding: 6px;
    background: var(--bg-primary);
    border-radius: var(--radius-sm);
  }

  .term-title-col {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .term-main-title {
    font-size: 0.95rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
  }

  .term-aka-desc {
    font-size: 0.76rem;
    font-weight: 600;
    color: var(--brand-blue);
  }

  .term-summary-text {
    font-size: 0.82rem;
    line-height: 1.45;
    color: var(--text-secondary);
    margin: 0;
  }

  .detail-callout-box {
    background: var(--bg-primary);
    border-radius: var(--radius-sm);
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    border-left: 3px solid var(--brand-blue);
  }

  .callout-section {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .callout-label {
    font-size: 0.68rem;
    font-weight: 800;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
  }

  .label-green {
    color: var(--brand-teal, #0D9488);
  }

  .callout-value {
    font-size: 0.78rem;
    color: var(--text-primary);
    line-height: 1.35;
  }

  .empty-state-box {
    padding: 36px 16px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  /* Modal Footer */
  .modal-footer-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 10px;
    border-top: 1px solid var(--border-light);
    gap: 10px;
    flex-wrap: wrap;
  }

  .footer-note {
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .btn-understand {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 8px 18px;
    font-size: 0.84rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: #FFFFFF;
    background: var(--brand-blue);
    border: none;
    border-radius: var(--radius-pill);
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(37, 99, 235, 0.25);
    transition: all 0.15s ease;
  }

  .btn-understand:hover {
    background-color: #1D4ED8;
  }
</style>
