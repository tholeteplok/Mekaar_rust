<script lang="ts">
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

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="glossary-backdrop" onclick={onclose}>
    <div class="glossary-modal sketch-card" onclick={(e) => e.stopPropagation()}>
      <!-- Header Bar -->
      <div class="modal-header-row">
        <div class="header-title-box">
          <span class="header-icon">📖</span>
          <div>
            <h3 class="modal-title">Kamus Istilah & Panduan Keamanan</h3>
            <span class="modal-subtitle">Penjelasan ramah tentang teknologi & fitur proteksi Mekaar</span>
          </div>
        </div>
        <button type="button" class="close-modal-btn" onclick={onclose} aria-label="Tutup Glosarium">✕</button>
      </div>

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
          Mekaar Aegis Shield Protocol • Dirancang untuk perlindungan data maksimal
        </span>
        <button type="button" class="btn-understand" onclick={onclose}>
          Paham & Tutup
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .glossary-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.65);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 120;
    animation: fadeIn 0.18s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .glossary-modal {
    width: min(600px, 100%);
    max-height: 86vh;
    display: flex;
    flex-direction: column;
    padding: 18px 20px 16px;
    background: var(--bg-card, #FFFFFF);
    border-radius: var(--radius-lg, 16px);
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.25);
    overflow: hidden;
  }

  .modal-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .header-title-box {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-icon {
    font-size: 1.8rem;
  }

  .modal-title {
    font-size: 1.05rem;
    font-weight: 800;
    color: var(--text-main);
    margin: 0;
  }

  .modal-subtitle {
    font-size: 0.78rem;
    color: var(--text-secondary);
    display: block;
    margin-top: 2px;
  }

  .close-modal-btn {
    width: 32px;
    height: 32px;
    background: var(--bg-secondary, #F1F5F9);
    border: 1.5px solid var(--sketch-border-color, #CBD5E1);
    border-radius: 8px;
    font-weight: 700;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .close-modal-btn:hover {
    background: #E2E8F0;
  }

  /* Search Box */
  .search-box-wrap {
    display: flex;
    align-items: center;
    background: var(--bg-secondary, #F8FAFC);
    border: 1.5px solid var(--border-sketch-color, #CBD5E1);
    border-radius: 10px;
    padding: 6px 12px;
    gap: 8px;
    margin-bottom: 10px;
  }

  .search-icon {
    font-size: 0.95rem;
    color: var(--text-muted);
  }

  .search-input-field {
    flex: 1;
    border: none;
    background: transparent;
    font-family: inherit;
    font-size: 0.86rem;
    color: var(--text-main);
    outline: none;
  }

  .clear-search-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
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
    font-size: 0.78rem;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 999px;
    border: 1.5px solid var(--border-sketch-color, #CBD5E1);
    background: var(--bg-main, #FFFFFF);
    color: var(--text-secondary);
    white-space: nowrap;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .cat-pill-btn.cat-active {
    background: rgba(2, 132, 199, 0.1);
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
    background: var(--bg-main, #FFFFFF);
    border: 1.5px solid var(--border-sketch-color, #E2E8F0);
    border-radius: 12px;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: 2px 2px 0px rgba(0, 0, 0, 0.04);
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
    background: var(--bg-secondary, #F1F5F9);
    border-radius: 8px;
  }

  .term-title-col {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .term-main-title {
    font-size: 0.95rem;
    font-weight: 800;
    color: var(--text-main);
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
    background: var(--bg-secondary, #F8FAFC);
    border-radius: 8px;
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
    color: var(--text-muted);
  }

  .label-green {
    color: var(--brand-teal, #0D9488);
  }

  .callout-value {
    font-size: 0.78rem;
    color: var(--text-main);
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
    border-top: 1px solid var(--border-light, #E2E8F0);
  }

  .footer-note {
    font-size: 0.72rem;
    color: var(--text-muted);
  }

  .btn-understand {
    padding: 8px 18px;
    font-size: 0.84rem;
    font-weight: 700;
    color: #FFFFFF;
    background: var(--brand-blue, #0284C7);
    border: 1.5px solid #0369A1;
    border-radius: 8px;
    cursor: pointer;
    box-shadow: 2px 2px 0px #0369A1;
    transition: all 0.15s ease;
  }

  .btn-understand:hover {
    transform: translate(-1px, -1px);
    box-shadow: 3px 3px 0px #0369A1;
  }

  .btn-understand:active {
    transform: translate(1px, 1px);
    box-shadow: 1px 1px 0px #0369A1;
  }
</style>
