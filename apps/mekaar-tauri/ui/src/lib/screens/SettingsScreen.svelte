<script lang="ts">
  import { mekaarStore, TAILSCALE_HTTP, TAILSCALE_WS, LOCAL_HTTP, LOCAL_WS } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';
  import SecureMediaVaultDialog from '../components/SecureMediaVaultDialog.svelte';
  import StegoDecoderDialog from '../components/StegoDecoderDialog.svelte';
  import ScheduledWipeDialog from '../components/ScheduledWipeDialog.svelte';
  import SecurityAuditDialog from '../components/SecurityAuditDialog.svelte';
  import ForensicWipeDialog from '../components/ForensicWipeDialog.svelte';
  import ConnectedDevicesDialog from '../components/ConnectedDevicesDialog.svelte';
  import BlockedListDialog from '../components/BlockedListDialog.svelte';
  import SecurityGlossaryDialog from '../components/SecurityGlossaryDialog.svelte';

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

  // Pilihan 5 Dialog States
  let showScheduledWipeModal = $state(false);
  let showSecurityAuditModal = $state(false);
  let showForensicWipeModal = $state(false);
  let showConnectedDevicesModal = $state(false);
  let showBlockedListModal = $state(false);

  // Pengaturan Privasi Lanjutan & Storage States
  let showAutoLockModal = $state(false);
  let showLastSeenModal = $state(false);
  let showInvitationModal = $state(false);
  let showGlossaryModal = $state(false);
  let cacheToast = $state('');

  function handleClearCache() {
    mekaarStore.clearTempCache();
    cacheToast = '✓ 18.4 MB berkas cache sementara berhasil dibersihkan!';
    setTimeout(() => {
      if (cacheToast.includes('18.4 MB')) cacheToast = '';
    }, 3000);
  }


</script>

<AppBar title="Pengaturan" />

<div class="settings-page-container">
  {#if cacheToast}
    <div class="cache-floating-toast">
      {cacheToast}
    </div>
  {/if}

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

        <!-- Kunci Otomatis (Inactivity Auto-Lock) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showAutoLockModal = true)}>
          <div class="item-left">
            <div class="item-icon-circle icon-blue">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <polyline points="12 6 12 12 16 14"></polyline>
              </svg>
            </div>
            <div>
              <div class="item-title">Kunci Otomatis (Auto-Lock)</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">Minta PIN setelah tidak ada aktivitas</div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value">
              {mekaarStore.autoLockMinutes === 0
                ? 'Segera'
                : mekaarStore.autoLockMinutes === -1
                  ? 'Nonaktif'
                  : `${mekaarStore.autoLockMinutes} Menit`}
            </span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
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
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showLastSeenModal = true)}>
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
            <span class="item-value">
              {mekaarStore.lastSeenPrivacy === 'everyone'
                ? 'Semua Orang'
                : mekaarStore.lastSeenPrivacy === 'nobody'
                  ? 'Tidak Ada'
                  : 'Kontak Saya'}
            </span>
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

        <div class="item-divider"></div>

        <!-- Pembersihan Pesan Terjadwal (Scheduled Wipe) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showScheduledWipeModal = true)}>
          <div class="item-left">
            <div class="item-icon-circle icon-blue">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <polyline points="12 6 12 12 16 14"></polyline>
              </svg>
            </div>
            <div>
              <div class="item-title">Pembersihan Pesan Terjadwal</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">
                {mekaarStore.scheduledWipe.mode === 'off'
                  ? 'Nonaktif'
                  : `${mekaarStore.scheduledWipe.mode === 'daily' ? 'Setiap Hari' : 'Sekali'} pukul ${mekaarStore.scheduledWipe.time}`}
              </div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value" style="text-transform: capitalize;">{mekaarStore.scheduledWipe.mode}</span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Perangkat Terhubung (Connected Devices) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showConnectedDevicesModal = true)}>
          <div class="item-left">
            <div class="item-icon-circle icon-blue-subtle">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
                <line x1="8" y1="21" x2="16" y2="21"></line>
                <line x1="12" y1="17" x2="12" y2="21"></line>
              </svg>
            </div>
            <div>
              <div class="item-title">Perangkat Terhubung</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">{mekaarStore.connectedDevices.length} sesi aktif E2EE</div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value">Kelola</span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Daftar Kontak Diblokir -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showBlockedListModal = true)}>
          <div class="item-left">
            <div class="item-icon-circle icon-orange">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#EA580C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
              </svg>
            </div>
            <div>
              <div class="item-title">Daftar Kontak Diblokir</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">{mekaarStore.blockedUsers.length} pengguna dibatasi</div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value">{mekaarStore.blockedUsers.length > 0 ? `${mekaarStore.blockedUsers.length} kontak` : 'Kosong'}</span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Izin Undangan Obrolan (Chat Requests) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-item-row" onclick={() => (showInvitationModal = true)}>
          <div class="item-left">
            <div class="item-icon-circle icon-purple">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
              </svg>
            </div>
            <div>
              <div class="item-title">Izin Undangan Obrolan</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">Filter kontak masuk & permintaan obrolan</div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value">
              {mekaarStore.chatInvitationMode === 'everyone'
                ? 'Semua Orang'
                : mekaarStore.chatInvitationMode === 'contacts'
                  ? 'Kontak Saya'
                  : 'Persetujuan'}
            </span>
            <div class="arrow-circle">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="9 18 15 12 9 6"></polyline>
              </svg>
            </div>
          </div>
        </div>

        <div class="item-divider"></div>

        <!-- Proteksi IP Panggilan (Relay Calls) -->
        <div class="setting-item-row">
          <div class="item-left">
            <div class="item-icon-circle icon-teal">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#0D9488" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
              </svg>
            </div>
            <div>
              <div class="item-title">Relay Calls (Sembunyikan IP)</div>
              <div style="font-size: 0.76rem; color: var(--text-secondary);">Alihkan panggilan via Mekaar Mesh Network agar alamat IP & lokasi aman</div>
            </div>
          </div>
          <div class="item-right">
            <span class="item-value">{mekaarStore.callRelayEnabled ? 'Aktif' : 'Nonaktif'}</span>
            <label class="switch-toggle">
              <input type="checkbox" bind:checked={mekaarStore.callRelayEnabled} />
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Section: Penyimpanan & Data (Storage & Cache Manager) -->
  <div class="settings-group">
    <span class="group-title">Penyimpanan & Data</span>
    <div class="clean-card with-ticks list-tile-card">
      <span class="pencil-tick tick-bl">⌞</span>
      <span class="pencil-tick tick-tr">⌝</span>

      <!-- Storage Breakdown Card -->
      <div class="storage-breakdown-card">
        <div class="storage-bar-wrap">
          <div class="bar-segment bar-media" style="width: 52%;" title="Media & Brankas: 24.8 MB"></div>
          <div class="bar-segment bar-cache" style="width: 32%;" title="Cache & Berkas Sementara: 18.4 MB"></div>
          <div class="bar-segment bar-db" style="width: 16%;" title="Basis Data SQLCipher: 4.2 MB"></div>
        </div>

        <div class="storage-legend-grid">
          <div class="legend-item">
            <span class="legend-dot dot-media">●</span>
            <span class="legend-label">Media: <strong>24.8 MB</strong></span>
          </div>
          <div class="legend-item">
            <span class="legend-dot dot-cache">●</span>
            <span class="legend-label">Cache: <strong>18.4 MB</strong></span>
          </div>
          <div class="legend-item">
            <span class="legend-dot dot-db">●</span>
            <span class="legend-label">Database: <strong>4.2 MB</strong></span>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Action: Bersihkan Cache Sementara -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={handleClearCache}>
        <div class="item-left">
          <div class="item-icon-circle icon-orange">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#EA580C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </div>
          <div>
            <div class="item-title">Bersihkan Cache Sementara</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Hapus berkas pratinjau & thumbnail tanpa menghapus riwayat pesan</div>
          </div>
        </div>
        <div class="item-right">
          <span class="item-value" style="color: var(--brand-blue); font-weight: 700;">Bersihkan</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Section: Server Relay teplok-pc & Forensik -->
  <div class="settings-group">
    <span class="group-title">Sistem & Keamanan Tingkat Lanjut</span>
    <div class="clean-card with-ticks list-tile-card">
      <span class="pencil-tick tick-bl">⌞</span>
      <span class="pencil-tick tick-tr">⌝</span>

      <!-- Mekaar Mesh Network Node -->
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
            <div class="item-title">Mekaar Mesh Network</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Jaringan privat terenkripsi • Latensi {mekaarStore.serverStatus.latencyMs}ms</div>
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

      <!-- Media Vault & Stego Photo -->
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
            <div class="item-title">Media Vault & Stego Photo</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Penyimpanan berkas aman & sembunyikan pesan di balik foto</div>
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

      <!-- Security Audit Log -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (showSecurityAuditModal = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-purple">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
              <line x1="16" y1="13" x2="8" y2="13"></line>
              <line x1="16" y1="17" x2="8" y2="17"></line>
            </svg>
          </div>
          <div>
            <div class="item-title">Security Audit Log</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Catatan integritas sistem & riwayat perlindungan perangkat</div>
          </div>
        </div>
        <div class="item-right">
          <span class="item-value">{mekaarStore.securityLogs.length} kejadian</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Digital Evidence Export -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (mekaarStore.forensicDialogOpen = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-purple">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
              <circle cx="12" cy="11" r="3"/>
            </svg>
          </div>
          <div>
            <div class="item-title">Digital Evidence Export</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Unduh arsip bukti obrolan dengan tanda tangan digital resmi</div>
          </div>
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

      <!-- Panic Wipe (Pemusnahan Darurat) -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row danger-row" onclick={() => (showForensicWipeModal = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-red">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#E11D48" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
              <line x1="12" y1="9" x2="12" y2="13"></line>
              <line x1="12" y1="17" x2="12.01" y2="17"></line>
            </svg>
          </div>
          <div>
            <div class="item-title" style="color: #E11D48;">Panic Wipe (Pemusnahan Darurat)</div>
            <div style="font-size: 0.76rem; color: #FDA4AF;">Hapus permanen seluruh percakapan & kunci saat situasi kritis</div>
          </div>
        </div>
        <div class="item-right">
          <span class="item-value" style="color: #E11D48; font-weight: 700;">Hancurkan Data</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#FDA4AF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Reset Pengenalan Aplikasi -->
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
          <div>
            <div class="item-title">Reset Pengenalan Aplikasi</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Mulai ulang panduan awal & orientasi akun baru</div>
          </div>
        </div>
        <div class="item-right">
          <span class="item-value" style="color: #EA580C; font-size: 0.8rem; font-weight: 600;">Mulai Ulang</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Section: Informasi & Panduan -->
  <div class="settings-group">
    <span class="group-title">Informasi & Panduan</span>
    <div class="clean-card with-ticks list-tile-card">
      <span class="pencil-tick tick-bl">⌞</span>
      <span class="pencil-tick tick-tr">⌝</span>

      <!-- Kamus Istilah Keamanan (Security Glossary) -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="setting-item-row" onclick={() => (showGlossaryModal = true)}>
        <div class="item-left">
          <div class="item-icon-circle icon-purple">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
              <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
            </svg>
          </div>
          <div>
            <div class="item-title">Kamus Istilah Keamanan (Security Glossary)</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Penjelasan ramah tentang Mekaar Mesh, E2EE, Call Relay & Stego Photo</div>
          </div>
        </div>
        <div class="item-right">
          <span class="item-value" style="color: var(--brand-blue); font-weight: 700;">Pelajari</span>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <div class="item-divider"></div>

      <!-- Tentang MEKAAR -->
      <div class="setting-item-row">
        <div class="item-left">
          <div class="item-icon-circle icon-blue-subtle">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="16" x2="12" y2="12"/>
              <line x1="12" y1="8" x2="12.01" y2="8"/>
            </svg>
          </div>
          <div>
            <div class="item-title">Tentang MEKAAR Tactical Suite</div>
            <div style="font-size: 0.76rem; color: var(--text-secondary);">Versi 0.1.0 • E2EE Mesh Platform</div>
          </div>
        </div>
        <div class="item-right">
          <span class="aegis-badge">Produksi</span>
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
  title="Audit Node Mekaar Mesh Network"
  onclose={() => (showTailscaleModal = false)}
>
  <div style="display: flex; flex-direction: column; gap: 12px; font-size: 0.88rem;">
    <p style="color: var(--text-secondary);">
      Aplikasi terhubung ke node <strong>Mekaar Mesh Network</strong> melalui terowongan terenkripsi privat.
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

<!-- Pembersihan Pesan Terjadwal -->
<ScheduledWipeDialog
  open={showScheduledWipeModal}
  onclose={() => (showScheduledWipeModal = false)}
/>

<!-- Log Audit Keamanan SQLCipher -->
<SecurityAuditDialog
  open={showSecurityAuditModal}
  onclose={() => (showSecurityAuditModal = false)}
/>

<!-- Pemusnahan Forensik Darurat (Forensic Wipe) -->
<ForensicWipeDialog
  open={showForensicWipeModal}
  onclose={() => (showForensicWipeModal = false)}
/>

<!-- Perangkat Terhubung (Connected Devices) -->
<ConnectedDevicesDialog
  open={showConnectedDevicesModal}
  onclose={() => (showConnectedDevicesModal = false)}
/>

<!-- Daftar Kontak Diblokir -->
<BlockedListDialog
  open={showBlockedListModal}
  onclose={() => (showBlockedListModal = false)}
/>

<!-- Dialog Kunci Otomatis (Inactivity Timer) -->
<Dialog
  open={showAutoLockModal}
  title="⏱️ Kunci Otomatis Aplikasi"
  onclose={() => (showAutoLockModal = false)}
>
  <div class="modal-options-list">
    <p style="font-size: 0.84rem; color: var(--text-secondary); margin-bottom: 6px;">
      Aplikasi akan terkunci dan meminta verifikasi PIN jika tidak ada interaksi selama jangka waktu berikut:
    </p>
    {#each [
      { label: 'Segera saat berpindah aplikasi', min: 0 },
      { label: '1 Menit inaktivitas', min: 1 },
      { label: '5 Menit inaktivitas (Rekomendasi)', min: 5 },
      { label: '15 Menit inaktivitas', min: 15 },
      { label: '1 Jam inaktivitas', min: 60 },
      { label: 'Nonaktifkan kunci otomatis', min: -1 }
    ] as item}
      <button
        type="button"
        class="dialog-option-btn {mekaarStore.autoLockMinutes === item.min ? 'dialog-option-active' : ''}"
        onclick={() => {
          mekaarStore.autoLockMinutes = item.min;
          showAutoLockModal = false;
        }}
      >
        <span>{item.label}</span>
        {#if mekaarStore.autoLockMinutes === item.min}<span>✓</span>{/if}
      </button>
    {/each}
  </div>
</Dialog>

<!-- Dialog Terakhir Dilihat & Online -->
<Dialog
  open={showLastSeenModal}
  title="👁️ Privasi Terakhir Dilihat & Online"
  onclose={() => (showLastSeenModal = false)}
>
  <div class="modal-options-list">
    <p style="font-size: 0.84rem; color: var(--text-secondary); margin-bottom: 6px;">
      Pilih siapa saja yang dapat melihat waktu terakhir kali Anda online di jaringan:
    </p>
    {#each [
      { label: 'Semua Orang', val: 'everyone', desc: 'Dapat dilihat oleh semua kontak terdaftar di Mekaar Mesh Network' },
      { label: 'Kontak Saya (Rekomendasi)', val: 'contacts', desc: 'Hanya kontak terverifikasi di daftar buku alamat Anda' },
      { label: 'Tidak Ada (Maksimal Privasi)', val: 'nobody', desc: 'Status online & waktu terakhir tidak pernah dibagikan' }
    ] as item}
      <button
        type="button"
        class="dialog-option-btn {mekaarStore.lastSeenPrivacy === item.val ? 'dialog-option-active' : ''}"
        onclick={() => {
          mekaarStore.lastSeenPrivacy = item.val as any;
          showLastSeenModal = false;
        }}
      >
        <div style="display: flex; flex-direction: column; text-align: left;">
          <strong>{item.label}</strong>
          <span style="font-size: 0.74rem; color: var(--text-secondary);">{item.desc}</span>
        </div>
        {#if mekaarStore.lastSeenPrivacy === item.val}<span>✓</span>{/if}
      </button>
    {/each}
  </div>
</Dialog>

<!-- Dialog Izin Undangan Obrolan -->
<Dialog
  open={showInvitationModal}
  title="📨 Izin Undangan Obrolan"
  onclose={() => (showInvitationModal = false)}
>
  <div class="modal-options-list">
    <p style="font-size: 0.84rem; color: var(--text-secondary); margin-bottom: 6px;">
      Tentukan bagaimana ruang obrolan baru dapat dimulai dengan akun Anda:
    </p>
    {#each [
      { label: 'Permintaan Wajib Disetujui (Default Mekaar)', val: 'approved_only', desc: 'Pesan dari orang tak dikenal masuk ke kotak Permintaan Chat terlebih dahulu' },
      { label: 'Hanya Kontak Terverifikasi', val: 'contacts', desc: 'Hanya pengguna yang sudah ada di daftar kontak Anda yang dapat mengirim pesan' },
      { label: 'Semua Orang', val: 'everyone', desc: 'Siapa pun yang memiliki ID/Username Anda dapat langsung mengirim pesan' }
    ] as item}
      <button
        type="button"
        class="dialog-option-btn {mekaarStore.chatInvitationMode === item.val ? 'dialog-option-active' : ''}"
        onclick={() => {
          mekaarStore.chatInvitationMode = item.val as any;
          showInvitationModal = false;
        }}
      >
        <div style="display: flex; flex-direction: column; text-align: left;">
          <strong>{item.label}</strong>
          <span style="font-size: 0.74rem; color: var(--text-secondary);">{item.desc}</span>
        </div>
        {#if mekaarStore.chatInvitationMode === item.val}<span>✓</span>{/if}
      </button>
    {/each}
  </div>
</Dialog>

<!-- Kamus Istilah Keamanan (Security Glossary) Dialog -->
<SecurityGlossaryDialog
  open={showGlossaryModal}
  onclose={() => (showGlossaryModal = false)}
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

  .cache-floating-toast {
    background: #0F172A;
    color: #38BDF8;
    font-size: 0.84rem;
    font-weight: 600;
    padding: 8px 16px;
    border-radius: 999px;
    border: 1.5px solid #38BDF8;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    align-self: center;
    margin-bottom: -10px;
    animation: fadeInToast 0.25s ease-out;
  }

  @keyframes fadeInToast {
    from { opacity: 0; transform: translateY(-8px); }
    to { opacity: 1; transform: translateY(0); }
  }

  /* Storage Breakdown Card */
  .storage-breakdown-card {
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .storage-bar-wrap {
    height: 10px;
    border-radius: 999px;
    display: flex;
    overflow: hidden;
    background: var(--bg-secondary, #E2E8F0);
    border: 1px solid var(--border-light, #CBD5E1);
  }

  .bar-segment {
    height: 100%;
    transition: width 0.3s ease;
  }

  .bar-media { background: var(--brand-blue, #0284C7); }
  .bar-cache { background: #EA580C; }
  .bar-db { background: var(--brand-teal, #0D9488); }

  .storage-legend-grid {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.76rem;
    color: var(--text-secondary);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .legend-dot {
    font-size: 0.9rem;
  }

  .dot-media { color: var(--brand-blue, #0284C7); }
  .dot-cache { color: #EA580C; }
  .dot-db { color: var(--brand-teal, #0D9488); }

  /* Modal Options List */
  .modal-options-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .dialog-option-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    border-radius: 10px;
    border: 1.5px solid var(--border-sketch-color, #E2E8F0);
    background: var(--bg-main, #FFFFFF);
    color: var(--text-main);
    cursor: pointer;
    font-size: 0.86rem;
    transition: all 0.15s ease;
  }

  .dialog-option-btn:hover {
    border-color: var(--brand-blue);
    background: var(--bg-secondary);
  }

  .dialog-option-btn.dialog-option-active {
    border-color: var(--brand-blue);
    background: #EFF6FF;
    color: var(--brand-blue);
    font-weight: 700;
  }
</style>
