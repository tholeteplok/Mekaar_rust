<script lang="ts">
  import { vaultStore } from '../stores';
  import { Button, Card } from '../components';

  interface Props {
    onunlocked?: () => void;
  }

  let { onunlocked }: Props = $props();

  const digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

  // Reaktif: saat status vaultStore.isLocked menjadi false, panggil onunlocked
  $effect(() => {
    if (!vaultStore.isLocked && onunlocked) {
      onunlocked();
    }
  });

  async function handleDigit(d: string) {
    await vaultStore.enterDigit(d);
  }

  async function handleActionSubmit() {
    await vaultStore.submitPin();
  }
</script>

<div class="vault-screen">
  <div class="vault-container">
    <!-- Vault Shield Icon & Header -->
    <div class="vault-header">
      <div class="shield-badge">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
        </svg>
      </div>
      <h1 class="vault-title">MEKAAR Secure Vault</h1>
      <p class="vault-subtitle">Masukkan PIN Master Anda untuk membuka database SQLCipher 256-bit.</p>
    </div>

    <!-- PIN Dots Indicator -->
    <div class="pin-indicator-row">
      {#each Array(6) as _, i}
        <div
          class="pin-dot"
          class:filled={i < vaultStore.pinInput.length}
          class:has-error={!!vaultStore.errorMessage}
        ></div>
      {/each}
    </div>

    {#if vaultStore.errorMessage}
      <div class="vault-error-message">
        {vaultStore.errorMessage}
      </div>
    {/if}

    <!-- Numeric Keypad Grid -->
    <div class="keypad-grid">
      {#each digits as d}
        <button
          type="button"
          class="keypad-btn"
          onclick={() => handleDigit(d)}
        >
          {d}
        </button>
      {/each}
      <button
        type="button"
        class="keypad-btn action-key"
        onclick={() => vaultStore.clearPin()}
      >
        C
      </button>
      <button
        type="button"
        class="keypad-btn"
        onclick={() => handleDigit('0')}
      >
        0
      </button>
      <button
        type="button"
        class="keypad-btn action-key"
        aria-label="Hapus Digit"
        onclick={() => vaultStore.deleteDigit()}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z"></path>
          <line x1="18" y1="9" x2="12" y2="15"></line>
          <line x1="12" y1="9" x2="18" y2="15"></line>
        </svg>
      </button>
    </div>

    <!-- Bottom Security Notice -->
    <Card variant="subtle" padding="sm">
      <div class="vault-footnote">
        <span class="lock-icon">🔒</span>
        <span>Dilindungi enkripsi memori Zeroize & Deteksi Duress Decoy PIN.</span>
      </div>
    </Card>
  </div>
</div>

<style>
  .vault-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: calc(100vh - 80px);
    padding: var(--space-4);
  }

  .vault-container {
    max-width: 380px;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-6);
  }

  .vault-header {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
  }

  .shield-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 64px;
    height: 64px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-light-blue);
    color: var(--brand-blue);
    margin-bottom: var(--space-2);
  }

  .shield-badge svg {
    width: 32px;
    height: 32px;
  }

  .vault-title {
    font-size: var(--font-h1-size);
    font-weight: 700;
    color: var(--text-primary);
  }

  .vault-subtitle {
    font-size: var(--font-body-size);
    color: var(--text-secondary);
    line-height: var(--font-body-line);
  }

  .pin-indicator-row {
    display: flex;
    gap: var(--space-3);
    margin: var(--space-2) 0;
  }

  .pin-dot {
    width: 16px;
    height: 16px;
    border-radius: var(--radius-pill);
    border: 2px solid var(--border-subtle);
    background-color: transparent;
    transition: all var(--motion-fast);
  }

  .pin-dot.filled {
    background-color: var(--brand-blue);
    border-color: var(--brand-blue);
    transform: scale(1.15);
  }

  .pin-dot.has-error {
    border-color: var(--sos-coral);
    background-color: var(--sos-coral);
  }

  .vault-error-message {
    font-size: var(--font-label-size);
    font-weight: 600;
    color: var(--sos-coral);
    margin-top: -8px;
  }

  .keypad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-3);
    width: 100%;
    max-width: 300px;
  }

  .keypad-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 68px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background-color: var(--surface-card);
    color: var(--text-primary);
    font-size: var(--font-h1-size);
    font-weight: 700;
    cursor: pointer;
    outline: none;
    user-select: none;
    box-shadow: var(--shadow-soft);
    transition: all var(--motion-fast);
  }

  .keypad-btn:hover {
    background-color: var(--surface-subtle);
    border-color: var(--brand-blue);
  }

  .keypad-btn:active {
    transform: scale(0.94);
    background-color: var(--brand-light-blue);
  }

  .action-key {
    font-size: var(--font-h2-size);
    color: var(--text-secondary);
  }

  .action-key svg {
    width: 24px;
    height: 24px;
  }

  .vault-footnote {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--font-caption-size);
    color: var(--text-secondary);
    line-height: var(--font-caption-line);
  }

  .lock-icon {
    font-size: 14px;
  }
</style>
