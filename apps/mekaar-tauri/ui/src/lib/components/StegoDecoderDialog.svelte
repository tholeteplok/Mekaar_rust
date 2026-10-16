<script lang="ts">
  import { decodeStegoMessage } from '../services/steganography';
  import Dialog from './Dialog.svelte';
  import {
    Image,
    LockKeyOpen,
    Info,
    WarningCircle,
    ShieldCheck,
    Copy,
    Check
  } from 'phosphor-svelte';

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
  let infoMessage = $state('');
  let copied = $state(false);
  let imgError = $state(false);

  $effect(() => {
    if (open && imageSrc) {
      extractedMessage = '';
      errorMessage = '';
      infoMessage = '';
      imgError = false;
      runDecode(false);
    }
  });

  async function runDecode(isManual = true) {
    isDecoding = true;
    errorMessage = '';
    infoMessage = '';
    copied = false;

    try {
      const res = await decodeStegoMessage(imageSrc, passkey);
      if (res.success) {
        extractedMessage = res.message;
        isEncrypted = !!res.isEncrypted;
      } else {
        if (res.isEncrypted) {
          isEncrypted = true;
          infoMessage = 'Pesan dalam citra ini dilindungi kata sandi. Masukkan kunci sandi lalu tekan Dekode.';
        } else if (isManual) {
          errorMessage = res.message || 'Gagal mengekstrak pesan tersembunyi.';
        } else {
          infoMessage = 'Masukkan kata sandi jika pesan steganografi dienkripsi, lalu tekan Dekode.';
        }
      }
    } catch (err: any) {
      if (isManual) {
        errorMessage = err?.message || 'Kesalahan saat decoding steganografi.';
      }
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

<Dialog
  {open}
  title="Stego Photo Decoder"
  {onclose}
  maxWidth="460px"
>
  <div class="stego-dialog-content">
    <p class="dialog-sub">Ekstraksi pesan rahasia yang tersembunyi di dalam foto</p>

    <!-- Carrier Image Preview -->
    <div class="carrier-preview-box">
      {#if imageSrc && !imgError}
        <img
          src={imageSrc}
          alt="Carrier Stego"
          class="carrier-img"
          onerror={() => (imgError = true)}
        />
      {:else}
        <div class="carrier-placeholder">
          <Image size={40} weight="duotone" color="var(--text-secondary)" />
          <span style="font-size: 0.82rem; color: var(--text-secondary); margin-top: 4px;">Citra Stego Aman</span>
        </div>
      {/if}
      <span class="carrier-name-tag">{carrierName}</span>
    </div>

    <!-- Passkey input if encrypted or requested -->
    <div class="passkey-input-row">
      <label for="stego-passkey-input" class="passkey-label">
        Kunci Sandi Foto (Opsional):
      </label>
      <div class="input-with-button">
        <input
          id="stego-passkey-input"
          type="password"
          class="passkey-input"
          placeholder="Masukkan kata sandi dekripsi..."
          bind:value={passkey}
          onkeydown={(e) => { if (e.key === 'Enter') runDecode(true); }}
        />
        <button
          type="button"
          class="btn-decode-action"
          disabled={isDecoding}
          onclick={() => runDecode(true)}
        >
          {#if isDecoding}
            <span>Menganalisis...</span>
          {:else}
            <LockKeyOpen size={16} weight="bold" />
            <span>Dekode</span>
          {/if}
        </button>
      </div>
    </div>

    <!-- Info Hint / Notice -->
    {#if infoMessage}
      <div class="info-banner">
        <span style="display: flex; align-items: center; gap: 6px;">
          <Info size={16} weight="bold" color="#1E40AF" />
          <span>{infoMessage}</span>
        </span>
      </div>
    {/if}

    <!-- Error Alert -->
    {#if errorMessage}
      <div class="error-banner">
        <span style="display: flex; align-items: center; gap: 6px;">
          <WarningCircle size={16} weight="bold" color="#DC2626" />
          <span>{errorMessage}</span>
        </span>
      </div>
    {/if}

    <!-- Extracted Result Box -->
    {#if extractedMessage}
      <div class="result-box">
        <div class="result-header">
          <span class="result-badge" style="display: flex; align-items: center; gap: 4px;">
            <ShieldCheck size={16} weight="bold" />
            <span>PESAN RAHASIA TERBUKA {isEncrypted ? '(TERDEKRIPSI)' : ''}</span>
          </span>
          <button
            type="button"
            class="btn-copy-secret"
            onclick={handleCopy}
          >
            {#if copied}
              <Check size={14} weight="bold" />
              <span>Disalin!</span>
            {:else}
              <Copy size={14} weight="bold" />
              <span>Salin</span>
            {/if}
          </button>
        </div>
        <p class="extracted-text">{extractedMessage}</p>
      </div>
    {/if}

    <!-- Forensic Notice -->
    <div class="forensic-notice-pill" style="display: flex; align-items: center; gap: 6px;">
      <ShieldCheck size={16} weight="bold" color="var(--brand-teal, #0D9488)" />
      <span>Pesan ini disematkan langsung ke dalam struktur piksel foto (Stego Photo) dan tidak dapat dideteksi oleh pemindai lalu lintas jaringan.</span>
    </div>
  </div>
</Dialog>

<style>
  .stego-dialog-content {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  .dialog-sub {
    font-size: 0.78rem;
    color: var(--text-secondary);
    margin: -6px 0 14px 0;
  }

  .carrier-preview-box {
    position: relative;
    width: 100%;
    max-height: 200px;
    background-color: #0F172A;
    border-radius: var(--radius-md);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 14px;
    border: 1px solid var(--border-light);
  }

  .carrier-img {
    width: 100%;
    height: 100%;
    max-height: 200px;
    object-fit: contain;
  }

  .carrier-placeholder {
    padding: 30px;
    color: var(--text-secondary);
    font-size: 0.9rem;
    display: flex;
    flex-direction: column;
    align-items: center;
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
    color: var(--text-secondary);
  }

  .input-with-button {
    display: flex;
    gap: 6px;
  }

  .passkey-input {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    padding: 8px 12px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-light);
    background-color: var(--bg-card);
    font-size: 0.85rem;
    font-family: var(--font-active);
    color: var(--text-primary);
    outline: none;
  }

  .passkey-input:focus {
    border-color: var(--brand-blue);
  }

  .btn-decode-action {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    padding: 8px 16px;
    border-radius: var(--radius-md);
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.82rem;
    font-weight: 700;
    font-family: var(--font-active);
    border: none;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .btn-decode-action:hover {
    background-color: #1D4ED8;
  }

  .info-banner {
    background-color: #EFF6FF;
    border: 1px solid #BFDBFE;
    border-radius: var(--radius-md);
    padding: 8px 12px;
    font-size: 0.78rem;
    color: #1E40AF;
    margin-bottom: 10px;
  }

  .error-banner {
    background-color: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: var(--radius-md);
    padding: 8px 12px;
    font-size: 0.78rem;
    color: #DC2626;
    margin-bottom: 10px;
  }

  .result-box {
    background-color: var(--brand-teal-light, rgba(13, 148, 136, 0.08));
    border: 1.5px solid var(--brand-teal, #0D9488);
    border-radius: var(--radius-md);
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
    color: var(--brand-teal, #0D9488);
    letter-spacing: 0.5px;
  }

  .btn-copy-secret {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: transparent;
    border: 1px solid var(--brand-teal, #0D9488);
    color: var(--brand-teal, #0D9488);
    font-size: 0.76rem;
    font-weight: 700;
    font-family: var(--font-active);
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    cursor: pointer;
  }

  .extracted-text {
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 4px 0 0 0;
    line-height: 1.45;
    background: var(--bg-card);
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    border: 1px solid rgba(13, 148, 136, 0.2);
    word-break: break-word;
  }

  .forensic-notice-pill {
    font-size: 0.72rem;
    color: var(--text-secondary);
    line-height: 1.35;
    background-color: var(--bg-primary);
    padding: 8px 12px;
    border-radius: var(--radius-md);
    text-align: center;
    margin-top: 4px;
  }
</style>
