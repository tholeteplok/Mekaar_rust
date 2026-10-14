<script lang="ts">
  import { mekaarStore, TAILSCALE_HTTP, TAILSCALE_WS, LOCAL_HTTP, LOCAL_WS } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import CardExpanded from '../components/CardExpanded.svelte';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let wipeConfirmInput = $state('');
  let wipeStatusMessage = $state('');

  function triggerRemoteWipeTest() {
    if (wipeConfirmInput !== 'WIPE-MEKAAR') {
      wipeStatusMessage = 'Ketik WIPE-MEKAAR dengan benar untuk menguji kunci darurat.';
      return;
    }
    wipeStatusMessage = '✅ Brankas lokal dikunci & token wipe diverifikasi.';
    wipeConfirmInput = '';
    mekaarStore.vaultUnlocked = false;
  }
</script>

<AppBar title="Pengaturan & Keamanan" subtitle="Node teplok-pc (8085), Tema, Privasi Layar, Notifikasi & Device Lost" />

<div class="settings-container">
  <!-- Profil & Identitas Kriptografi -->
  <Card padding="16px">
    <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px; flex-wrap: wrap;">
      <div style="display: flex; align-items: center; gap: 14px;">
        <SketchAvatar name={mekaarStore.currentUser.displayName} color={mekaarStore.currentUser.avatarColor} size={58} online={true} />
        <div>
          <h3 style="font-size: 1.08rem;">{mekaarStore.currentUser.displayName}</h3>
          <div style="font-size: 0.84rem; color: var(--text-secondary);">{mekaarStore.currentUser.username} • {mekaarStore.currentUser.bio}</div>
          <div style="font-size: 0.75rem; font-family: var(--font-mono); color: var(--text-muted); margin-top: 2px;">
            🔑 {mekaarStore.currentUser.publicKeyFingerprint}
          </div>
        </div>
      </div>
      <div style="display: flex; gap: 8px;">
        <Button size="sm" variant="secondary" onclick={() => (mekaarStore.forensicDialogOpen = true)}>
          🔐 Ekspor Forensik
        </Button>
        <Button size="sm" variant="danger" onclick={() => (mekaarStore.vaultUnlocked = false)}>
          🔒 Kunci Vault
        </Button>
      </div>
    </div>
  </Card>

  <div style="height: 14px;"></div>

  <!-- 1. Konfigurasi Server teplok-pc (Tailscale & Local 8085) -->
  <CardExpanded
    title="🛰️ Koneksi Server Relay (teplok-pc • Port 8085)"
    subtitle="Tailscale 100.101.96.120:8085 & Local 127.0.0.1:8085"
    badgeText={mekaarStore.serverStatus.online ? 'Terhubung' : 'Siaga'}
    defaultOpen={true}
  >
    <div style="display: flex; flex-direction: column; gap: 10px;">
      <div class="sketch-card" style="padding: 10px 12px; background: var(--bg-secondary); font-size: 0.83rem;">
        <div><strong>Service:</strong> {mekaarStore.serverStatus.service} v{mekaarStore.serverStatus.version} (SQLite WAL • ~1.3 MB RAM)</div>
        <div><strong>HTTP Health:</strong> <code>{mekaarStore.serverStatus.httpEndpoint}</code> ({mekaarStore.serverStatus.latencyMs} ms)</div>
        <div><strong>WebSocket Relay:</strong> <code>{mekaarStore.serverStatus.wsEndpoint}</code></div>
      </div>
      <div style="display: flex; gap: 8px; flex-wrap: wrap;">
        <Button
          size="sm"
          variant={mekaarStore.serverStatus.mode === 'tailscale' ? 'primary' : 'secondary'}
          onclick={() => mekaarStore.checkServerHealth('tailscale')}
        >
          🌐 Gunakan Tailscale ({TAILSCALE_WS})
        </Button>
        <Button
          size="sm"
          variant={mekaarStore.serverStatus.mode === 'local' ? 'primary' : 'secondary'}
          onclick={() => mekaarStore.checkServerHealth('local')}
        >
          💻 Gunakan Localhost ({LOCAL_WS})
        </Button>
        <Button size="sm" variant="secondary" onclick={() => mekaarStore.checkServerHealth()}>
          🔄 Verifikasi Ulang Endpoint
        </Button>
      </div>
    </div>
  </CardExpanded>

  <!-- 2. Tema & Tampilan (theme_settings_screen.dart & chat_settings_screen.dart parity) -->
  <CardExpanded title="🎨 Tema, Tipografi & Pengaturan Chat" subtitle="Mode Terang/Gelap, Font DynaPuff/Plus Jakarta, Skala Teks">
    <div style="display: flex; flex-direction: column; gap: 10px;">
      <div style="display: flex; align-items: center; justify-content: space-between;">
        <span>Mode Tampilan</span>
        <div style="display: flex; gap: 6px;">
          <Button
            size="sm"
            variant={mekaarStore.themeMode === 'light' ? 'primary' : 'secondary'}
            onclick={() => (mekaarStore.themeMode = 'light')}
          >
            ☀️ Terang
          </Button>
          <Button
            size="sm"
            variant={mekaarStore.themeMode === 'dark' ? 'primary' : 'secondary'}
            onclick={() => (mekaarStore.themeMode = 'dark')}
          >
            🌙 Gelap
          </Button>
        </div>
      </div>

      <div style="display: flex; align-items: center; justify-content: space-between;">
        <span>Jenis Huruf Utama (`font_provider.dart` parity)</span>
        <div style="display: flex; gap: 6px;">
          <Button
            size="sm"
            variant={mekaarStore.fontFamily === 'Plus Jakarta Sans' ? 'primary' : 'secondary'}
            onclick={() => (mekaarStore.fontFamily = 'Plus Jakarta Sans')}
          >
            Plus Jakarta Sans
          </Button>
          <Button
            size="sm"
            variant={mekaarStore.fontFamily === 'DynaPuff' ? 'primary' : 'secondary'}
            onclick={() => (mekaarStore.fontFamily = 'DynaPuff')}
          >
            DynaPuff (Krayon)
          </Button>
        </div>
      </div>

      <label style="font-size: 0.85rem; font-weight: 600;">
        Skala Ukuran Huruf ({Math.round(mekaarStore.fontScale * 100)}%)
        <input
          type="range"
          min="0.9"
          max="1.2"
          step="0.05"
          bind:value={mekaarStore.fontScale}
          style="width: 100%; margin-top: 4px;"
        />
      </label>

      <label style="display: flex; justify-content: space-between; align-items: center;">
        <span>✓✓ Kirim Laporan Baca (Read Receipts)</span>
        <input type="checkbox" bind:checked={mekaarStore.readReceipts} />
      </label>
      <label style="display: flex; justify-content: space-between; align-items: center;">
        <span>✍️ Tampilkan Indikator Sedang Mengetik</span>
        <input type="checkbox" bind:checked={mekaarStore.typingIndicator} />
      </label>
    </div>
  </CardExpanded>

  <!-- 3. Keamanan, Screen Protection & Daftar Blokir (security_settings_screen.dart parity) -->
  <CardExpanded title="🛡️ Keamanan, Proteksi Layar & Log Audit" subtitle="Blokir Screenshot, Blur Fokus, 2FA & Pengguna Diblokir">
    <div style="display: flex; flex-direction: column; gap: 10px;">
      <label style="display: flex; justify-content: space-between; align-items: center;">
        <span>📵 Blokir Tangkapan Layar OS (`screenshotBlockProvider`)</span>
        <input type="checkbox" bind:checked={mekaarStore.screenshotBlock} />
      </label>
      <label style="display: flex; justify-content: space-between; align-items: center;">
        <span>🙈 Samarkan Layar Saat Jendela Tidak Fokus (Screen Shield)</span>
        <input type="checkbox" bind:checked={mekaarStore.privacyBlurOnUnfocus} />
      </label>
      <label style="display: flex; justify-content: space-between; align-items: center;">
        <span>🔔 Notifikasi Tersamar (Sembunyikan Nama & Isi Pesan)</span>
        <input type="checkbox" bind:checked={mekaarStore.disguisedNotifications} />
      </label>

      <div style="font-weight: 700; font-size: 0.85rem; margin-top: 6px;">📋 Log Keamanan Terbaru:</div>
      {#each mekaarStore.securityLogs as log (log.id)}
        <div class="sketch-card" style="padding: 8px 12px; font-size: 0.8rem;">
          <div style="display: flex; justify-content: space-between;">
            <strong>{log.eventTitle}</strong>
            <span style="color: var(--text-muted);">{log.timestamp}</span>
          </div>
          <div style="color: var(--text-secondary);">{log.deviceInfo} • Kategori: {log.eventCategory}</div>
        </div>
      {/each}

      <div style="font-weight: 700; font-size: 0.85rem; margin-top: 6px;">
        🚫 Daftar Pengguna Diblokir ({mekaarStore.blockedUsers.length}):
      </div>
      {#each mekaarStore.blockedUsers as blk (blk.id)}
        <div class="sketch-card" style="padding: 8px 12px; display: flex; justify-content: space-between; align-items: center; font-size: 0.82rem;">
          <div>
            <strong>{blk.displayName}</strong> ({blk.username}) — {blk.reason}
          </div>
          <Button
            size="sm"
            variant="secondary"
            onclick={() => (mekaarStore.blockedUsers = mekaarStore.blockedUsers.filter((b) => b.id !== blk.id))}
          >
            Buka Blokir
          </Button>
        </div>
      {/each}
    </div>
  </CardExpanded>

  <!-- 4. Mode Perangkat Hilang / Remote Wipe (device_lost_screen.dart parity) -->
  <CardExpanded title="📱 Mode Perangkat Hilang (Device Lost Protection)" subtitle="Penghapusan Brankas Darurat & Batas Percobaan PIN">
    <div style="display: flex; flex-direction: column; gap: 10px;">
      <div style="font-size: 0.84rem; color: var(--text-secondary);">
        Guardian Pemulihan: <strong>{mekaarStore.deviceLost.recoveryEmailOrPeer}</strong> • Auto-wipe setelah
        <strong>{mekaarStore.deviceLost.autoWipeAfterFailedPinAttempts}x</strong> salah PIN.
      </div>
      <div style="display: flex; gap: 8px; flex-wrap: wrap;">
        <input
          type="text"
          class="sketch-input"
          style="flex: 1;"
          placeholder="Ketik WIPE-MEKAAR untuk menguji kunci darurat..."
          bind:value={wipeConfirmInput}
        />
        <Button variant="danger" onclick={triggerRemoteWipeTest}>Kunci & Hapus Sesi Lokal</Button>
      </div>
      {#if wipeStatusMessage}
        <div style="font-size: 0.82rem; font-weight: 700;">{wipeStatusMessage}</div>
      {/if}
    </div>
  </CardExpanded>
</div>

<style>
  .settings-container {
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
</style>
