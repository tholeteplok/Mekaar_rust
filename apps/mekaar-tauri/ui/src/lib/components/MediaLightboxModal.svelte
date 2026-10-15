<script lang="ts">
  interface Props {
    open: boolean;
    src: string;
    name: string;
    sizeLabel?: string;
    isStego?: boolean;
    onclose: () => void;
    onopenstego?: (src: string, name: string) => void;
  }

  let {
    open = false,
    src,
    name,
    sizeLabel = '',
    isStego = false,
    onclose,
    onopenstego
  }: Props = $props();

  let zoomLevel = $state(1); // 1, 1.5, 2, 2.5, 3
  let isSavedSuccess = $state(false);

  function handleZoomIn() {
    if (zoomLevel < 3) {
      zoomLevel = Math.min(3, +(zoomLevel + 0.5).toFixed(1));
    }
  }

  function handleZoomOut() {
    if (zoomLevel > 1) {
      zoomLevel = Math.max(1, +(zoomLevel - 0.5).toFixed(1));
    }
  }

  function handleResetZoom() {
    zoomLevel = 1;
  }

  function handleDownload() {
    if (!src) return;
    const a = document.createElement('a');
    a.href = src;
    a.download = name || 'mekaar_media.png';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    isSavedSuccess = true;
    setTimeout(() => {
      isSavedSuccess = false;
    }, 2500);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      onclose();
    } else if (e.key === '+' || e.key === '=') {
      handleZoomIn();
    } else if (e.key === '-') {
      handleZoomOut();
    } else if (e.key === '0') {
      handleResetZoom();
    }
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="lightbox-backdrop"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={onclose}
  >
    <!-- Lightbox Container (stop click propagation) -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="lightbox-modal"
      role="document"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Top Bar -->
      <div class="lightbox-header">
        <div class="header-left">
          <div class="image-title-row">
            <span class="image-name">{name || 'Foto Media'}</span>
            {#if isStego}
              <span class="stego-pill-tag">🕵️ LSB Stego</span>
            {/if}
            <span class="e2ee-pill-tag">🔒 E2EE</span>
          </div>
          {#if sizeLabel}
            <span class="image-subtext">{sizeLabel}</span>
          {/if}
        </div>

        <!-- Zoom Bar -->
        <div class="zoom-controls-cluster">
          <button
            type="button"
            class="zoom-btn"
            onclick={handleZoomOut}
            disabled={zoomLevel <= 1}
            title="Perkecil (-)"
            aria-label="Perkecil"
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
              <line x1="8" y1="11" x2="14" y2="11"></line>
            </svg>
          </button>

          <button
            type="button"
            class="zoom-level-indicator"
            onclick={handleResetZoom}
            title="Klik untuk reset ke 100%"
          >
            {Math.round(zoomLevel * 100)}%
          </button>

          <button
            type="button"
            class="zoom-btn"
            onclick={handleZoomIn}
            disabled={zoomLevel >= 3}
            title="Perbesar (+)"
            aria-label="Perbesar"
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
              <line x1="11" y1="8" x2="11" y2="14"></line>
              <line x1="8" y1="11" x2="14" y2="11"></line>
            </svg>
          </button>
        </div>

        <!-- Close Action -->
        <button
          type="button"
          class="lightbox-close-btn"
          onclick={onclose}
          title="Tutup (Esc)"
          aria-label="Tutup Pratinjau"
        >
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <!-- Main Viewport Area -->
      <div class="lightbox-stage">
        <div
          class="image-wrapper"
          style="transform: scale({zoomLevel}); cursor: {zoomLevel > 1 ? 'grab' : 'zoom-in'};"
          onclick={() => {
            if (zoomLevel === 1) zoomLevel = 1.5;
            else if (zoomLevel === 1.5) zoomLevel = 2;
            else zoomLevel = 1;
          }}
          role="button"
          tabindex="0"
          onkeydown={(e) => e.key === 'Enter' && handleZoomIn()}
        >
          <img src={src} alt={name} class="lightbox-img" />
        </div>
      </div>

      <!-- Stego Alert Notice if applicable -->
      {#if isStego}
        <div class="stego-detection-bar">
          <div class="stego-desc">
            <span class="stego-icon">🕵️</span>
            <div>
              <strong>Muatan Pesan Tersembunyi (LSB) Terdeteksi</strong>
              <p>Gambar ini memuat payload cipher terenkripsi di dalam piksel gambar.</p>
            </div>
          </div>
          {#if onopenstego}
            <button
              type="button"
              class="stego-extract-action-btn"
              onclick={() => {
                onclose();
                onopenstego!(src, name);
              }}
            >
              🔓 Ekstrak Pesan Steganografi
            </button>
          {/if}
        </div>
      {/if}

      <!-- Bottom Action Bar -->
      <div class="lightbox-footer">
        <div class="footer-left">
          {#if isSavedSuccess}
            <span class="saved-alert">✓ Berhasil disimpan ke perangkat</span>
          {/if}
        </div>

        <div class="footer-actions">
          <button
            type="button"
            class="footer-btn action-save"
            onclick={handleDownload}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="7 10 12 15 17 10"></polyline>
              <line x1="12" y1="15" x2="12" y2="3"></line>
            </svg>
            <span>Simpan Gambar</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .lightbox-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9999;
    background-color: rgba(10, 15, 29, 0.94);
    backdrop-filter: blur(12px);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .lightbox-modal {
    position: relative;
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    color: #F8FAFC;
    user-select: none;
    outline: none;
  }

  .lightbox-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 20px;
    background: linear-gradient(to bottom, rgba(15, 23, 42, 0.9), rgba(15, 23, 42, 0.4));
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    z-index: 10;
  }

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .image-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .image-name {
    font-size: 0.95rem;
    font-weight: 700;
    color: #FFFFFF;
    max-width: 280px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .stego-pill-tag {
    background-color: rgba(13, 148, 136, 0.3);
    border: 1px solid #14B8A6;
    color: #5EEAD4;
    font-size: 0.7rem;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 999px;
  }

  .e2ee-pill-tag {
    background-color: rgba(2, 132, 199, 0.3);
    border: 1px solid #38BDF8;
    color: #7DD3FC;
    font-size: 0.7rem;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 999px;
  }

  .image-subtext {
    font-size: 0.75rem;
    color: #94A3B8;
  }

  .zoom-controls-cluster {
    display: flex;
    align-items: center;
    gap: 6px;
    background-color: rgba(30, 41, 59, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 999px;
    padding: 3px 8px;
  }

  .zoom-btn {
    background: none;
    border: none;
    color: #E2E8F0;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 50%;
    transition: all 0.15s ease;
  }

  .zoom-btn:hover:not(:disabled) {
    background-color: rgba(255, 255, 255, 0.15);
    color: #FFFFFF;
  }

  .zoom-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .zoom-level-indicator {
    background: none;
    border: none;
    color: #38BDF8;
    font-size: 0.82rem;
    font-weight: 700;
    padding: 2px 6px;
    cursor: pointer;
  }

  .lightbox-close-btn {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #FFFFFF;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .lightbox-close-btn:hover {
    background-color: #E11D48;
    border-color: #E11D48;
  }

  .lightbox-stage {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    position: relative;
    padding: 20px;
  }

  .image-wrapper {
    transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    max-width: 90vw;
    max-height: 80vh;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .lightbox-img {
    max-width: 100%;
    max-height: 80vh;
    object-fit: contain;
    border-radius: 8px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.7);
  }

  .stego-detection-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin: 0 20px 10px 20px;
    padding: 10px 18px;
    background-color: rgba(13, 148, 136, 0.25);
    border: 1px solid rgba(20, 184, 166, 0.5);
    border-radius: 12px;
    backdrop-filter: blur(8px);
  }

  .stego-desc {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .stego-icon {
    font-size: 1.6rem;
  }

  .stego-desc strong {
    display: block;
    font-size: 0.88rem;
    color: #5EEAD4;
  }

  .stego-desc p {
    margin: 2px 0 0 0;
    font-size: 0.75rem;
    color: #CBD5E1;
  }

  .stego-extract-action-btn {
    background-color: #0D9488;
    color: #FFFFFF;
    border: none;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 0.82rem;
    font-weight: 700;
    cursor: pointer;
    white-space: nowrap;
    transition: background-color 0.15s ease;
  }

  .stego-extract-action-btn:hover {
    background-color: #0F766E;
  }

  .lightbox-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 24px;
    background: linear-gradient(to top, rgba(15, 23, 42, 0.9), rgba(15, 23, 42, 0.4));
    border-top: 1px solid rgba(255, 255, 255, 0.08);
  }

  .saved-alert {
    color: #34D399;
    font-size: 0.82rem;
    font-weight: 700;
    animation: fadeIn 0.2s ease;
  }

  .footer-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .footer-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 10px;
    font-size: 0.85rem;
    font-weight: 600;
    border: none;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-save {
    background-color: rgba(255, 255, 255, 0.12);
    color: #FFFFFF;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .action-save:hover {
    background-color: rgba(255, 255, 255, 0.22);
  }
</style>
