<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import CardExpanded from '../components/CardExpanded.svelte';
  import Button from '../components/Button.svelte';
  import Dialog from '../components/Dialog.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let originInput = $state('');
  let destinationInput = $state('');
  let etaInput = $state(20);

  // Modal Tambah Guardian Baru
  let showAddGuardianDialog = $state(false);
  let newGuardianName = $state('');
  let newGuardianUsername = $state('');
  let newGuardianRelation = $state('Rekan Siaga');
  let newGuardianCanLoc = $state(true);
  let newGuardianCanSos = $state(true);

  async function startNewTrip() {
    if (!originInput.trim() || !destinationInput.trim()) return;
    const activeGuardians = mekaarStore.guardians
      .filter((g) => g.status === 'active')
      .map((g) => g.name);

    await mekaarStore.startTrip({
      originName: originInput.trim(),
      destinationName: destinationInput.trim(),
      etaMinutes: etaInput,
      activeGuardians: activeGuardians
    });

    originInput = '';
    destinationInput = '';
  }

  async function completeTrip(tripId: string) {
    await mekaarStore.completeTrip(tripId, 'arrived_confirmed');
  }

  async function handleAddGuardian() {
    if (!newGuardianName.trim() || !newGuardianUsername.trim()) return;
    const cleanUsername = newGuardianUsername.trim().startsWith('@')
      ? newGuardianUsername.trim()
      : `@${newGuardianUsername.trim()}`;

    await mekaarStore.addGuardian({
      name: newGuardianName.trim(),
      username: cleanUsername,
      relation: newGuardianRelation,
      canViewLiveLocation: newGuardianCanLoc,
      canReceiveSosAlert: newGuardianCanSos
    });

    newGuardianName = '';
    newGuardianUsername = '';
    showAddGuardianDialog = false;
  }

  async function handleApproveGuardian(guardianId: string) {
    await mekaarStore.updateGuardianStatus(guardianId, 'active');
  }

  async function handleDeleteGuardian(guardianId: string) {
    await mekaarStore.deleteGuardian(guardianId);
  }
</script>

<AppBar title="Guardian & Kawal Perjalanan" subtitle="Pendamping Lokasi Langsung, Izin Perjalanan & Riwayat Rute">
  {#snippet actions()}
    <Button size="sm" variant="primary" onclick={() => (showAddGuardianDialog = true)}>
      + Tambah Guardian
    </Button>
  {/snippet}
</AppBar>

<div class="guardian-container">
  <!-- Status Telemetri Langsung & Geofencing -->
  <div class="sketch-card telemetry-card">
    <div style="display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px;">
      <div style="display: flex; align-items: center; gap: 10px;">
        <span class="sketch-badge" style="background: {mekaarStore.telemetry.isInSafeZone ? '#DCFCE7' : '#FEF3C7'}; font-weight: 700;">
          {mekaarStore.telemetry.isInSafeZone ? '🛡️ ' + (mekaarStore.telemetry.activeZoneName ?? 'Zona Aman Aktif') : '⚠️ Di Luar Zona Aman'}
        </span>
        <span style="font-size: 0.82rem; color: var(--text-secondary);">
          🔋 Baterai: <strong>{mekaarStore.telemetry.batteryLevel}%</strong>
        </span>
      </div>
      <div style="font-size: 0.78rem; font-family: var(--font-mono); color: var(--text-muted);">
        GPS: {mekaarStore.telemetry.latitude.toFixed(4)}, {mekaarStore.telemetry.longitude.toFixed(4)} • E2EE Enclave
      </div>
    </div>
  </div>

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
  <div style="display: flex; justify-content: space-between; align-items: center; margin: 18px 0 10px;">
    <h3 style="font-family: var(--font-crayon); font-size: 1.05rem; margin: 0;">
      🛡️ Daftar Guardian Pendamping ({mekaarStore.guardians.length})
    </h3>
    <Button size="sm" variant="secondary" onclick={() => (showAddGuardianDialog = true)}>
      + Tambah
    </Button>
  </div>

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

          <div style="display: flex; gap: 6px; align-items: center;">
            {#if g.status === 'pending_request'}
              <Button size="sm" variant="primary" onclick={() => handleApproveGuardian(g.id)}>Setujui Guardian</Button>
            {:else}
              <Button
                size="sm"
                variant={g.canViewLiveLocation ? 'primary' : 'secondary'}
                onclick={() => (g.canViewLiveLocation = !g.canViewLiveLocation)}
              >
                📍 Lokasi: {g.canViewLiveLocation ? 'Aktif' : 'Nonaktif'}
              </Button>
            {/if}
            <Button size="sm" variant="danger" onclick={() => handleDeleteGuardian(g.id)}>
              ✕
            </Button>
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
              {trip.startedAt} — {trip.endedAt ?? 'Selesai'} • Dikawal oleh {trip.activeGuardians.join(', ')}
            </div>
          </div>
          <span class="sketch-badge" style="background: #DCFCE7;">✓ Selesai Aman</span>
        </div>
      </Card>
    {/each}
  </div>
</div>

<!-- Dialog Tambah Guardian Baru -->
<Dialog open={showAddGuardianDialog} title="🛡️ Tambah Guardian Pendamping" onclose={() => (showAddGuardianDialog = false)}>
  <div style="display: flex; flex-direction: column; gap: 12px;">
    <p style="font-size: 0.84rem; color: var(--text-secondary); margin: 0;">
      Guardian akan menerima notifikasi status perjalanan dan koordinat langsung saat sesi aktif.
    </p>
    <div>
      <label for="guardian-name-input" style="font-size: 0.8rem; font-weight: 700; display: block; margin-bottom: 4px;">Nama Lengkap</label>
      <input id="guardian-name-input" type="text" class="sketch-input" placeholder="Nama lengkap..." bind:value={newGuardianName} />
    </div>
    <div>
      <label for="guardian-username-input" style="font-size: 0.8rem; font-weight: 700; display: block; margin-bottom: 4px;">Username Mekaar</label>
      <input id="guardian-username-input" type="text" class="sketch-input" placeholder="@username..." bind:value={newGuardianUsername} />
    </div>
    <div>
      <label for="guardian-relation-select" style="font-size: 0.8rem; font-weight: 700; display: block; margin-bottom: 4px;">Hubungan / Peran</label>
      <select id="guardian-relation-select" class="sketch-input" bind:value={newGuardianRelation}>
        <option value="Guardian Utama">Guardian Utama</option>
        <option value="Rekan Siaga">Rekan Siaga</option>
        <option value="Keluarga">Keluarga</option>
        <option value="Koordinator Lapangan">Koordinator Lapangan</option>
      </select>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px; margin: 4px 0;">
      <label style="font-size: 0.82rem; display: flex; align-items: center; gap: 8px;">
        <input type="checkbox" bind:checked={newGuardianCanLoc} /> Izinkan Melihat Lokasi Langsung
      </label>
      <label style="font-size: 0.82rem; display: flex; align-items: center; gap: 8px;">
        <input type="checkbox" bind:checked={newGuardianCanSos} /> Terima Notifikasi Peringatan SOS
      </label>
    </div>
    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
      <Button variant="secondary" onclick={() => (showAddGuardianDialog = false)}>Batal</Button>
      <Button variant="primary" onclick={handleAddGuardian}>Simpan Guardian</Button>
    </div>
  </div>
</Dialog>

<style>
  .guardian-container {
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
  .telemetry-card {
    padding: 12px 16px;
    margin-bottom: 16px;
    background: var(--bg-secondary);
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
