<script lang="ts">
  import { mekaarStore, TAILSCALE_HTTP, TAILSCALE_WS } from '../stores';
  import MikaMascot from '../components/MikaMascot.svelte';

  let displayName = $state('');
  let username = $state('');
  let isSyncing = $state(false);
  let syncStatus = $state<'idle' | 'syncing' | 'success' | 'error'>('idle');
  let errorMessage = $state('');

  const cleanUsername = $derived(
    username.replace(/^@/, '').trim().toLowerCase()
  );

  async function handleRegisterAndSync() {
    errorMessage = '';
    if (!displayName.trim()) {
      errorMessage = 'Mohon masukkan nama tampilan Anda.';
      return;
    }
    if (cleanUsername.length < 3) {
      errorMessage = 'Username minimal 3 karakter alfanumerik.';
      return;
    }
    if (!/^[a-z0-9_]+$/.test(cleanUsername)) {
      errorMessage = 'Username hanya boleh berisi huruf kecil, angka, dan garis bawah (_).';
      return;
    }

    isSyncing = true;
    syncStatus = 'syncing';

    try {
      // 1. Mandatory Sync to teplok-pc relay node
      await mekaarStore.registerAndSyncNewUser({
        displayName: displayName.trim(),
        username: `@${cleanUsername}`
      });

      syncStatus = 'success';
      setTimeout(() => {
        mekaarStore.advanceFtue('authenticated');
      }, 700);
    } catch (err: any) {
      syncStatus = 'error';
      errorMessage = err?.message || 'Gagal sinkronisasi ke teplok-pc. Pastikan jaringan terhubung.';
      isSyncing = false;
    }
  }
</script>

<div class="setup-identity-container">
  <!-- Top Stage: Mascot & Welcome -->
  <div class="top-hero">
    <MikaMascot pose="happy" size={120} alt="Mika Menyambut" />
    <h1 class="hero-title">Buat Identitas Anda</h1>
    <p class="hero-subtitle">
      Tentukan nama tampilan dan username unik untuk sinkronisasi enkripsi ke teplok-pc.
    </p>
  </div>

  <!-- Form Stage -->
  <div class="form-card clean-card with-ticks">
    <span class="pencil-tick tick-bl">⌞</span>
    <span class="pencil-tick tick-tr">⌝</span>

    <div class="input-group">
      <label for="displayName" class="input-label">Nama Tampilan</label>
      <input
        id="displayName"
        type="text"
        class="text-input"
        placeholder="Contoh: Budi Santoso"
        bind:value={displayName}
        disabled={isSyncing}
      />
    </div>

    <div class="input-group">
      <label for="username" class="input-label">Username Unik</label>
      <div class="handle-input-wrap">
        <span class="prefix">@</span>
        <input
          id="username"
          type="text"
          class="text-input handle-input"
          placeholder="username"
          bind:value={username}
          disabled={isSyncing}
        />
      </div>
      <span class="input-hint">Digunakan kontak lain untuk menemukan kunci publik Anda.</span>
    </div>

    <!-- Mandatory Sync Badge Card -->
    <div class="sync-info-box">
      <div class="sync-header">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="var(--brand-blue)">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14h2v2h-2v-2zm0-10h2v8h-2V6z"/>
        </svg>
        <span class="sync-title">Mandatori Sinkronisasi Relay teplok-pc</span>
      </div>
      <p class="sync-desc">
        Identitas akan di-hash menggunakan <strong>Ed25519</strong> dan didaftarkan langsung ke direktori relay Tailscale <code>100.101.96.120:8085</code>.
      </p>
      <div class="relay-live-status">
        <span class="status-pulse-dot"></span>
        <span>Node Relay: {TAILSCALE_HTTP}</span>
      </div>
    </div>

    {#if errorMessage}
      <div class="error-banner">
        ⚠️ {errorMessage}
      </div>
    {/if}

    <button
      type="button"
      class="btn-primary submit-sync-btn"
      onclick={handleRegisterAndSync}
      disabled={isSyncing}
    >
      {#if isSyncing}
        <span class="spinner"></span>
        <span>Menyinkronkan ke teplok-pc...</span>
      {:else if syncStatus === 'success'}
        <span>✓ Berhasil Tersinkronisasi!</span>
      {:else}
        <span>Daftar & Sinkronkan ke teplok-pc ➔</span>
      {/if}
    </button>
  </div>
</div>

<style>
  .setup-identity-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: calc(24px + env(safe-area-inset-top, 0px)) 20px calc(24px + env(safe-area-inset-bottom, 0px));
    background-color: var(--bg-primary);
    box-sizing: border-box;
    user-select: none;
    -webkit-user-select: none;
  }

  .top-hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    margin-bottom: 20px;
    max-width: 340px;
  }

  .hero-title {
    font-size: 1.6rem;
    font-weight: 800;
    color: var(--text-primary);
    margin-top: 14px;
    margin-bottom: 6px;
    letter-spacing: -0.2px;
  }

  .hero-subtitle {
    font-size: 0.92rem;
    color: var(--text-secondary);
    line-height: 1.45;
  }

  .form-card {
    width: min(390px, 100%);
    padding: 22px 20px;
    background: #FFFFFF;
    border-radius: var(--radius-card);
    border: 0.75px solid var(--border-subtle);
    box-shadow: var(--shadow-card);
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-sizing: border-box;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .input-label {
    font-size: 0.86rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .text-input {
    width: 100%;
    height: 46px;
    padding: 0 14px;
    border-radius: 12px;
    border: 1px solid var(--border-subtle);
    background-color: #F8FAFC;
    font-size: 0.95rem;
    color: var(--text-primary);
    box-sizing: border-box;
    outline: none;
    transition: border-color 0.15s ease, background-color 0.15s ease;
  }

  .text-input:focus {
    border-color: var(--brand-blue);
    background-color: #FFFFFF;
  }

  .handle-input-wrap {
    display: flex;
    align-items: center;
    position: relative;
  }

  .prefix {
    position: absolute;
    left: 14px;
    color: var(--brand-blue);
    font-weight: 700;
    font-size: 1rem;
    pointer-events: none;
  }

  .handle-input {
    padding-left: 32px;
  }

  .input-hint {
    font-size: 0.76rem;
    color: var(--text-secondary);
  }

  .sync-info-box {
    background-color: #EFF6FF;
    border: 1px solid #BFDBFE;
    border-radius: 14px;
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .sync-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .sync-title {
    font-size: 0.86rem;
    font-weight: 700;
    color: #1E40AF;
  }

  .sync-desc {
    font-size: 0.78rem;
    color: #1E3A8A;
    line-height: 1.4;
  }

  .relay-live-status {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.74rem;
    font-family: var(--font-mono);
    color: #2563EB;
    margin-top: 4px;
  }

  .status-pulse-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background-color: #10B981;
    box-shadow: 0 0 6px rgba(16, 185, 129, 0.6);
  }

  .error-banner {
    padding: 10px 12px;
    border-radius: 10px;
    background-color: #FFF1F2;
    border: 1px solid #FECDD3;
    color: #BE123C;
    font-size: 0.84rem;
    font-weight: 600;
  }

  .submit-sync-btn {
    width: 100%;
    padding: 14px;
    font-size: 0.98rem;
    border-radius: var(--radius-pill);
    gap: 10px;
  }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-top-color: #FFFFFF;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
