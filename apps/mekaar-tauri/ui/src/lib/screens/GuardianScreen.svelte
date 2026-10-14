<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import CardExpanded from '../components/CardExpanded.svelte';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let originInput = $state('');
  let destinationInput = $state('');
  let etaInput = $state(20);

  function startNewTrip() {
    if (!originInput.trim() || !destinationInput.trim()) return;
    mekaarStore.trips = [
      {
        id: `trip-${Date.now()}`,
        originName: originInput.trim(),
        destinationName: destinationInput.trim(),
        startedAt: 'Baru saja',
        etaMinutes: etaInput,
        status: 'ongoing',
        activeGuardians: mekaarStore.guardians.filter((g) => g.status === 'active').map((g) => g.name),
        progressPercent: 10
      },
      ...mekaarStore.trips
    ];
    originInput = '';
    destinationInput = '';
  }

  function completeTrip(tripId: string) {
    const t = mekaarStore.trips.find((tr) => tr.id === tripId);
    if (t) {
      t.status = 'completed';
      t.progressPercent = 100;
      t.endedAt = new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' });
    }
  }
</script>

<AppBar title="Guardian & Kawal Perjalanan" subtitle="Pendamping Lokasi Langsung, Izin Perjalanan & Riwayat Rute" />

<div class="guardian-container">
  <!-- Panel Perjalanan Aktif (guardian_map_screen.dart parity) -->
  {#each mekaarStore.trips.filter((t) => t.status === 'ongoing') as activeTrip (activeTrip.id)}
    <div class="sketch-card" style="padding: 16px; margin-bottom: 16px; background: #ECFDF5;">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
        <span class="sketch-badge" style="background: #10B981; color: #FFFFFF;">🛵 PERJALANAN SEDANG DIKAWAL</span>
        <span style="font-size: 0.82rem; font-weight: 700;">ETA {activeTrip.etaMinutes} Menit</span>
      </div>
      <h3 style="font-size: 1.05rem; margin-bottom: 4px;">
        {activeTrip.originName} → {activeTrip.destinationName}
      </h3>
      <p style="font-size: 0.83rem; color: var(--text-secondary); margin-bottom: 10px;">
        Guardian Pemantau: <strong>{activeTrip.activeGuardians.join(', ')}</strong>
      </p>
      <div class="trip-progress-track">
        <div class="trip-progress-fill" style="width: {activeTrip.progressPercent}%;"></div>
      </div>
      <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 12px;">
        <Button size="sm" variant="primary" onclick={() => completeTrip(activeTrip.id)}>
          ✅ Saya Sudah Tiba dengan Aman
        </Button>
      </div>
    </div>
  {/each}

  <!-- Mulai Kawal Perjalanan Baru -->
  <CardExpanded title="🧭 Mulai Sesi Kawal Perjalanan Baru" subtitle="Bagikan rute & estimasi tiba ke Guardian" defaultOpen={false}>
    <div style="display: flex; flex-direction: column; gap: 10px;">
      <input type="text" class="sketch-input" placeholder="Titik Keberangkatan..." bind:value={originInput} />
      <input type="text" class="sketch-input" placeholder="Titik Tujuan..." bind:value={destinationInput} />
      <label style="font-size: 0.84rem; font-weight: 700;">
        Estimasi Waktu Tempuh (Menit): {etaInput} mnt
        <input type="range" min="5" max="120" step="5" bind:value={etaInput} style="width: 100%; margin-top: 4px;" />
      </label>
      <Button variant="primary" onclick={startNewTrip}>Mulai Kawal Perjalanan</Button>
    </div>
  </CardExpanded>

  <!-- Daftar Guardian & Permintaan Pendamping (guardian_dashboard_screen & guardian_request_screen parity) -->
  <h3 style="font-family: var(--font-crayon); font-size: 1.05rem; margin: 14px 0 10px;">
    🛡️ Daftar Guardian Pendamping ({mekaarStore.guardians.length})
  </h3>
  <div style="display: flex; flex-direction: column; gap: 10px; margin-bottom: 18px;">
    {#each mekaarStore.guardians as g (g.id)}
      <Card>
        <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px; flex-wrap: wrap;">
          <div style="display: flex; align-items: center; gap: 12px;">
            <SketchAvatar name={g.name} color={g.avatarColor} online={g.status !== 'pending_request'} />
            <div>
              <div style="display: flex; align-items: center; gap: 6px;">
                <strong>{g.name}</strong>
                <span class="sketch-badge" style="background: {g.status === 'active' ? '#DCFCE7' : '#FEF3C7'};">
                  {g.status === 'active' ? 'Mengawal' : g.status === 'standby' ? 'Siaga' : 'Menunggu Konfirmasi'}
                </span>
              </div>
              <div style="font-size: 0.82rem; color: var(--text-secondary);">
                {g.relation} ({g.username}) • 🔋 {g.batteryPercent}% • {g.lastUpdated}
              </div>
            </div>
          </div>

          <div style="display: flex; gap: 6px;">
            {#if g.status === 'pending_request'}
              <Button size="sm" variant="primary" onclick={() => (g.status = 'active')}>Setujui Guardian</Button>
            {:else}
              <Button
                size="sm"
                variant={g.canViewLiveLocation ? 'primary' : 'secondary'}
                onclick={() => (g.canViewLiveLocation = !g.canViewLiveLocation)}
              >
                📍 Lokasi: {g.canViewLiveLocation ? 'Aktif' : 'Nonaktif'}
              </Button>
            {/if}
          </div>
        </div>
      </Card>
    {/each}
  </div>

  <!-- Riwayat Perjalanan (trip_history_screen.dart parity) -->
  <h3 style="font-family: var(--font-crayon); font-size: 1.05rem; margin: 14px 0 10px;">
    📜 Riwayat Perjalanan Terkawal
  </h3>
  <div style="display: flex; flex-direction: column; gap: 10px;">
    {#each mekaarStore.trips.filter((t) => t.status !== 'ongoing') as trip (trip.id)}
      <Card>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <div>
            <strong>{trip.originName} → {trip.destinationName}</strong>
            <div style="font-size: 0.8rem; color: var(--text-secondary);">
              {trip.startedAt} — {trip.endedAt} • Dikawal oleh {trip.activeGuardians.join(', ')}
            </div>
          </div>
          <span class="sketch-badge" style="background: #DCFCE7;">✓ Selesai Aman</span>
        </div>
      </Card>
    {/each}
  </div>
</div>

<style>
  .guardian-container {
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
  .trip-progress-track {
    width: 100%;
    height: 12px;
    background: #FFFFFF;
    border: 2px solid var(--sketch-border-color);
    border-radius: 999px;
    overflow: hidden;
  }
  .trip-progress-fill {
    height: 100%;
    background: #10B981;
  }
</style>
