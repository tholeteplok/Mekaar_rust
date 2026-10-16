<script lang="ts">
  import { mekaarStore } from '../stores';
  import { parseContactPayload } from '../services/qr';
  import Dialog from './Dialog.svelte';

  let { open = false, onclose, onConnectSuccess }: {
    open: boolean;
    onclose: () => void;
    onConnectSuccess?: (conversationId: string) => void;
  } = $props();

  let activeTab = $state<'camera' | 'manual' | 'upload'>('camera');
  let manualInput = $state('');
  let errorMessage = $state('');
  let isProcessing = $state(false);
  let videoEl = $state<HTMLVideoElement | null>(null);
  let cameraStream: MediaStream | null = null;
  let scanAnimationTimer: number | null = null;

  $effect(() => {
    if (open && activeTab === 'camera') {
      startCamera();
    } else {
      stopCamera();
    }

    return () => {
      stopCamera();
    };
  });

  async function startCamera() {
    errorMessage = '';
    try {
      if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
        cameraStream = await navigator.mediaDevices.getUserMedia({
          video: { facingMode: 'environment', width: { ideal: 640 }, height: { ideal: 480 } }
        });
        if (videoEl) {
          videoEl.srcObject = cameraStream;
          await videoEl.play().catch(() => {});
          initBarcodeDetection();
        }
      } else {
        errorMessage = 'Kamera tidak didukung pada perangkat ini. Silakan gunakan tab Tempel/Manual.';
      }
    } catch (err) {
      console.warn('Camera access error:', err);
      errorMessage = 'Tidak dapat mengakses kamera. Silakan periksa izin atau gunakan tab Tempel/Manual.';
    }
  }

  function stopCamera() {
    if (cameraStream) {
      cameraStream.getTracks().forEach((t) => t.stop());
      cameraStream = null;
    }
    if (scanAnimationTimer) {
      cancelAnimationFrame(scanAnimationTimer);
      scanAnimationTimer = null;
    }
  }

  function initBarcodeDetection() {
    if ('BarcodeDetector' in window) {
      try {
        // @ts-ignore
        const barcodeDetector = new window.BarcodeDetector({ formats: ['qr_code'] });
        const checkFrame = async () => {
          if (!videoEl || !cameraStream || isProcessing) return;
          try {
            const barcodes = await barcodeDetector.detect(videoEl);
            if (barcodes && barcodes.length > 0) {
              const rawValue = barcodes[0].rawValue;
              if (rawValue) {
                processPayload(rawValue);
                return;
              }
            }
          } catch (_) {}
          scanAnimationTimer = requestAnimationFrame(checkFrame);
        };
        scanAnimationTimer = requestAnimationFrame(checkFrame);
      } catch (_) {}
    }
  }

  async function processPayload(raw: string) {
    if (isProcessing) return;
    errorMessage = '';
    isProcessing = true;

    try {
      const parsed = parseContactPayload(raw);
      if (!parsed) {
        errorMessage = 'Format Kode QR tidak dikenali sebagai kontak Mekaar.';
        isProcessing = false;
        return;
      }

      // Validasi 1: Diri sendiri
      if (
        parsed.userId === mekaarStore.currentUser.id ||
        (parsed.username && parsed.username.toLowerCase() === mekaarStore.currentUser.username.toLowerCase())
      ) {
        errorMessage = 'Tidak bisa memulai chat dengan diri sendiri.';
        isProcessing = false;
        return;
      }

      // Validasi 2: Blokir
      const isBlocked = mekaarStore.contacts.some(
        (c) => c.id === parsed.userId && c.isBlocked
      );
      if (isBlocked) {
        errorMessage = 'Pengguna ini telah Anda blokir.';
        isProcessing = false;
        return;
      }

      // Simulasi jeda autentikasi kriptografis tatap muka (300ms)
      await new Promise((r) => setTimeout(r, 400));

      const res = mekaarStore.addContactFromQr(raw);
      if (res.success && res.conversationId) {
        stopCamera();
        onclose();
        if (onConnectSuccess) {
          onConnectSuccess(res.conversationId);
        } else {
          mekaarStore.openConversation(res.conversationId);
        }
      } else {
        errorMessage = res.message || 'Gagal menambahkan kontak.';
      }
    } catch (e: any) {
      errorMessage = e?.message || 'Gagal memproses kode QR. Coba pindai ulang.';
    } finally {
      isProcessing = false;
    }
  }

  function handleManualSubmit() {
    if (!manualInput.trim()) return;
    processPayload(manualInput.trim());
  }

  async function handleFileUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;
    const file = target.files[0];

    if ('BarcodeDetector' in window) {
      try {
        isProcessing = true;
        // @ts-ignore
        const barcodeDetector = new window.BarcodeDetector({ formats: ['qr_code'] });
        const img = new Image();
        img.src = URL.createObjectURL(file);
        await img.decode();
        const barcodes = await barcodeDetector.detect(img);
        if (barcodes && barcodes.length > 0) {
          processPayload(barcodes[0].rawValue);
        } else {
          errorMessage = 'Tidak ditemukan kode QR pada gambar tersebut.';
          isProcessing = false;
        }
      } catch (err: any) {
        errorMessage = 'Gagal memindai gambar: ' + (err?.message || 'Format tidak didukung');
        isProcessing = false;
      }
    } else {
      errorMessage = 'Fitur pemindai berkas membutuhkan browser dengan dukungan BarcodeDetector. Gunakan tab Tempel/Manual.';
    }
  }
</script>

<Dialog
  {open}
  title="Pindai QR Kontak"
  {onclose}
  maxWidth="460px"
>
  <div class="scan-dialog-content">
    <!-- Segment Tabs (Kamera, Manual/Tempel, Unggah) -->
    <div class="scan-tabs-bar">
      <button
        type="button"
        class="scan-tab-btn {activeTab === 'camera' ? 'active' : ''}"
        onclick={() => { activeTab = 'camera'; startCamera(); }}
      >
        📷 Kamera
      </button>
      <button
        type="button"
        class="scan-tab-btn {activeTab === 'manual' ? 'active' : ''}"
        onclick={() => { activeTab = 'manual'; stopCamera(); }}
      >
        ⌨️ Tempel / Manual
      </button>
      <button
        type="button"
        class="scan-tab-btn {activeTab === 'upload' ? 'active' : ''}"
        onclick={() => { activeTab = 'upload'; stopCamera(); }}
      >
        🖼️ Unggah
      </button>
    </div>

    <!-- Error Alert Banner -->
    {#if errorMessage}
      <div class="error-banner">
        <span class="error-icon">⚠️</span>
        <span class="error-text">{errorMessage}</span>
      </div>
    {/if}

    <!-- Tab Content 1: Camera Viewfinder -->
    {#if activeTab === 'camera'}
      <div class="camera-viewport-wrap">
        <video
          bind:this={videoEl}
          class="camera-video-feed"
          playsinline
          muted
          autoplay
        ></video>

        <!-- Viewfinder Frame ala contact_qr_scan_screen.dart (240x240 Soft Coral/Aegis Blue) -->
        <div class="viewfinder-frame">
          <div class="viewfinder-corner top-left"></div>
          <div class="viewfinder-corner top-right"></div>
          <div class="viewfinder-corner bottom-left"></div>
          <div class="viewfinder-corner bottom-right"></div>
          <div class="laser-scanner-line"></div>
        </div>

        <div class="camera-guidance-pill">
          Arahkan kamera ke kode QR profil teman Anda untuk memulai obrolan.
        </div>
      </div>
    {/if}

    <!-- Tab Content 2: Manual Payload Input (Off-grid / Desktop Fallback) -->
    {#if activeTab === 'manual'}
      <div class="manual-input-wrap">
        <p class="manual-desc">
          Masukkan ID kontak, tautan <code>mekaar://user/...</code>, atau <code>@username</code> untuk memulai obrolan langsung secara off-grid.
        </p>
        <div class="input-row">
          <input
            type="text"
            class="manual-text-input"
            placeholder="Contoh: mekaar://user/usr-citra-99?name=Citra+Kirana"
            bind:value={manualInput}
            onkeydown={(e) => { if (e.key === 'Enter') handleManualSubmit(); }}
          />
        </div>

        <!-- Quick Test Presets -->
        <div class="preset-pills-row">
          <span class="preset-label">Cepat:</span>
          <button
            type="button"
            class="preset-chip"
            onclick={() => (manualInput = 'mekaar://user/usr-citra-99?name=Citra+Kirana&username=citrakirana&pubkey=ED25519-78AB')}
          >
            + Citra Kirana
          </button>
          <button
            type="button"
            class="preset-chip"
            onclick={() => (manualInput = 'mekaar://user/usr-dewi-88?name=Dewi+Anggraini&username=dewianggraini&pubkey=ED25519-99FF')}
          >
            + Dewi Anggraini
          </button>
        </div>

        <button
          type="button"
          class="btn-submit-manual"
          disabled={!manualInput.trim() || isProcessing}
          onclick={handleManualSubmit}
        >
          Hubungkan Kontak & Mulai Chat
        </button>
      </div>
    {/if}

    <!-- Tab Content 3: Upload Image File -->
    {#if activeTab === 'upload'}
      <div class="upload-wrap">
        <div class="upload-dropzone">
          <span class="upload-icon">📂</span>
          <p class="upload-title">Pilih Berkas Gambar Kode QR</p>
          <p class="upload-subtitle">Format PNG, JPG, atau WebP</p>
          <label class="btn-browse-file">
            Pilih Berkas
            <input type="file" accept="image/*" onchange={handleFileUpload} style="display: none;" />
          </label>
        </div>
      </div>
    {/if}

    <!-- Processing Backdrop Overlay -->
    {#if isProcessing}
      <div class="processing-overlay">
        <div class="processing-card">
          <div class="processing-spinner"></div>
          <h4>Memproses kode...</h4>
          <p>Mekaar sedang mengenali profil teman Anda.</p>
        </div>
      </div>
    {/if}
  </div>
</Dialog>

<style>
  .scan-dialog-content {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  /* Tabs Bar */
  .scan-tabs-bar {
    display: flex;
    background-color: var(--bg-primary);
    padding: 4px;
    border-radius: var(--radius-pill);
    gap: 4px;
    margin-bottom: 14px;
  }

  .scan-tab-btn {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 7px 10px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    font-size: 0.8rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .scan-tab-btn.active {
    background-color: var(--bg-card);
    color: var(--brand-blue);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  }

  /* Error Banner */
  .error-banner {
    background-color: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: var(--radius-md);
    padding: 10px 14px;
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
    font-size: 0.82rem;
    color: #DC2626;
  }

  /* Camera Viewport */
  .camera-viewport-wrap {
    position: relative;
    width: 100%;
    height: 320px;
    background-color: #0F172A;
    border-radius: var(--radius-lg);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .camera-video-feed {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  /* Viewfinder Frame (240x240 ala Flutter) */
  .viewfinder-frame {
    position: absolute;
    width: 200px;
    height: 200px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 20px;
    box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5);
    pointer-events: none;
  }

  .viewfinder-corner {
    position: absolute;
    width: 24px;
    height: 24px;
    border-color: #F43F5E;
    border-style: solid;
    pointer-events: none;
  }

  .viewfinder-corner.top-left {
    top: -2px;
    left: -2px;
    border-width: 4px 0 0 4px;
    border-top-left-radius: 18px;
  }

  .viewfinder-corner.top-right {
    top: -2px;
    right: -2px;
    border-width: 4px 4px 0 0;
    border-top-right-radius: 18px;
  }

  .viewfinder-corner.bottom-left {
    bottom: -2px;
    left: -2px;
    border-width: 0 0 4px 4px;
    border-bottom-left-radius: 18px;
  }

  .viewfinder-corner.bottom-right {
    bottom: -2px;
    right: -2px;
    border-width: 0 4px 4px 0;
    border-bottom-right-radius: 18px;
  }

  .laser-scanner-line {
    position: absolute;
    top: 0;
    left: 4px;
    right: 4px;
    height: 2px;
    background: linear-gradient(90deg, transparent, #F43F5E, transparent);
    box-shadow: 0 0 8px #F43F5E;
    animation: scanLaser 2s infinite ease-in-out;
  }

  .camera-guidance-pill {
    position: absolute;
    bottom: 14px;
    left: 20px;
    right: 20px;
    padding: 8px 14px;
    background-color: rgba(15, 23, 42, 0.75);
    backdrop-filter: blur(4px);
    border-radius: var(--radius-pill);
    font-size: 0.78rem;
    color: #FFFFFF;
    text-align: center;
  }

  /* Manual Input View */
  .manual-input-wrap {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 10px 0;
  }

  .manual-desc {
    font-size: 0.82rem;
    color: var(--text-secondary);
    line-height: 1.4;
    margin: 0;
  }

  .manual-desc code {
    background-color: var(--bg-primary);
    padding: 2px 5px;
    border-radius: 4px;
    font-size: 0.8rem;
    color: var(--brand-blue);
  }

  .manual-text-input {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    padding: 10px 14px;
    border-radius: var(--radius-md);
    border: 1.5px solid var(--border-light);
    background-color: var(--bg-card);
    font-size: 0.88rem;
    font-family: var(--font-active);
    color: var(--text-primary);
    outline: none;
    box-sizing: border-box;
  }

  .manual-text-input:focus {
    border-color: var(--brand-blue);
  }

  .preset-pills-row {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .preset-label {
    font-size: 0.78rem;
    color: var(--text-secondary);
    font-weight: 600;
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
    font-size: 0.76rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .preset-chip:hover {
    background-color: var(--brand-blue-subtle);
    border-color: var(--brand-blue);
  }

  .btn-submit-manual {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    height: 44px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.9rem;
    font-weight: 700;
    font-family: var(--font-active);
    border: none;
    cursor: pointer;
    margin-top: 6px;
    transition: all 0.15s ease;
  }

  .btn-submit-manual:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Upload View */
  .upload-wrap {
    padding: 16px 0;
  }

  .upload-dropzone {
    border: 2px dashed var(--border-light);
    border-radius: var(--radius-lg);
    padding: 36px 20px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-primary);
  }

  .upload-icon {
    font-size: 2.2rem;
  }

  .upload-title {
    font-size: 0.95rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary);
    margin: 0;
  }

  .upload-subtitle {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin: 0 0 10px 0;
  }

  .btn-browse-file {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 9px 20px;
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.85rem;
    font-weight: 700;
    font-family: var(--font-active);
    border-radius: var(--radius-pill);
    cursor: pointer;
  }

  /* Processing Overlay */
  .processing-overlay {
    position: absolute;
    inset: 0;
    background-color: rgba(15, 23, 42, 0.7);
    backdrop-filter: blur(3px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    z-index: 5;
    border-radius: var(--radius-lg);
  }

  .processing-card {
    background-color: var(--bg-card);
    border-radius: var(--radius-lg);
    padding: 24px 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    width: 100%;
    max-width: 280px;
  }

  .processing-spinner {
    width: 36px;
    height: 36px;
    border: 3px solid rgba(2, 132, 199, 0.2);
    border-top-color: var(--brand-blue);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: 12px;
  }

  .processing-card h4 {
    font-size: 1rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
    margin: 0 0 4px 0;
  }

  .processing-card p {
    font-size: 0.82rem;
    color: var(--text-secondary);
    margin: 0;
  }

  @keyframes scanLaser {
    0% { top: 0%; opacity: 0; }
    15% { opacity: 1; }
    85% { opacity: 1; }
    100% { top: 100%; opacity: 0; }
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
