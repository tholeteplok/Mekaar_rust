<script lang="ts">
  import type { AttachmentModel } from '../types';
  import { encodeStegoMessage, createSyntheticCarrierImage } from '../services/steganography';
  import Dialog from './Dialog.svelte';

  let {
    open = false,
    onclose,
    onSelectAttachment
  }: {
    open: boolean;
    onclose: () => void;
    onSelectAttachment: (attachment: AttachmentModel) => void;
  } = $props();

  let activeTab = $state<'image' | 'stego' | 'document' | 'voice' | 'location'>('stego');

  // Steganography States
  let stegoSecretText = $state('');
  let stegoPasskey = $state('');
  let stegoCarrierPreview = $state('');
  let isEncodingStego = $state(false);
  let stegoErrorMessage = $state('');

  // Document States
  let docFileName = $state('Laporan_Operasi_Aegis.pdf');
  let docFileSize = $state('850 KB');

  // Location States
  let locationMode = $state<'live' | 'static'>('live');
  let liveDurationMinutes = $state<number>(60);

  $effect(() => {
    if (open && !stegoCarrierPreview) {
      // Siapkan synthetic carrier default
      stegoCarrierPreview = createSyntheticCarrierImage(
        'Laporan Posko Jebres',
        'Dokumen Koordinasi Relawan'
      );
    }
  });

  function handleImageFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;
    const file = target.files[0];
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result === 'string') {
        stegoCarrierPreview = reader.result;
      }
    };
    reader.readAsDataURL(file);
  }

  let isViewOnce = $state(false);

  function sendStandardImage() {
    onSelectAttachment({
      id: `att-${Date.now()}`,
      kind: 'image',
      name: isViewOnce ? 'Foto_Sekali_Lihat.png' : 'Foto_Dokumentasi_Lapangan.png',
      sizeLabel: isViewOnce ? '1.2 MB • Sekali Lihat (View-Once)' : '1.2 MB • Terenkripsi E2EE',
      urlOrPreview: stegoCarrierPreview || createSyntheticCarrierImage('Foto Lapangan', 'Dokumentasi Situasi'),
      encryptedChecksum: 'SHA256: 88F1-CC40-1120-E9AB'
    });
    isViewOnce = false;
    onclose();
  }

  async function sendSteganographyImage() {
    if (!stegoSecretText.trim()) {
      stegoErrorMessage = 'Silakan tulis pesan rahasia yang ingin disisipkan.';
      return;
    }

    isEncodingStego = true;
    stegoErrorMessage = '';

    try {
      const carrier = stegoCarrierPreview || createSyntheticCarrierImage('Dokumentasi Wilayah', 'Peta Lapangan');
      const stegoPng = await encodeStegoMessage(carrier, stegoSecretText.trim(), stegoPasskey.trim());

      onSelectAttachment({
        id: `att-${Date.now()}`,
        kind: 'image',
        name: 'Citra_Stego_LSB.png',
        sizeLabel: '1.5 MB • Stego LSB',
        urlOrPreview: stegoPng,
        isStego: true,
        stegoPasskeyProtected: !!stegoPasskey.trim(),
        encryptedChecksum: 'SHA256: E801-44B2-990A-FA33'
      });
      stegoSecretText = '';
      stegoPasskey = '';
      onclose();
    } catch (err: any) {
      stegoErrorMessage = err?.message || 'Gagal menyisipkan pesan ke dalam gambar.';
    } finally {
      isEncodingStego = false;
    }
  }

  function sendDocument() {
    onSelectAttachment({
      id: `att-${Date.now()}`,
      kind: 'document',
      name: docFileName,
      sizeLabel: `${docFileSize} • ChaCha20-Poly1305`,
      urlOrPreview: docFileName,
      encryptedChecksum: 'SHA256: 12DE-55F4-A011-884C'
    });
    onclose();
  }

  function sendVoiceNote() {
    onSelectAttachment({
      id: `att-${Date.now()}`,
      kind: 'voice',
      name: 'Pesan Suara (00:15)',
      sizeLabel: '0:15 • Opus E2EE',
      urlOrPreview: 'voice',
      durationSec: 15,
      encryptedChecksum: 'SHA256: 77A2-11C0-B904-44F1'
    });
    onclose();
  }

  function sendLocation() {
    if (locationMode === 'live') {
      const label = liveDurationMinutes >= 60 ? `${liveDurationMinutes / 60} Jam` : `${liveDurationMinutes} Menit`;
      onSelectAttachment({
        id: `att-${Date.now()}`,
        kind: 'location',
        name: `Lokasi Langsung: Surakarta (${label})`,
        sizeLabel: `Live GPS • ${label}`,
        urlOrPreview: 'geo:-7.5583,110.8566',
        latitude: -7.5583,
        longitude: 110.8566,
        isLiveLocation: true,
        liveDurationMinutes,
        liveExpiresAt: `${liveDurationMinutes}m lagi`,
        batteryLevel: 86,
        isLiveActive: true,
        encryptedChecksum: 'SHA256: AA90-1122-3344-5566'
      });
    } else {
      onSelectAttachment({
        id: `att-${Date.now()}`,
        kind: 'location',
        name: 'Titik Aman: Posko Jebres, Surakarta',
        sizeLabel: '-7.5583, 110.8566 • GPS Terenkripsi',
        urlOrPreview: 'geo:-7.5583,110.8566',
        latitude: -7.5583,
        longitude: 110.8566,
        isLiveLocation: false,
        encryptedChecksum: 'SHA256: AA90-1122-3344-5566'
      });
    }
    onclose();
  }
</script>

<Dialog
  {open}
  title="Kirim Lampiran Aman"
  {onclose}
  maxWidth="480px"
>
  <div class="attach-dialog-content">
    <!-- Segment Tabs -->
    <div class="attach-tabs-bar">
      <button
        type="button"
        class="attach-tab-btn {activeTab === 'stego' ? 'active' : ''}"
        onclick={() => (activeTab = 'stego')}
      >
        🕵️ Steganografi
      </button>
      <button
        type="button"
        class="attach-tab-btn {activeTab === 'image' ? 'active' : ''}"
        onclick={() => (activeTab = 'image')}
      >
        🖼️ Gambar
      </button>
      <button
        type="button"
        class="attach-tab-btn {activeTab === 'document' ? 'active' : ''}"
        onclick={() => (activeTab = 'document')}
      >
        📄 Dokumen
      </button>
      <button
        type="button"
        class="attach-tab-btn {activeTab === 'voice' ? 'active' : ''}"
        onclick={() => (activeTab = 'voice')}
      >
        🎙️ Suara
      </button>
      <button
        type="button"
        class="attach-tab-btn {activeTab === 'location' ? 'active' : ''}"
        onclick={() => (activeTab = 'location')}
      >
        📍 Lokasi
      </button>
    </div>

    <!-- Tab 1: Steganography (LSB Covert Text) -->
    {#if activeTab === 'stego'}
      <div class="tab-content-wrap">
        <p class="tab-desc">
          Sisipkan pesan rahasia ke dalam bit warna citra. Gambar terlihat wajar bagi penyadap, namun membawa payload tersembunyi.
        </p>

        <div class="carrier-box">
          {#if stegoCarrierPreview}
            <img src={stegoCarrierPreview} alt="Carrier" class="preview-carrier-img" />
          {/if}
          <label class="btn-change-carrier">
            Ganti Gambar Pembawa
            <input type="file" accept="image/*" onchange={handleImageFileSelect} style="display: none;" />
          </label>
        </div>

        <div class="field-block">
          <label for="secret-message-text" class="field-label">Pesan Rahasia yang Disembunyikan:</label>
          <textarea
            id="secret-message-text"
            class="secret-textarea"
            placeholder="Tuliskan pesan rahasia tingkat tinggi (misal: Rencana evakuasi darurat jam 22:00)..."
            bind:value={stegoSecretText}
            rows="3"
          ></textarea>
        </div>

        <div class="field-block">
          <label for="stego-passkey" class="field-label">Kunci Sandi Ekstraksi (Opsional):</label>
          <input
            id="stego-passkey"
            type="password"
            class="passkey-input"
            placeholder="Kosongkan jika tanpa kata sandi..."
            bind:value={stegoPasskey}
          />
        </div>

        {#if stegoErrorMessage}
          <div class="error-banner">⚠️ {stegoErrorMessage}</div>
        {/if}

        <button
          type="button"
          class="btn-primary-send"
          disabled={isEncodingStego || !stegoSecretText.trim()}
          onclick={sendSteganographyImage}
        >
          {isEncodingStego ? 'Menyisipkan Bit LSB...' : '🚀 Enkripsi & Kirim Gambar Steganografi'}
        </button>
      </div>
    {/if}

    <!-- Tab 2: Standard Image -->
    {#if activeTab === 'image'}
      <div class="tab-content-wrap">
        <p class="tab-desc">Kirim foto dokumen atau dokumentasi visual dengan enkripsi AES-256 E2EE.</p>
        <div class="carrier-box">
          {#if stegoCarrierPreview}
            <img src={stegoCarrierPreview} alt="Carrier" class="preview-carrier-img" />
          {/if}
          <label class="btn-change-carrier">
            Pilih Foto Galeri
            <input type="file" accept="image/*" onchange={handleImageFileSelect} style="display: none;" />
          </label>
        </div>
        <div style="display: flex; align-items: center; gap: 8px; margin: 10px 0; padding: 10px; background: var(--bg-surface, #F8FAFC); border: 1px solid var(--border-subtle, #E2E8F0); border-radius: 8px;">
          <input type="checkbox" id="view-once-chk" bind:checked={isViewOnce} style="cursor: pointer;" />
          <label for="view-once-chk" style="font-size: 0.85rem; font-weight: 600; cursor: pointer; color: var(--text-primary);">
            🔒 Pesan Sekali Lihat (View-Once)
          </label>
        </div>
        <button type="button" class="btn-primary-send" onclick={sendStandardImage}>
          Kirim Gambar Terenkripsi
        </button>
      </div>
    {/if}

    <!-- Tab 3: Document -->
    {#if activeTab === 'document'}
      <div class="tab-content-wrap">
        <p class="tab-desc">Kirim berkas PDF, dokumen teks, atau arsip ZIP dengan verifikasi integritas checksum SHA-256.</p>
        <div class="doc-preset-card">
          <span class="doc-icon">📑</span>
          <div class="doc-meta">
            <input type="text" class="doc-name-input" bind:value={docFileName} />
            <span class="doc-sub">{docFileSize} • ChaCha20-Poly1305</span>
          </div>
        </div>
        <div class="preset-docs-row">
          <button
            type="button"
            class="preset-chip"
            onclick={() => { docFileName = 'Laporan_Evakuasi.pdf'; docFileSize = '1.2 MB'; }}
          >
            + Laporan_Evakuasi.pdf
          </button>
          <button
            type="button"
            class="preset-chip"
            onclick={() => { docFileName = 'Kunci_Cadangan_Grup.enc'; docFileSize = '128 KB'; }}
          >
            + Kunci_Cadangan.enc
          </button>
        </div>
        <button type="button" class="btn-primary-send" onclick={sendDocument}>
          Kirim Dokumen Terenkripsi
        </button>
      </div>
    {/if}

    <!-- Tab 4: Voice Note -->
    {#if activeTab === 'voice'}
      <div class="tab-content-wrap center-align">
        <div class="mic-pulse-circle">🎙️</div>
        <p class="voice-title">Pesan Suara Terenkripsi Opus</p>
        <p class="tab-desc">Audio dikompresi dengan codec Opus latensi rendah dan dienkripsi ujung-ke-ujung.</p>
        <button type="button" class="btn-primary-send" onclick={sendVoiceNote}>
          Kirim Rekaman Suara (00:15)
        </button>
      </div>
    {/if}

    <!-- Tab 5: Location -->
    {#if activeTab === 'location'}
      <div class="tab-content-wrap">
        <!-- Mode Selector Pill Bar -->
        <div class="location-mode-toggle">
          <button
            type="button"
            class="mode-pill-btn {locationMode === 'live' ? 'active-live' : ''}"
            onclick={() => (locationMode = 'live')}
          >
            🟢 Lokasi Langsung (Live)
          </button>
          <button
            type="button"
            class="mode-pill-btn {locationMode === 'static' ? 'active-static' : ''}"
            onclick={() => (locationMode = 'static')}
          >
            📍 Titik Lokasi Statis
          </button>
        </div>

        {#if locationMode === 'live'}
          <div class="live-config-card">
            <span class="live-config-title">Pilih Durasi Siaran Langsung:</span>
            <div class="duration-pills-row">
              <button
                type="button"
                class="duration-pill {liveDurationMinutes === 15 ? 'selected' : ''}"
                onclick={() => (liveDurationMinutes = 15)}
              >
                15 Menit
              </button>
              <button
                type="button"
                class="duration-pill {liveDurationMinutes === 60 ? 'selected' : ''}"
                onclick={() => (liveDurationMinutes = 60)}
              >
                1 Jam
              </button>
              <button
                type="button"
                class="duration-pill {liveDurationMinutes === 480 ? 'selected' : ''}"
                onclick={() => (liveDurationMinutes = 480)}
              >
                8 Jam
              </button>
            </div>

            <div class="telemetry-info-box">
              <div class="telemetry-item">
                <span class="telemetry-icon">🔋</span>
                <span>Baterai: <strong>86%</strong></span>
              </div>
              <div class="telemetry-item">
                <span class="telemetry-icon">🛰️</span>
                <span>Akurasi: <strong>±4m (E2EE)</strong></span>
              </div>
            </div>

            <p class="live-notice-text">
              Penerima dapat memantau pergerakan rute Anda secara langsung di Peta Radar selama sesi aktif. Anda dapat membatalkan kapan saja.
            </p>
          </div>
        {:else}
          <div class="static-config-card">
            <div class="static-pin-preview">📍</div>
            <p class="static-name-label">Posko Jebres, Surakarta</p>
            <p class="static-coord-label">-7.5583, 110.8566</p>
            <p class="tab-desc">Membagikan titik statis lokasi saat ini tanpa pelacakan posisi secara kontinyu.</p>
          </div>
        {/if}

        <button
          type="button"
          class="btn-primary-send"
          onclick={sendLocation}
        >
          {locationMode === 'live' ? `Bagikan Lokasi Langsung (${liveDurationMinutes >= 60 ? `${liveDurationMinutes / 60} Jam` : `${liveDurationMinutes} Menit`})` : 'Bagikan Titik Lokasi Aman'}
        </button>
      </div>
    {/if}
  </div>
</Dialog>

<style>
  .attach-dialog-content {
    display: flex;
    flex-direction: column;
    max-height: 75vh;
  }

  .attach-tabs-bar {
    display: flex;
    background-color: var(--bg-primary);
    padding: 4px;
    border-radius: var(--radius-pill);
    gap: 4px;
    margin-bottom: 14px;
    overflow-x: auto;
  }

  .attach-tab-btn {
    flex: 1;
    white-space: nowrap;
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 6px 12px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    font-size: 0.74rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .attach-tab-btn.active {
    background-color: var(--bg-card);
    color: var(--brand-blue);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  }

  .tab-content-wrap {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .tab-content-wrap.center-align {
    align-items: center;
    text-align: center;
    padding: 16px 0;
  }

  .tab-desc {
    font-size: 0.8rem;
    color: var(--text-secondary);
    line-height: 1.4;
    margin: 0;
  }

  .carrier-box {
    position: relative;
    width: 100%;
    height: 140px;
    background-color: #0F172A;
    border-radius: var(--radius-md);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .preview-carrier-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .btn-change-carrier {
    position: absolute;
    bottom: 8px;
    right: 8px;
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(15, 23, 42, 0.8);
    color: #FFFFFF;
    font-size: 0.74rem;
    font-weight: 700;
    font-family: var(--font-active);
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    cursor: pointer;
    backdrop-filter: blur(4px);
  }

  .field-block {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .field-label {
    font-size: 0.74rem;
    font-weight: 700;
    color: var(--text-secondary);
  }

  .secret-textarea {
    width: 100%;
    padding: 8px 12px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-light);
    background-color: var(--bg-card);
    font-size: 0.84rem;
    font-family: var(--font-active);
    color: var(--text-primary);
    outline: none;
    box-sizing: border-box;
    resize: none;
  }

  .secret-textarea:focus {
    border-color: var(--brand-blue);
  }

  .passkey-input {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    padding: 8px 12px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-light);
    background-color: var(--bg-card);
    font-size: 0.84rem;
    font-family: var(--font-active);
    color: var(--text-primary);
    outline: none;
    box-sizing: border-box;
  }

  .passkey-input:focus {
    border-color: var(--brand-blue);
  }

  .error-banner {
    background-color: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: var(--radius-sm);
    padding: 6px 10px;
    font-size: 0.76rem;
    color: #DC2626;
  }

  .btn-primary-send {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    height: 44px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.88rem;
    font-weight: 700;
    font-family: var(--font-active);
    border: none;
    cursor: pointer;
    margin-top: 4px;
    transition: all 0.15s ease;
  }

  .btn-primary-send:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Document Tab */
  .doc-preset-card {
    display: flex;
    align-items: center;
    gap: 12px;
    background-color: var(--bg-primary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-md);
    padding: 12px;
  }

  .doc-icon {
    font-size: 2rem;
  }

  .doc-meta {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .doc-name-input {
    width: 100%;
    font-size: 0.88rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary);
    border: 1px solid transparent;
    background: transparent;
    outline: none;
  }

  .doc-name-input:focus {
    border-bottom: 1px solid var(--brand-blue);
  }

  .doc-sub {
    font-size: 0.76rem;
    color: var(--text-secondary);
  }

  .preset-docs-row {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }

  .preset-chip {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    background-color: var(--bg-primary);
    border: 1px solid var(--border-light);
    color: var(--brand-blue);
    font-size: 0.74rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
  }

  /* Voice */
  .mic-pulse-circle {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    background-color: var(--brand-blue-subtle);
    color: var(--brand-blue);
    font-size: 1.8rem;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 6px;
  }

  .voice-title {
    font-size: 1rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
    margin: 0;
  }

  .location-mode-toggle {
    display: flex;
    background-color: var(--bg-primary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-pill);
    padding: 4px;
    gap: 6px;
  }

  .mode-pill-btn {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 8px 10px;
    border-radius: var(--radius-pill);
    font-size: 0.78rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    transition: all 0.15s ease;
  }

  .mode-pill-btn.active-live {
    background-color: var(--brand-teal-light, #ECFDF5);
    color: var(--brand-teal, #059669);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  }

  .mode-pill-btn.active-static {
    background-color: var(--bg-card);
    color: var(--brand-blue);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  }

  .live-config-card, .static-config-card {
    background-color: var(--bg-primary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-md);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .live-config-title {
    font-size: 0.8rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .duration-pills-row {
    display: flex;
    gap: 8px;
  }

  .duration-pill {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 8px 0;
    border-radius: var(--radius-pill);
    border: 1.5px solid var(--border-light);
    background-color: var(--bg-card);
    color: var(--text-primary);
    font-size: 0.8rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .duration-pill.selected {
    border-color: #10B981;
    background-color: #ECFDF5;
    color: #047857;
  }

  .telemetry-info-box {
    display: flex;
    justify-content: space-between;
    background-color: var(--brand-blue-subtle);
    border: 1px solid rgba(2, 132, 199, 0.15);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    font-size: 0.76rem;
    color: var(--text-secondary);
  }

  .telemetry-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .telemetry-item strong {
    color: var(--text-primary);
  }

  .live-notice-text {
    font-size: 0.72rem;
    color: var(--text-secondary);
    line-height: 1.4;
    margin: 0;
  }

  .static-pin-preview {
    font-size: 2.2rem;
    text-align: center;
    margin-top: 4px;
  }

  .static-name-label {
    font-size: 0.92rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
    text-align: center;
    margin: 0;
  }

  .static-coord-label {
    font-size: 0.76rem;
    color: var(--brand-blue);
    font-family: var(--font-mono);
    text-align: center;
    margin: 0;
  }
</style>
