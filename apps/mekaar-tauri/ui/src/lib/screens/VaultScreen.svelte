<script lang="ts">
  import { mekaarStore } from '../stores';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let mode = $state<'vault' | 'pin' | 'onboarding'>('vault');
  let passphrase = $state('');
  let pinInput = $state('');
  let newUsername = $state(mekaarStore.currentUser.username);
  let newDisplayName = $state(mekaarStore.currentUser.displayName);
  let errorMsg = $state('');

  function handleUnlockVault() {
    errorMsg = '';
    if (passphrase.trim().length < 4) {
      errorMsg = 'Masukkan passphrase brankas minimal 4 karakter.';
      return;
    }
    mekaarStore.vaultUnlocked = true;
    mekaarStore.pinVerified = true;
    mekaarStore.checkServerHealth();
  }

  function handleVerifyPin() {
    errorMsg = '';
    if (pinInput !== mekaarStore.pinCode) {
      errorMsg = 'Kode PIN tidak sesuai (Default: 123456).';
      return;
    }
    mekaarStore.pinVerified = true;
    mekaarStore.vaultUnlocked = true;
  }

  function saveOnboardingProfile() {
    mekaarStore.currentUser.username = newUsername.startsWith('@') ? newUsername : `@${newUsername}`;
    mekaarStore.currentUser.displayName = newDisplayName || 'Pengguna Mekaar';
    mekaarStore.onboardingComplete = true;
    mekaarStore.vaultUnlocked = true;
    mekaarStore.pinVerified = true;
  }
</script>

<div class="vault-screen">
  <div class="vault-card sketch-card">
    <div style="display: flex; justify-content: center; margin-bottom: 12px;">
      <SketchAvatar name={mekaarStore.currentUser.displayName} color="#FDE68A" size={68} online={true} />
    </div>

    <h2 style="font-family: var(--font-crayon); text-align: center; font-size: 1.45rem; margin-bottom: 4px;">
      Mekaar Rust • Secure Vault
    </h2>
    <p style="text-align: center; font-size: 0.85rem; color: var(--text-secondary); margin-bottom: 16px;">
      Identitas Ed25519 & SQLite WAL E2EE • Relay Tailscale <code>100.101.96.120:8085</code>
    </p>

    <div class="mode-switcher">
      <button type="button" class:active={mode === 'vault'} onclick={() => (mode = 'vault')}>🔐 Passphrase Vault</button>
      <button type="button" class:active={mode === 'pin'} onclick={() => (mode = 'pin')}>🔢 Kunci PIN</button>
      <button type="button" class:active={mode === 'onboarding'} onclick={() => (mode = 'onboarding')}>✨ Profil & 2FA</button>
    </div>

    {#if mode === 'vault'}
      <div style="display: flex; flex-direction: column; gap: 12px;">
        <input
          type="password"
          class="sketch-input"
          placeholder="Masukkan Passphrase Vault..."
          bind:value={passphrase}
          onkeydown={(e) => e.key === 'Enter' && handleUnlockVault()}
        />
        <Button variant="primary" fullWidth onclick={handleUnlockVault}>Buka Brankas & Sambungkan Relay</Button>
      </div>
    {:else if mode === 'pin'}
      <div style="display: flex; flex-direction: column; gap: 12px;">
        <input
          type="password"
          maxlength="6"
          class="sketch-input"
          style="text-align: center; letter-spacing: 0.35em; font-size: 1.2rem; font-weight: 800;"
          placeholder="••••••"
          bind:value={pinInput}
          onkeydown={(e) => e.key === 'Enter' && handleVerifyPin()}
        />
        <Button variant="primary" fullWidth onclick={handleVerifyPin}>Verifikasi PIN (123456)</Button>
      </div>
    {:else}
      <div style="display: flex; flex-direction: column; gap: 10px;">
        <label style="font-size: 0.82rem; font-weight: 700;">
          Nama Tampilan
          <input type="text" class="sketch-input" bind:value={newDisplayName} style="margin-top: 4px;" />
        </label>
        <label style="font-size: 0.82rem; font-weight: 700;">
          Username Unik
          <input type="text" class="sketch-input" bind:value={newUsername} style="margin-top: 4px;" />
        </label>
        <div class="sketch-card" style="padding: 8px 10px; background: var(--bg-secondary); font-size: 0.78rem;">
          Sidik Jari Kunci Publik: <strong>{mekaarStore.currentUser.publicKeyFingerprint}</strong>
        </div>
        <Button variant="primary" fullWidth onclick={saveOnboardingProfile}>Simpan Identitas & Masuk</Button>
      </div>
    {/if}

    {#if errorMsg}
      <p style="color: var(--accent-danger); font-size: 0.84rem; font-weight: 700; margin-top: 10px; text-align: center;">
        {errorMsg}
      </p>
    {/if}
  </div>
</div>

<style>
  .vault-screen {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    background: var(--bg-primary);
  }
  .vault-card {
    width: min(440px, 100%);
    padding: 26px 24px;
  }
  .mode-switcher {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
    margin-bottom: 16px;
  }
  .mode-switcher button {
    padding: 7px 6px;
    font-size: 0.78rem;
    font-weight: 700;
    background: var(--bg-secondary);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 10px;
    cursor: pointer;
  }
  .mode-switcher button.active {
    background: var(--accent-secondary);
    box-shadow: var(--sketch-shadow-sm);
  }
</style>
