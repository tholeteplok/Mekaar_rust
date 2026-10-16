<script lang="ts">
  import { mekaarStore } from '../stores';
  import { generateContactPayload, generateQrSvg } from '../services/qr';
  import Dialog from './Dialog.svelte';

  let { open = false, onclose, onScanClick }: {
    open: boolean;
    onclose: () => void;
    onScanClick?: () => void;
  } = $props();

  let qrSvg = $state('');
  let copied = $state(false);
  let copyTimeout: ReturnType<typeof setTimeout> | null = null;

  const qrPayload = $derived(
    generateContactPayload(
      mekaarStore.currentUser.id,
      mekaarStore.currentUser.username,
      mekaarStore.currentUser.displayName,
      mekaarStore.currentUser.publicKeyFingerprint
    )
  );

  $effect(() => {
    if (open && qrPayload) {
      generateQrSvg(qrPayload, {
        size: 210,
        margin: 2,
        darkColor: '#0F172A',
        lightColor: '#FFFFFF'
      }).then((svg) => {
        qrSvg = svg;
      }).catch((err) => {
        console.error('Failed to generate QR SVG:', err);
      });
    }
  });

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(qrPayload);
      copied = true;
      if (copyTimeout) clearTimeout(copyTimeout);
      copyTimeout = setTimeout(() => {
        copied = false;
      }, 2500);
    } catch (_) {
      copied = true;
      setTimeout(() => (copied = false), 2000);
    }
  }

  function handleScan() {
    onclose();
    if (onScanClick) onScanClick();
  }
</script>

<Dialog
  {open}
  title="Kode QR Profil"
  {onclose}
  maxWidth="420px"
>
  <div class="qr-dialog-content">
    <p class="dialog-desc">
      Tunjukkan kode QR ini ke teman Anda agar dapat langsung terhubung dalam obrolan terenkripsi Mekaar Aegis.
    </p>

    <!-- Telegram-Style Overlapping QR Card -->
    <div class="qr-card-stack">
      <!-- Floating Avatar on Top -->
      <div class="floating-avatar-wrap">
        <div
          class="floating-avatar"
          style="background-color: {mekaarStore.currentUser.avatarColor || '#BFDBFE'};"
        >
          {mekaarStore.currentUser.displayName.charAt(0).toUpperCase()}
        </div>
      </div>

      <!-- Inner White Card -->
      <div class="inner-qr-card">
        <!-- QR Matrix Box -->
        <div class="qr-box">
          {#if qrSvg}
            {@html qrSvg}
          {:else}
            <div class="qr-loading-placeholder">
              <span class="spinner-dot"></span>
              <span>Menghasilkan kode QR...</span>
            </div>
          {/if}
        </div>

        <!-- Profile Details -->
        <h4 class="profile-name">{mekaarStore.currentUser.displayName}</h4>
        <p class="profile-handle">{mekaarStore.currentUser.username}</p>

        <!-- Aegis Protection Badge -->
        <div class="aegis-badge">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z"/>
          </svg>
          <span>Mekaar Aegis E2EE</span>
        </div>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="action-buttons-wrap">
      <!-- Primary Action: Copy Link -->
      <button
        type="button"
        class="btn-primary-share {copied ? 'btn-copied' : ''}"
        onclick={handleCopy}
      >
        {#if copied}
          ✓ Tautan Profil Disalin!
        {:else}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
          </svg>
          Salin & Bagikan Tautan QR
        {/if}
      </button>

      <!-- Secondary Action: Scan Friend QR -->
      <button
        type="button"
        class="btn-secondary-scan"
        onclick={handleScan}
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="7" height="7"></rect>
          <rect x="14" y="3" width="7" height="7"></rect>
          <rect x="14" y="14" width="7" height="7"></rect>
          <rect x="3" y="14" width="7" height="7"></rect>
        </svg>
        Pindai Kode QR Teman
      </button>
    </div>
  </div>
</Dialog>

<style>
  .qr-dialog-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    position: relative;
  }

  .dialog-desc {
    font-size: 0.82rem;
    color: var(--text-secondary);
    line-height: 1.45;
    margin: 0 0 24px 0;
  }

  /* Telegram Overlapping Card Stack */
  .qr-card-stack {
    position: relative;
    width: 100%;
    max-width: 320px;
    margin-top: 24px;
    margin-bottom: 24px;
  }

  .floating-avatar-wrap {
    position: absolute;
    top: -28px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 2;
    padding: 4px;
    background-color: var(--bg-card);
    border-radius: 50%;
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.12);
  }

  .floating-avatar {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    font-weight: 800;
    color: #1E293B;
  }

  .inner-qr-card {
    background-color: var(--bg-primary);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-light);
    padding: 44px 20px 20px 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
  }

  .qr-box {
    background: #FFFFFF;
    padding: 12px;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 220px;
    height: 220px;
    box-sizing: border-box;
  }

  .qr-box :global(svg) {
    width: 100%;
    height: 100%;
    display: block;
  }

  .qr-loading-placeholder {
    font-size: 0.8rem;
    color: var(--text-secondary);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .profile-name {
    font-size: 1.15rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
    margin: 16px 0 2px 0;
  }

  .profile-handle {
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--brand-blue);
    margin: 0 0 10px 0;
  }

  .aegis-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background-color: var(--brand-teal-light, rgba(16, 185, 129, 0.12));
    color: var(--brand-teal, #059669);
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.3px;
  }

  /* Action Buttons */
  .action-buttons-wrap {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .btn-primary-share {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    height: 48px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.92rem;
    font-weight: 700;
    font-family: var(--font-active);
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    cursor: pointer;
    box-shadow: 0 4px 12px rgba(2, 132, 199, 0.25);
    transition: all 0.15s ease;
  }

  .btn-primary-share:hover {
    filter: brightness(1.05);
  }

  .btn-primary-share:active {
    transform: scale(0.98);
  }

  .btn-primary-share.btn-copied {
    background-color: #059669;
    box-shadow: 0 4px 12px rgba(5, 150, 105, 0.25);
  }

  .btn-secondary-scan {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    height: 46px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1.5px solid var(--border-light);
    color: var(--text-primary);
    font-size: 0.88rem;
    font-weight: 700;
    font-family: var(--font-active);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-secondary-scan:hover {
    background-color: var(--bg-primary);
    border-color: var(--text-secondary);
  }

  .btn-secondary-scan:active {
    transform: scale(0.98);
  }
</style>
