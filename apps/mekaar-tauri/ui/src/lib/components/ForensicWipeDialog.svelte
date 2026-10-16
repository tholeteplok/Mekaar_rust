<script lang="ts">
  import { mekaarStore } from '../stores';
  import BottomSheet from './BottomSheet.svelte';

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

<BottomSheet
  {open}
  title="Pemusnahan Forensik Darurat"
  subtitle="Penghapusan permanen militer (Crypto-Shredding Zeroing)"
  {onclose}
  maxWidth="520px"
>
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
</BottomSheet>

<style>
  .sheet-body {
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
    min-height: var(--touch-target-min, 40px);
    padding: 10px 14px;
    border-radius: var(--radius-md);
    border: 1.5px solid var(--border-light);
    font-size: 1.05rem;
    font-weight: 700;
    font-family: var(--font-mono);
    letter-spacing: 2px;
    text-align: center;
    background: var(--bg-card);
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
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 18px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 600;
    font-family: var(--font-active);
    cursor: pointer;
  }

  .btn-wipe {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 20px;
    border-radius: var(--radius-pill);
    background-color: #94A3B8;
    border: none;
    color: #FFFFFF;
    font-size: 0.85rem;
    font-weight: 700;
    font-family: var(--font-active);
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
