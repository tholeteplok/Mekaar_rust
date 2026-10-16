<script lang="ts">
  import { mekaarStore } from '../stores';
  import SosButton from '../components/SosButton.svelte';

  interface Props {
    isSetup?: boolean;
    onComplete?: () => void;
  }

  let { isSetup = false, onComplete }: Props = $props();

  let pin = $state('');
  let firstPin = $state('');
  let isConfirming = $state(false);
  let obscurePin = $state(true);
  let hasError = $state(false);
  let errorMessage = $state('');
  let isShaking = $state(false);

  const PIN_LENGTH = 6;

  const headerTitle = $derived.by(() => {
    if (isSetup) {
      return isConfirming ? 'Konfirmasi PIN Baru' : 'Buat PIN Baru';
    }
    return 'Buka Kunci Aplikasi';
  });

  const headerSubtitle = $derived.by(() => {
    if (isSetup) {
      return isConfirming
        ? 'Masukkan PIN sekali lagi untuk konfirmasi.'
        : 'Buat PIN 6 digit untuk mengamankan aplikasi.';
    }
    return 'Masukkan PIN 6 digit Anda untuk masuk.';
  });

  function handleKeyPress(key: string) {
    if (key === '⌫') {
      if (pin.length > 0) {
        pin = pin.slice(0, -1);
        hasError = false;
        errorMessage = '';
      }
      return;
    }

    if (pin.length >= PIN_LENGTH) return;

    pin += key;
    hasError = false;
    errorMessage = '';

    if (pin.length === PIN_LENGTH) {
      processPin();
    }
  }

  async function processPin() {
    if (isSetup) {
      if (!isConfirming) {
        // Step 1 done -> Save firstPin and ask for confirmation
        firstPin = pin;
        pin = '';
        isConfirming = true;
      } else {
        // Step 2 done -> Verify confirmation
        if (pin === firstPin) {
          if (navigator.vibrate) navigator.vibrate(50);
          await mekaarStore.setupPins(pin);
          if (onComplete) {
            onComplete();
          } else {
            mekaarStore.advanceFtue('authenticated');
          }
        } else {
          hasError = true;
          errorMessage = 'PIN konfirmasi tidak cocok. Silakan buat ulang.';
          triggerShake();
          setTimeout(() => {
            pin = '';
            firstPin = '';
            isConfirming = false;
          }, 800);
        }
      }
    } else {
      // Normal / Duress Unlock verification via Rust unlock_vault IPC
      const res = await mekaarStore.unlockVault(pin);
      if (res.success) {
        if (navigator.vibrate) navigator.vibrate(50);
        mekaarStore.advanceFtue('authenticated');
        mekaarStore.checkServerHealth('tailscale');
      } else {
        hasError = true;
        errorMessage = res.message || 'PIN tidak sesuai. Silakan coba lagi.';
        triggerShake();
        setTimeout(() => {
          pin = '';
        }, 600);
      }
    }
  }

  function triggerShake() {
    isShaking = true;
    if (navigator.vibrate) navigator.vibrate([80, 40, 80]);
    setTimeout(() => {
      isShaking = false;
    }, 600);
  }

  function handleForgotPin() {
    if (isSetup) {
      pin = '';
      firstPin = '';
      isConfirming = false;
    } else {
      errorMessage = 'Petunjuk: PIN tersimpan di brankas lokal perangkat Anda.';
      hasError = true;
      triggerShake();
    }
  }

  function handleSosTrigger() {
    mekaarStore.vaultUnlocked = true;
    mekaarStore.pinVerified = true;
    mekaarStore.setTab('sos');
  }
</script>

<div class="pin-screen">
  <!-- Top Section: Shield Icon & Title -->
  <div class="top-section">
    <div class="shield-badge-glow">
      <svg width="36" height="36" viewBox="0 0 24 24" fill="#136CFC">
        <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 6c1.66 0 3 1.34 3 3 0 1.13-.63 2.11-1.56 2.61L14 16h-4v-3.39C9.07 12.11 8.5 11.13 8.5 10c0-1.66 1.34-3 3-3z"/>
      </svg>
    </div>

    <h1 class="pin-title">{headerTitle}</h1>
    <p class="pin-subtitle">{headerSubtitle}</p>
  </div>

  <!-- Middle Section: Obscure Toggle & 6 Pin Circles -->
  <div class="indicator-section {isShaking ? 'shake-anim' : ''}">
    <button
      type="button"
      class="eye-toggle-btn"
      onclick={() => (obscurePin = !obscurePin)}
      aria-label={obscurePin ? 'Tampilkan PIN' : 'Sembunyikan PIN'}
    >
      {#if obscurePin}
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
          <line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
      {:else}
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
          <circle cx="12" cy="12" r="3"></circle>
        </svg>
      {/if}
    </button>

    <div class="pin-dots-row">
      {#each Array(PIN_LENGTH) as _, i}
        {@const isFilled = pin.length > i}
        {@const isCurrent = pin.length === i}
        <div
          class="pin-dot {isFilled ? 'filled' : ''} {isCurrent ? 'current' : ''} {hasError ? 'error' : ''}"
        >
          {#if isFilled && !obscurePin}
            <span class="revealed-digit">{pin[i]}</span>
          {/if}
        </div>
      {/each}
    </div>

    {#if errorMessage}
      <div class="error-text">{errorMessage}</div>
    {/if}
  </div>

  <!-- Keypad Section: 74px Circular Buttons -->
  <div class="keypad-grid">
    {#each ['1', '2', '3', '4', '5', '6', '7', '8', '9'] as digit}
      <button
        type="button"
        class="keypad-circle-btn"
        onclick={() => handleKeyPress(digit)}
      >
        {digit}
      </button>
    {/each}

    <!-- Bottom Row: Lupa PIN? (atau Ulangi saat setup) / 0 / ⌫ -->
    <button
      type="button"
      class="keypad-text-btn"
      onclick={handleForgotPin}
    >
      {#if isSetup}
        {isConfirming ? 'Ulangi' : ''}
      {:else}
        Lupa<br />PIN?
      {/if}
    </button>

    <button
      type="button"
      class="keypad-circle-btn"
      onclick={() => handleKeyPress('0')}
    >
      0
    </button>

    <button
      type="button"
      class="keypad-icon-btn"
      onclick={() => handleKeyPress('⌫')}
      aria-label="Hapus Digit"
    >
      <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z"></path>
        <line x1="18" y1="9" x2="12" y2="15"></line>
        <line x1="12" y1="9" x2="18" y2="15"></line>
      </svg>
    </button>
  </div>

  <!-- Bottom Section: SOS Button & Label -->
  <div class="bottom-sos-bar">
    <SosButton size={54} onTrigger={handleSosTrigger} />
    <span class="sos-bar-label">Tahan 2 dtk untuk darurat</span>
  </div>
</div>

<style>
  .pin-screen {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    padding: calc(48px + env(safe-area-inset-top, 0px)) 24px calc(28px + env(safe-area-inset-bottom, 0px));
    background-color: var(--bg-primary);
    box-sizing: border-box;
    user-select: none;
    -webkit-user-select: none;
  }

  .top-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .shield-badge-glow {
    width: 76px;
    height: 76px;
    border-radius: 50%;
    background-color: #DBEAFE;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 8px 24px rgba(19, 108, 252, 0.16);
    margin-bottom: 24px;
  }

  .pin-title {
    font-family: var(--font-sans);
    font-size: 1.6rem;
    font-weight: 800;
    color: var(--text-primary);
    margin-bottom: 8px;
    letter-spacing: -0.2px;
  }

  .pin-subtitle {
    font-size: 0.95rem;
    color: var(--text-secondary);
    line-height: 1.35;
    max-width: 320px;
  }

  .indicator-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 12px 0;
  }

  .eye-toggle-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 6px 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
    transition: color 0.15s ease;
  }

  .eye-toggle-btn:active {
    color: var(--brand-blue);
  }

  .pin-dots-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 14px;
  }

  .pin-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid #93C5FD;
    background-color: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .pin-dot.current {
    border-color: var(--brand-blue);
    background-color: rgba(19, 108, 252, 0.12);
    transform: scale(1.1);
  }

  .pin-dot.filled {
    border-color: var(--brand-blue);
    background-color: var(--brand-blue);
    box-shadow: 0 2px 8px rgba(19, 108, 252, 0.35);
  }

  .pin-dot.error {
    border-color: var(--sos-coral) !important;
    background-color: var(--sos-coral) !important;
    box-shadow: 0 2px 8px rgba(255, 93, 93, 0.45);
  }

  .revealed-digit {
    color: #FFFFFF;
    font-size: 11px;
    font-weight: 800;
  }

  .error-text {
    margin-top: 10px;
    font-size: 0.85rem;
    color: var(--sos-coral);
    font-weight: 600;
  }

  /* Keypad Grid: 3 columns, 74px circle buttons */
  .keypad-grid {
    display: grid;
    grid-template-columns: repeat(3, 74px);
    grid-gap: 18px 24px;
    justify-content: center;
    margin: 8px 0;
  }

  .keypad-circle-btn {
    width: 74px;
    height: 74px;
    border-radius: 50%;
    background-color: #EFF6FF;
    border: none;
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 1.65rem;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.12s ease, background-color 0.12s ease;
    box-shadow: 0 2px 6px rgba(19, 108, 252, 0.05);
  }

  .keypad-circle-btn:active {
    transform: scale(0.92);
    background-color: #DBEAFE;
  }

  .keypad-text-btn {
    width: 74px;
    height: 74px;
    border-radius: 50%;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 600;
    line-height: 1.25;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    transition: color 0.12s ease;
  }

  .keypad-text-btn:active {
    color: var(--brand-blue);
  }

  .keypad-icon-btn {
    width: 74px;
    height: 74px;
    border-radius: 50%;
    background: transparent;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.12s ease, color 0.12s ease;
  }

  .keypad-icon-btn:active {
    transform: scale(0.92);
    color: var(--brand-blue);
  }

  .bottom-sos-bar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 14px;
    margin-top: 10px;
  }

  .sos-bar-label {
    color: var(--sos-coral);
    font-size: 0.88rem;
    font-weight: 600;
    letter-spacing: 0.1px;
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    20%, 60% { transform: translateX(-10px); }
    40%, 80% { transform: translateX(10px); }
  }

  .shake-anim {
    animation: shake 0.5s ease-in-out;
  }
</style>
