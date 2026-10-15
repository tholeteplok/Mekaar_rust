<script lang="ts">
  import { mekaarStore } from '../stores';

  let {
    open = false,
    onclose
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  let confirmInput = $state('');
  let isExecuting = $state(false);
  let errorMessage = $state('');

  $effect(() => {
    if (open) {
      confirmInput = '';
      errorMessage = '';
      isExecuting = false;
    }
  });

  const isValid = $derived(confirmInput.trim() === 'WIPE-MEKAAR');

  async function handleWipe() {
    if (!isValid) {
      errorMessage = 'Ketik "WIPE-MEKAAR" dengan tepat untuk mengonfirmasi pemusnahan.';
      return;
    }
    isExecuting = true;
    errorMessage = '';

    try {
      const ok = await mekaarStore.executeForensicWipe('WIPE-MEKAAR');
      if (ok) {
        onclose();
      } else {
        errorMessage = 'Gagal mengeksekusi penghancuran forensik.';
      }
    } catch (err: any) {
      errorMessage = err?.message || 'Terjadi kesalahan sistem saat pemusnahan data.';
    } finally {
      isExecuting = false;
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={onclose}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="modal-sheet"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-label="Pemusnahan Forensik Darurat"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="sheet-drag-handle"></div>

      <div class="sheet-header">
        <div style="display: flex; align-items: center; gap: 10px;">
          <div class="header-icon-circle">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#E11D48" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
              <line x1="12" y1="9" x2="12" y2="13"></line>
              <line x1="12" y1="17" x2="12.01" y2="17"></line>
            </svg>
          </div>
          <div>
            <h3 class="sheet-title">Pemusnahan Forensik Darurat</h3>
            <p class="sheet-sub">Penghapusan permanen militer (Crypto-Shredding Zeroing)</p>
          </div>
        </div>
        <button type="button" class="close-x-btn" onclick={onclose} aria-label="Tutup">✕</button>
      </div>

      <div class="sheet-body">
        <div class="danger-callout">
          <div class="danger-badge">⚠️ PERINGATAN KRITIS • TIDAK DAPAT DIBATALKAN</div>
          <p class="danger-text">
            Protokol ini dirancang untuk situasi darurat ekstrem ketika perangkat Anda berisiko disita fisik atau digeledah.
          </p>
          <ul class="danger-list">
            <li>Semua riwayat percakapan dan log obrolan akan dimusnahkan.</li>
            <li>Brankas media, berkas steganografi, dan file lampiran akan di-zeroing.</li>
            <li>Kunci sesi X25519 dan identitas lokal dihapus total.</li>
            <li>Aplikasi akan langsung di-reset ke kondisi awal <em>Cold-Start</em>.</li>
          </ul>
        </div>

        <div class="confirm-box">
          <label class="confirm-label" for="wipe-confirm-input">
            Ketik <strong>WIPE-MEKAAR</strong> di bawah ini untuk konfirmasi:
          </label>
          <input
            id="wipe-confirm-input"
            type="text"
            class="confirm-input"
            placeholder="WIPE-MEKAAR"
            bind:value={confirmInput}
            autocomplete="off"
            spellcheck="false"
          />
        </div>

        {#if errorMessage}
          <div class="error-banner">
            {errorMessage}
          </div>
        {/if}

        <div class="dialog-actions-row">
          <button type="button" class="btn-cancel" onclick={onclose} disabled={isExecuting}>
            Batalkan
          </button>
          <button
            type="button"
            class="btn-wipe {isValid ? 'ready' : ''}"
            onclick={handleWipe}
            disabled={!isValid || isExecuting}
          >
            {#if isExecuting}
              Sedang Memusnahkan Data...
            {:else}
              💥 Hancurkan Seluruh Data Sekarang
            {/if}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(15, 23, 42, 0.6);
    backdrop-filter: blur(6px);
    z-index: 999;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }

  .modal-sheet {
    background-color: var(--bg-card);
    width: 100%;
    max-width: 520px;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    box-shadow: 0 -8px 32px rgba(15, 23, 42, 0.25);
    display: flex;
    flex-direction: column;
    max-height: 85vh;
    overflow-y: auto;
    border-top: 2px solid #E11D48;
    animation: slideUp 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideUp {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }

  .sheet-drag-handle {
    width: 36px;
    height: 4px;
    background-color: var(--border-light);
    border-radius: var(--radius-pill);
    margin: 10px auto 4px auto;
  }

  .sheet-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px 14px 20px;
    border-bottom: 1px solid var(--border-light);
  }

  .header-icon-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: #FFE4E6;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .sheet-title {
    font-size: 1.05rem;
    font-weight: 700;
    color: #9F1239;
    margin: 0;
  }

  .sheet-sub {
    font-size: 0.78rem;
    color: var(--text-secondary);
    margin: 2px 0 0 0;
  }

  .close-x-btn {
    background: none;
    border: none;
    font-size: 1.15rem;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 50%;
  }

  .sheet-body {
    padding: 18px 20px 24px 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .danger-callout {
    background-color: #FFF1F2;
    border: 1px solid #FECDD3;
    border-radius: var(--radius-lg);
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .danger-badge {
    font-size: 0.74rem;
    font-weight: 800;
    color: #9F1239;
    letter-spacing: 0.5px;
  }

  .danger-text {
    font-size: 0.82rem;
    color: #881337;
    margin: 0;
    line-height: 1.45;
  }

  .danger-list {
    margin: 0;
    padding-left: 18px;
    font-size: 0.78rem;
    color: #881337;
    line-height: 1.5;
  }

  .confirm-box {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .confirm-label {
    font-size: 0.82rem;
    color: var(--text-primary);
  }

  .confirm-input {
    padding: 10px 14px;
    border-radius: var(--radius-md);
    border: 1.5px solid var(--border-light);
    font-size: 1.05rem;
    font-weight: 700;
    letter-spacing: 2px;
    text-align: center;
    background: #FFFFFF;
    color: #E11D48;
    outline: none;
  }

  .confirm-input:focus {
    border-color: #E11D48;
  }

  .error-banner {
    padding: 10px 14px;
    background-color: #FEE2E2;
    border: 1px solid #FCA5A5;
    border-radius: var(--radius-md);
    color: #991B1B;
    font-size: 0.82rem;
    font-weight: 600;
  }

  .dialog-actions-row {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 4px;
  }

  .btn-cancel {
    padding: 10px 18px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
  }

  .btn-wipe {
    padding: 10px 20px;
    border-radius: var(--radius-pill);
    background-color: #94A3B8;
    border: none;
    color: #FFFFFF;
    font-size: 0.85rem;
    font-weight: 700;
    cursor: not-allowed;
    transition: all 0.15s ease;
  }

  .btn-wipe.ready {
    background-color: #E11D48;
    cursor: pointer;
    box-shadow: 0 4px 12px rgba(225, 29, 72, 0.35);
  }

  .btn-wipe.ready:hover {
    background-color: #BE123C;
  }
</style>
