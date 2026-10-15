<script lang="ts">
  import { decodeStegoMessage } from '../services/steganography';

  let {
    open = false,
    imageSrc = '',
    carrierName = 'Citra Steganografi',
    onclose
  }: {
    open: boolean;
    imageSrc: string;
    carrierName?: string;
    onclose: () => void;
  } = $props();

  let passkey = $state('');
  let extractedMessage = $state('');
  let isEncrypted = $state(false);
  let isDecoding = $state(false);
  let errorMessage = $state('');
  let copied = $state(false);

  $effect(() => {
    if (open && imageSrc) {
      extractedMessage = '';
      errorMessage = '';
      runDecode();
    }
  });

  async function runDecode() {
    isDecoding = true;
    errorMessage = '';
    copied = false;

    try {
      const res = await decodeStegoMessage(imageSrc, passkey);
      if (res.success) {
        extractedMessage = res.message;
        isEncrypted = !!res.isEncrypted;
      } else {
        if (res.isEncrypted) {
          isEncrypted = true;
          errorMessage = res.message;
        } else {
          errorMessage = res.message || 'Gagal mengekstrak pesan tersembunyi.';
        }
      }
    } catch (err: any) {
      errorMessage = err?.message || 'Kesalahan saat decoding steganografi.';
    } finally {
      isDecoding = false;
    }
  }

  async function handleCopy() {
    if (!extractedMessage) return;
    try {
      await navigator.clipboard.writeText(extractedMessage);
      copied = true;
      setTimeout(() => (copied = false), 2000);
    } catch (_) {
      copied = true;
      setTimeout(() => (copied = false), 2000);
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="stego-dialog-backdrop" onclick={onclose}>
    <div class="stego-dialog-modal" onclick={(e) => e.stopPropagation()}>
      <!-- Header -->
      <div class="dialog-header-row">
        <div class="header-title-wrap">
          <span class="stego-icon">🕵️</span>
          <div>
            <h3 class="dialog-title">Dekoder Steganografi Mekaar</h3>
            <p class="dialog-sub">Ekstraksi pesan rahasia terselubung di dalam piksel citra</p>
          </div>
        </div>
        <button type="button" class="close-x-btn" onclick={onclose} aria-label="Tutup">
          ✕
        </button>
      </div>

      <!-- Carrier Image Preview -->
      <div class="carrier-preview-box">
        {#if imageSrc}
          <img src={imageSrc} alt="Carrier Stego" class="carrier-img" />
        {:else}
          <div class="carrier-placeholder">
            <span>🖼️ Citra Pembawa</span>
          </div>
        {/if}
        <span class="carrier-name-tag">{carrierName}</span>
      </div>

      <!-- Passkey input if encrypted or requested -->
      <div class="passkey-input-row">
        <label for="stego-passkey-input" class="passkey-label">
          Kunci Sandi Steganografi (jika ada):
        </label>
        <div class="input-with-button">
          <input
            id="stego-passkey-input"
            type="password"
            class="passkey-input"
            placeholder="Masukkan kata sandi dekripsi..."
            bind:value={passkey}
            onkeydown={(e) => { if (e.key === 'Enter') runDecode(); }}
          />
          <button
            type="button"
            class="btn-decode-action"
            disabled={isDecoding}
            onclick={runDecode}
          >
            {isDecoding ? 'Menganalisis...' : '🔓 Dekode'}
          </button>
        </div>
      </div>

      <!-- Error Alert -->
      {#if errorMessage}
        <div class="error-banner">
          <span>⚠️ {errorMessage}</span>
        </div>
      {/if}

      <!-- Extracted Result Box -->
      {#if extractedMessage}
        <div class="result-box">
          <div class="result-header">
            <span class="result-badge">
              ✓ PESAN RAHASIA TERBUKA {isEncrypted ? '(TERDEKRIPSI)' : ''}
            </span>
            <button
              type="button"
              class="btn-copy-secret"
              onclick={handleCopy}
            >
              {copied ? '✓ Disalin!' : '📋 Salin'}
            </button>
          </div>
          <p class="extracted-text">{extractedMessage}</p>
        </div>
      {/if}

      <!-- Forensic Notice -->
      <div class="forensic-notice-pill">
        🛡️ Pesan ini ditanamkan langsung pada bit LSB warna (RGB) dan tidak dapat dideteksi oleh filter lalu lintas teks relay.
      </div>
    </div>
  </div>
{/if}

<style>
  .stego-dialog-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(15, 23, 42, 0.75);
    backdrop-filter: blur(5px);
    z-index: 1070;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    animation: fadeIn 0.2s ease-out;
  }

  .stego-dialog-modal {
    width: 100%;
    max-width: 440px;
    background-color: var(--bg-card, #FFFFFF);
    border-radius: var(--radius-xl, 24px);
    border: 1px solid var(--border-subtle, #E2E8F0);
    box-shadow: 0 20px 40px -15px rgba(0, 0, 0, 0.35);
    padding: 20px;
    display: flex;
    flex-direction: column;
    position: relative;
    max-height: 90vh;
    overflow-y: auto;
  }

  .dialog-header-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 14px;
  }

  .header-title-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .stego-icon {
    font-size: 1.8rem;
  }

  .dialog-title {
    font-size: 1.05rem;
    font-weight: 800;
    color: var(--text-primary, #0F172A);
    margin: 0;
  }

  .dialog-sub {
    font-size: 0.74rem;
    color: var(--text-secondary, #64748B);
    margin: 2px 0 0 0;
  }

  .close-x-btn {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: transparent;
    border: none;
    color: var(--text-secondary, #64748B);
    font-size: 1.1rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-x-btn:hover {
    background-color: var(--border-light, #F1F5F9);
    color: var(--text-primary, #0F172A);
  }

  .carrier-preview-box {
    position: relative;
    width: 100%;
    max-height: 200px;
    background-color: #0F172A;
    border-radius: 14px;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 14px;
    border: 1px solid var(--border-subtle, #E2E8F0);
  }

  .carrier-img {
    width: 100%;
    height: 100%;
    max-height: 200px;
    object-fit: contain;
  }

  .carrier-placeholder {
    padding: 30px;
    color: #94A3B8;
    font-size: 0.9rem;
  }

  .carrier-name-tag {
    position: absolute;
    bottom: 8px;
    left: 8px;
    background-color: rgba(15, 23, 42, 0.75);
    color: #FFFFFF;
    font-size: 0.72rem;
    padding: 2px 8px;
    border-radius: 6px;
    backdrop-filter: blur(4px);
  }

  .passkey-input-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 12px;
  }

  .passkey-label {
    font-size: 0.76rem;
    font-weight: 700;
    color: var(--text-secondary, #64748B);
  }

  .input-with-button {
    display: flex;
    gap: 6px;
  }

  .passkey-input {
    flex: 1;
    padding: 8px 12px;
    border-radius: 10px;
    border: 1px solid var(--border-subtle, #CBD5E1);
    background-color: var(--bg-surface, #F8FAFC);
    font-size: 0.85rem;
    color: var(--text-primary, #0F172A);
    outline: none;
  }

  .passkey-input:focus {
    border-color: var(--brand-blue, #0284C7);
  }

  .btn-decode-action {
    padding: 8px 16px;
    border-radius: 10px;
    background-color: var(--brand-blue, #0284C7);
    color: #FFFFFF;
    font-size: 0.82rem;
    font-weight: 700;
    border: none;
    cursor: pointer;
    white-space: nowrap;
  }

  .error-banner {
    background-color: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 10px;
    padding: 8px 12px;
    font-size: 0.78rem;
    color: #DC2626;
    margin-bottom: 10px;
  }

  .result-box {
    background-color: rgba(13, 148, 136, 0.08);
    border: 1.5px solid #0D9488;
    border-radius: 12px;
    padding: 12px;
    margin-bottom: 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .result-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .result-badge {
    font-size: 0.7rem;
    font-weight: 800;
    color: #0D9488;
    letter-spacing: 0.5px;
  }

  .btn-copy-secret {
    background: transparent;
    border: 1px solid #0D9488;
    color: #0D9488;
    font-size: 0.72rem;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: var(--radius-pill, 999px);
    cursor: pointer;
  }

  .extracted-text {
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary, #0F172A);
    margin: 4px 0 0 0;
    line-height: 1.45;
    background: #FFFFFF;
    padding: 8px 10px;
    border-radius: 8px;
    border: 1px solid rgba(13, 148, 136, 0.2);
    word-break: break-word;
  }

  .forensic-notice-pill {
    font-size: 0.72rem;
    color: var(--text-secondary, #64748B);
    line-height: 1.35;
    background-color: var(--bg-surface, #F1F5F9);
    padding: 8px 12px;
    border-radius: 10px;
    text-align: center;
    margin-top: 4px;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
