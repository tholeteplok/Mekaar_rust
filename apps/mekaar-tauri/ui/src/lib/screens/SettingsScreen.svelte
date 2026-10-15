<script lang="ts">
  import { mekaarStore, TAILSCALE_HTTP, TAILSCALE_WS, LOCAL_HTTP, LOCAL_WS } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';
  import SecureMediaVaultDialog from '../components/SecureMediaVaultDialog.svelte';
  import StegoDecoderDialog from '../components/StegoDecoderDialog.svelte';

  let showFontModal = $state(false);
  let showTailscaleModal = $state(false);
  let showDuressModal = $state(false);
  let inputDuressPin = $state('');
  let duressError = $state('');
  let duressSuccess = $state('');
  let showMediaVault = $state(false);
  let showStegoDecoder = $state(false);
  let selectedStegoSrc = $state('');
  let selectedStegoName = $state('');

</script>

<AppBar title="Pengaturan" />

<div class="settings-page-container">
  <!-- Section: Akun -->
  <div class="settings-group">
    <span class="group-title">Akun</span>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="clean-card with-ticks account-card"
      onclick={() => mekaarStore.setTab('profile')}
    >
      <span class="pencil-tick tick-bl">⌞</span>
      <span class="pencil-tick tick-tr">⌝</span>
      <div class="account-card-left">
        <SketchAvatar
          name={mekaarStore.currentUser.displayName || 'Mika'}
          size={52}
          online={true}
        />
        <div class="account-info">
          <span class="account-name">{mekaarStore.currentUser.displayName}</span>
          <span class="account-email">servisio.apk@gmail.com</span>
        </div>
      </div>
      <div class="arrow-circle">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="9 18 15 12 9 6"></polyline>
        </svg>
      </div>
    </div>
  </div>

  <!-- Section: Tampilan -->
  <div class="settings-group">
    <span class="group-title">Tampilan</span>
    <div class="clean-card with-ticks list-tile-card">
      <span class="pencil-tick tick-bl">⌞</span>
      <span class="pencil-tick tick-tr">⌝</span>

      <!-- Tampilan & Font -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (showFontModal = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-purple">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M4 7V4h16v3M9 20h6M12 4v16"/>
            </svg>
          </div>
          <span class="item-title">Tampilan & Font</span>
        </div>
        <div class="item-right">
          <span class="item-value">{mekaarStore.fontFamily}</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Tema & Wallpaper Chat -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (mekaarStore.themeMode = mekaarStore.themeMode === 'light' ? 'dark' : 'light')}>
        <div class="item-left">
          <div class="item-icon-circle icon-blue">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
            </svg>
          </div>
          <span class="item-title">Tema & Wallpaper Chat</span>
        </div>
        <div class="item-right">
          <span class="item-value">{mekaarStore.themeMode === 'light' ? 'Mode Terang' : 'Mode Gelap'}</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Section: Privasi (Disembunyikan jika dalam Mode Duress / Paksaan) -->
  {#if !mekaarStore.isDuressActive}
    <div class="settings-group">
      <span class="group-title">Privasi & Mode Samaran</span>
      <div class="clean-card with-ticks list-tile-card">
        <span class="pencil-tick tick-bl">⌞</span>
        <span class="pencil-tick tick-tr">⌝</span>

        <!-- Proteksi Layar -->
        <div class="setting-item-row">
          <div class="item-left">
            <div class="item-icon-circle icon-teal">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#0D9488" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="5" y="2" width="14" height="20" rx="2" ry="2"></rect>
                <line x1="12" y1="18" x2="12.01" y2="18"></line>
              </svg>
            </div>
            <span class="item-title">Proteksi Layar</span>
          </div>
          <div class="item-right">
            <span class="item-value">{mekaarStore.privacyBlurOnUnfocus ? 'Aktif' : 'Nonaktif'}</span>
            <label class="switch-toggle">
              <input type="checkbox" bind:checked={mekaarStore.privacyBlurOnUnfocus} />
              <span class="slider"></span>
            </label>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Sembunyikan Notifikasi Darurat -->
        <div class="setting-item-row">
          <div class="item-left">
            <div class="item-icon-circle icon-orange">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#EA580C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="3"></circle>
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
              </svg>
            </div>
            <span class="item-title">Sembunyikan Notifikasi Darurat</span>
          </div>
          <div class="item-right">
            <span class="item-value">{mekaarStore.disguisedNotifications ? 'Aktif' : 'Nonaktif'}</span>
            <label class="switch-toggle">
              <input type="checkbox" bind:checked={mekaarStore.disguisedNotifications} />
              <span class="slider"></span>
            </label>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Terakhir Dilihat & Online -->
        <div class="setting-item-row">
          <div class="item-left">
            <div class="item-icon-circle icon-blue-subtle">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                <circle cx="12" cy="7" r="4"></circle>
              </svg>
            </div>
            <span class="item-title">Terakhir Dilihat & Online</span>
          </div>
          <div class="item-right">
            <span class="item-value">Kontak saya</span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Bukti Baca (Read Receipt) -->
        <div class="setting-item-row">
          <div class="item-left">
            <div class="item-icon-circle icon-lime">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#16A34A" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
            </div>
            <span class="item-title">Bukti Baca (Read Receipt)</span>
          </div>
          <div class="item-right">
            <span class="item-value">{mekaarStore.readReceipts ? 'Aktif' : 'Nonaktif'}</span>
            <label class="switch-toggle">
              <input type="checkbox" bind:checked={mekaarStore.readReceipts} />
              <span class="slider"></span>
            </label>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- PIN Paksaan (Duress PIN) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showDuressModal = true)}>
          <div class="item-left">
            <div class="item-icon-circle icon-orange">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#EA580C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                <path d="M12 8v4"/>
                <path d="M12 16h.01"/>
              </svg>
            </div>
            <div>
              <div class="item-title">PIN Paksaan (Duress Cloak)</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">Aktifkan mode darurat senyap jika dipaksa</div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value">{mekaarStore.duressPinCode ? 'Dikonfigurasi' : 'Belum diatur'}</span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Section: Server Relay teplok-pc & Forensik -->
  <div class="settings-group">
    <span class="group-title">Sistem & Keamanan Tingkat Lanjut</span>
    <div class="clean-card with-ticks list-tile-card">
      <span class="pencil-tick tick-bl">⌞</span>
      <span class="pencil-tick tick-tr">⌝</span>

      <!-- teplok-pc Relay Node -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (showTailscaleModal = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-blue">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="2" y1="12" x2="22" y2="12"></line>
              <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
            </svg>
          </div>
          <div>
            <div class="item-title">Node teplok-pc (Tailscale)</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">100.101.96.120:8085 • {mekaarStore.serverStatus.latencyMs}ms</div>
          </div>
        </div>
        <div class="item-right">
          <span class="aegis-badge">Terhubung</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Brankas Media Terenkripsi -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (showMediaVault = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-blue">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="18" height="18" rx="2"/>
              <path d="M7 7h.01M7 12h.01M7 17h.01M11 7h6M11 12h6M11 17h6"/>
            </svg>
          </div>
          <div>
            <div class="item-title">Brankas Media & Steganografi</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Galeri berkas terenkripsi SQLCipher & LSB</div>
          </div>
        </div>
        <div class="item-right">
          <span class="item-value">{mekaarStore.vaultMediaItems.length} berkas</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Ekspor Bukti Forensik -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (mekaarStore.forensicDialogOpen = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-purple">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
              <line x1="16" y1="13" x2="8" y2="13"></line>
              <line x1="16" y1="17" x2="8" y2="17"></line>
            </svg>
          </div>
          <span class="item-title">Ekspor Bukti Forensik Terenkripsi</span>
        </div>
        <div class="item-right">
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Reset / Uji Ulang Alur Pengguna Baru (FTUE) -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => mekaarStore.resetFtueSimulation()}>
        <div class="item-left">
          <div class="item-icon-circle icon-orange">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#EA580C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
              <path d="M3 3v5h5"></path>
            </svg>
          </div>
          <span class="item-title">Uji Ulang Onboarding (Reset FTUE)</span>
        </div>
        <div class="item-right">
          <span class="item-value" style="color: #EA580C; font-size: 0.8rem; font-weight: 600;">Simulasi Cold-Start</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Modal Dialog Font & Tampilan -->
<Dialog
  open={showFontModal}
  title="Pilih Jenis Huruf Utama"
  onclose={() => (showFontModal = false)}
>
  <div style="display: flex; flex-direction: column; gap: 12px;">
    <button
      type="button"
      class="font-option-btn {mekaarStore.fontFamily === 'Plus Jakarta Sans' ? 'active' : ''}"
      onclick={() => {
        mekaarStore.fontFamily = 'Plus Jakarta Sans';
        showFontModal = false;
      }}
    >
      <span style="font-family: 'Plus Jakarta Sans', sans-serif; font-weight: 700;">Plus Jakarta Sans (Standar Citrea)</span>
      {#if mekaarStore.fontFamily === 'Plus Jakarta Sans'}<span>✓</span>{/if}
    </button>
    <button
      type="button"
      class="font-option-btn {mekaarStore.fontFamily === 'DynaPuff' ? 'active' : ''}"
      onclick={() => {
        mekaarStore.fontFamily = 'DynaPuff';
        showFontModal = false;
      }}
    >
      <span style="font-family: 'DynaPuff', cursive; font-weight: 700;">DynaPuff (Gaya Sketsa Krayon)</span>
      {#if mekaarStore.fontFamily === 'DynaPuff'}<span>✓</span>{/if}
    </button>
  </div>
</Dialog>

<!-- Modal Dialog Endpoint Tailscale -->
<Dialog
  open={showTailscaleModal}
  title="Konfigurasi Server Relay"
  onclose={() => (showTailscaleModal = false)}
>
  <div style="display: flex; flex-direction: column; gap: 12px; font-size: 0.88rem;">
    <p style="color: var(--text-secondary);">
      Aplikasi terhubung ke relay <strong>teplok-pc</strong> Linux via Tailscale Mesh.
    </p>
    <div style="background: #F1F5F9; padding: 12px; border-radius: 12px; font-family: var(--font-mono); font-size: 0.8rem;">
      <div>HTTP: {TAILSCALE_HTTP}</div>
      <div>WS: {TAILSCALE_WS}</div>
      <div>Status: Terhubung ({mekaarStore.serverStatus.latencyMs} ms)</div>
    </div>
    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 6px;">
      <button type="button" class="btn-primary" onclick={() => (showTailscaleModal = false)}>
        Tutup
      </button>
    </div>
  </div>
</Dialog>

<!-- Modal Dialog Atur Duress PIN -->
<Dialog
  open={showDuressModal}
  title="Konfigurasi PIN Paksaan (Duress PIN)"
  onclose={() => {
    showDuressModal = false;
    duressError = '';
    duressSuccess = '';
  }}
>
  <div style="display: flex; flex-direction: column; gap: 12px; font-size: 0.88rem;">
    <p style="color: var(--text-secondary);">
      Jika Anda berada di bawah tekanan atau ancaman untuk membuka aplikasi, masukkan PIN Paksaan ini.
      Aplikasi akan terbuka dalam mode samaran (cloaked), menyembunyikan brankas rahasia, dan menyiarkan <strong>Silent SOS</strong> ke Guardian Anda.
    </p>

    <label style="font-size: 0.85rem; font-weight: 600;">
      PIN Paksaan Baru (6 Digit)
      <input
        type="password"
        maxlength="6"
        class="sketch-input"
        placeholder="Contoh: 999999"
        bind:value={inputDuressPin}
        style="margin-top: 6px; letter-spacing: 4px; font-size: 1.1rem; text-align: center;"
      />
    </label>

    {#if duressError}
      <div style="color: var(--accent-danger); font-size: 0.82rem; font-weight: 600;">{duressError}</div>
    {/if}
    {#if duressSuccess}
      <div style="color: #059669; font-size: 0.82rem; font-weight: 600;">{duressSuccess}</div>
    {/if}

    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 6px;">
      <button
        type="button"
        class="btn-secondary"
        onclick={() => {
          showDuressModal = false;
          duressError = '';
          duressSuccess = '';
        }}
      >
        Batal
      </button>
      <button
        type="button"
        class="btn-primary"
        onclick={async () => {
          duressError = '';
          duressSuccess = '';
          if (inputDuressPin.length !== 6 || !/^\d+$/.test(inputDuressPin)) {
            duressError = 'PIN Paksaan harus terdiri dari 6 angka.';
            return;
          }
          if (inputDuressPin === mekaarStore.pinCode) {
            duressError = 'PIN Paksaan tidak boleh sama dengan PIN Utama!';
            return;
          }
          await mekaarStore.setupPins(mekaarStore.pinCode, inputDuressPin);
          duressSuccess = 'PIN Paksaan berhasil disimpan ke brankas keamanan.';
          setTimeout(() => {
            showDuressModal = false;
            duressSuccess = '';
          }, 1200);
        }}
      >
        Simpan PIN
      </button>
    </div>
  </div>
</Dialog>

<!-- Brankas Media & Berkas Terenkripsi -->
<SecureMediaVaultDialog
  open={showMediaVault}
  onclose={() => (showMediaVault = false)}
  onOpenStegoDecoder={(src, name) => {
    selectedStegoSrc = src;
    selectedStegoName = name;
    showStegoDecoder = true;
  }}
/>

<!-- Steganografi LSB Decoder Dialog -->
<StegoDecoderDialog
  open={showStegoDecoder}
  imageSrc={selectedStegoSrc}
  carrierName={selectedStegoName}
  onclose={() => (showStegoDecoder = false)}
/>

<style>
  .settings-page-container {
    padding: 0 16px 28px 16px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .settings-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .group-title {
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-left: 4px;
  }

  .account-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    cursor: pointer;
  }

  .account-card-left {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .account-info {
    display: flex;
    flex-direction: column;
  }

  .account-name {
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .account-email {
    font-size: 0.88rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .arrow-circle {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background-color: #F1F5F9;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .list-tile-card {
    padding: 6px 16px;
  }

  .setting-item-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 0;
    cursor: pointer;
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .item-icon-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .icon-purple { background-color: #F3E8FF; }
  .icon-blue { background-color: #EFF6FF; }
  .icon-teal { background-color: #CCFBF1; }
  .icon-orange { background-color: #FFEDD5; }
  .icon-blue-subtle { background-color: #E0F2FE; }
  .icon-lime { background-color: #DCFCE7; }

  .item-title {
    font-size: 0.98rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .item-right {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .item-value {
    font-size: 0.88rem;
    color: var(--text-secondary);
  }

  .item-divider {
    height: 1px;
    background-color: var(--border-light);
    margin: 0;
  }

  /* iOS / Flutter style pill switch toggle */
  .switch-toggle {
    position: relative;
    display: inline-block;
    width: 46px;
    height: 26px;
  }

  .switch-toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    inset: 0;
    background-color: #CBD5E1;
    transition: 0.2s;
    border-radius: 34px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.2s;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  input:checked + .slider {
    background-color: var(--brand-blue);
  }

  input:checked + .slider:before {
    transform: translateX(20px);
  }

  .font-option-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-radius: 12px;
    border: 1px solid var(--border-subtle);
    background: #FFFFFF;
    cursor: pointer;
    font-size: 0.95rem;
  }

  .font-option-btn.active {
    border-color: var(--brand-blue);
    background-color: #EFF6FF;
    color: var(--brand-blue);
  }
</style>
