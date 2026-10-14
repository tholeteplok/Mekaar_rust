<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import HoldToConfirmButton from '../components/HoldToConfirmButton.svelte';

  let cancelPin = $state('');
  let pinError = $state('');
  let countdownTimer: ReturnType<typeof setInterval> | null = null;

  function startCountdown() {
    mekaarStore.sosSession.countdownActive = true;
    mekaarStore.sosSession.countdownRemaining = 5;
    countdownTimer = setInterval(() => {
      mekaarStore.sosSession.countdownRemaining -= 1;
      if (mekaarStore.sosSession.countdownRemaining <= 0) {
        if (countdownTimer) clearInterval(countdownTimer);
        mekaarStore.activateSos();
      }
    }, 1000);
  }

  function abortCountdown() {
    if (countdownTimer) clearInterval(countdownTimer);
    mekaarStore.sosSession.countdownActive = false;
  }

  function handleDeactivateSos() {
    pinError = '';
    if (!mekaarStore.deactivateSos(cancelPin)) {
      pinError = 'PIN pembatalan salah (Default: 123456).';
      return;
    }
    cancelPin = '';
  }
</script>

<AppBar title="Mode Darurat (SOS)" subtitle="Penyiaran Koordinat Darurat, Sirene & Peringatan Guardian" />

<div class="sos-container">
  {#if mekaarStore.sosSession.active}
    <!-- Layar SOS Aktif (sos_active_screen.dart parity) -->
    <div class="sketch-card sos-active-banner">
      <div style="font-size: 2.5rem; margin-bottom: 6px;">🚨</div>
      <h2 style="font-family: var(--font-crayon); font-size: 1.45rem; color: #881337;">
        SINYAL SOS DARURAT AKTIF!
      </h2>
      <p style="font-size: 0.9rem; margin: 8px 0 12px;">
        {mekaarStore.sosSession.emergencyMessage}
      </p>
      <div class="sketch-card" style="padding: 10px 14px; background: #FFFFFF; margin-bottom: 14px; text-align: left;">
        <div style="font-size: 0.84rem;">
          📍 <strong>Koordinat Disiarkan:</strong> {mekaarStore.sosSession.latitude}, {mekaarStore.sosSession.longitude}
        </div>
        <div style="font-size: 0.84rem;">
          🛡️ <strong>Guardian Diberitahu:</strong> {mekaarStore.sosSession.notifiedGuardians.join(', ')}
        </div>
        <div style="font-size: 0.84rem;">
          🔊 <strong>Sirene Darurat:</strong> {mekaarStore.sosSession.sirenEnabled ? 'Berbunyi Keras' : 'Senyap (Stealth)'}
        </div>
      </div>

      <div style="display: flex; gap: 8px; align-items: center; justify-content: center; flex-wrap: wrap;">
        <input
          type="password"
          class="sketch-input"
          style="max-width: 200px; text-align: center;"
          placeholder="PIN Batal (123456)"
          bind:value={cancelPin}
        />
        <Button variant="secondary" onclick={handleDeactivateSos}>Matikan Sinyal SOS</Button>
      </div>
      {#if pinError}
        <p style="color: #881337; font-weight: 700; font-size: 0.84rem; margin-top: 8px;">{pinError}</p>
      {/if}
    </div>
  {:else if mekaarStore.sosSession.countdownActive}
    <!-- Layar Hitung Mundur SOS (sos_countdown_screen.dart parity) -->
    <div class="sketch-card sos-countdown-box">
      <div class="countdown-number">{mekaarStore.sosSession.countdownRemaining}</div>
      <h3 style="font-family: var(--font-crayon); font-size: 1.25rem; margin-bottom: 6px;">
        Mengirim Sinyal Darurat ke Guardian...
      </h3>
      <p style="font-size: 0.86rem; color: var(--text-secondary); margin-bottom: 16px;">
        Lokasi GPS dan peringatan darurat akan dikirim melalui <code>{mekaarStore.serverStatus.wsEndpoint}</code>.
      </p>
      <Button variant="secondary" size="lg" onclick={abortCountdown}>✕ Batalkan Hitung Mundur</Button>
    </div>
  {:else}
    <!-- Pemicu SOS & Pengaturan SOS (sos_settings_screen.dart parity) -->
    <div style="display: flex; flex-direction: column; gap: 14px;">
      <Card padding="20px">
        <h3 style="font-family: var(--font-crayon); font-size: 1.15rem; margin-bottom: 6px;">
          Pemicu Darurat Cepat
        </h3>
        <p style="font-size: 0.86rem; color: var(--text-secondary); margin-bottom: 14px;">
          Tahan tombol merah di bawah selama 2 detik untuk langsung menyiarkan SOS, atau mulai hitung mundur 5 detik.
        </p>
        <HoldToConfirmButton onconfirm={() => mekaarStore.activateSos()} />
        <div style="margin-top: 10px;">
          <Button variant="secondary" fullWidth onclick={startCountdown}>
            ⏱️ Mulai Hitung Mundur SOS (5 Detik)
          </Button>
        </div>
      </Card>

      <Card padding="18px">
        <h3 style="font-family: var(--font-crayon); font-size: 1.05rem; margin-bottom: 10px;">
          ⚙️ Pengaturan Mode Darurat (SOS Settings)
        </h3>
        <div style="display: flex; flex-direction: column; gap: 10px;">
          <label style="display: flex; justify-content: space-between; align-items: center;">
            <span>🔊 Bunyikan Sirene Darurat Saat SOS Aktif</span>
            <input type="checkbox" bind:checked={mekaarStore.sosSession.sirenEnabled} />
          </label>
          <label style="display: flex; justify-content: space-between; align-items: center;">
            <span>📳 Goyangkan Perangkat (Shake-to-SOS)</span>
            <input type="checkbox" bind:checked={mekaarStore.sosSession.shakeToTrigger} />
          </label>
          <label style="display: flex; justify-content: space-between; align-items: center;">
            <span>📍 Siarkan Koordinat GPS Otomatis ke Guardian</span>
            <input type="checkbox" bind:checked={mekaarStore.sosSession.autoBroadcastLocation} />
          </label>
          <label style="font-size: 0.85rem; font-weight: 700; margin-top: 4px;">
            Pesan Darurat Kustom:
            <input
              type="text"
              class="sketch-input"
              bind:value={mekaarStore.sosSession.emergencyMessage}
              style="margin-top: 6px;"
            />
          </label>
        </div>
      </Card>
    </div>
  {/if}
</div>

<style>
  .sos-container {
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
  .sos-active-banner {
    padding: 24px;
    background: #FFE4E6;
    border-color: #9F1239;
    text-align: center;
  }
  .sos-countdown-box {
    padding: 28px;
    text-align: center;
    background: #FFF1F2;
  }
  .countdown-number {
    font-family: var(--font-crayon);
    font-size: 4rem;
    font-weight: 800;
    color: #E11D48;
    line-height: 1;
    margin-bottom: 10px;
  }
</style>
