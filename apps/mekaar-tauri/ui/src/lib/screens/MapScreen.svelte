<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import type { SavedPlaceModel } from '../types';

  let subView = $state<'radar' | 'nearby' | 'places'>('radar');
  let newPlaceName = $state('');
  let newPlaceAddress = $state('');
  let newPlaceCategory = $state<SavedPlaceModel['category']>('safe_zone');
  let newPlaceRadius = $state(150);
  let newPlaceAutoNotify = $state(true);
  let selectedPin = $state<string>('Teplok (Anda) • Zona Aman Aktif');

  function calculateDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const r = 6371000;
    const p1 = (lat1 * Math.PI) / 180;
    const p2 = (lat2 * Math.PI) / 180;
    const dp = ((lat2 - lat1) * Math.PI) / 180;
    const dl = ((lon2 - lon1) * Math.PI) / 180;
    const a =
      Math.sin(dp / 2) * Math.sin(dp / 2) +
      Math.cos(p1) * Math.cos(p2) * Math.sin(dl / 2) * Math.sin(dl / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return Math.round(r * c);
  }

  async function addSavedPlace() {
    if (!newPlaceName.trim()) return;
    await mekaarStore.addSavedPlace({
      name: newPlaceName.trim(),
      category: newPlaceCategory,
      address: newPlaceAddress.trim() || 'Surakarta, Jawa Tengah',
      latitude: mekaarStore.telemetry.latitude,
      longitude: mekaarStore.telemetry.longitude,
      radiusMeters: newPlaceRadius,
      autoNotifyGuardian: newPlaceAutoNotify
    });
    newPlaceName = '';
    newPlaceAddress = '';
  }

  async function deleteSavedPlace(placeId: string) {
    await mekaarStore.deleteSavedPlace(placeId);
  }

  async function setSimulatedLocation(lat: number, lng: number, label: string) {
    await mekaarStore.updateLiveTelemetry(lat, lng, mekaarStore.telemetry.batteryLevel);
    selectedPin = `Lokasi Disimulasikan: ${label} (${lat.toFixed(4)}, ${lng.toFixed(4)})`;
  }
</script>

<AppBar title="Peta & Lokasi Langsung" subtitle="Radar Teman Sekitar & Zona Aman (Saved Places)">
  {#snippet actions()}
    <Button size="sm" variant={subView === 'radar' ? 'primary' : 'secondary'} onclick={() => (subView = 'radar')}>
      🗺️ Peta Interaktif
    </Button>
    <Button size="sm" variant={subView === 'nearby' ? 'primary' : 'secondary'} onclick={() => (subView = 'nearby')}>
      📡 Teman Sekitar ({mekaarStore.nearbyFriends.length})
    </Button>
    <Button size="sm" variant={subView === 'places' ? 'primary' : 'secondary'} onclick={() => (subView = 'places')}>
      📌 Lokasi Tersimpan ({mekaarStore.savedPlaces.length})
    </Button>
  {/snippet}
</AppBar>

<div class="map-container">
  <!-- Status Bar Geofence & Telemetri -->
  <div class="sketch-card map-telemetry-header">
    <div style="display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px;">
      <div style="display: flex; align-items: center; gap: 8px;">
        <span class="sketch-badge" style="background: {mekaarStore.telemetry.isInSafeZone ? '#DCFCE7' : '#FEF3C7'}; font-weight: 700;">
          {mekaarStore.telemetry.isInSafeZone ? '🛡️ ' + (mekaarStore.telemetry.activeZoneName ?? 'Zona Aman') : '⚠️ Di Luar Zona Aman'}
        </span>
        <span style="font-size: 0.82rem; color: var(--text-secondary);">
          🔋 Baterai: <strong>{mekaarStore.telemetry.batteryLevel}%</strong>
        </span>
      </div>
      <div style="display: flex; gap: 6px; align-items: center;">
        <span style="font-size: 0.78rem; color: var(--text-muted);">Simulasi GPS:</span>
        <Button size="sm" variant="secondary" onclick={() => setSimulatedLocation(-7.5568, 110.8521, 'Rumah Utama')}>
          🏠 Rumah
        </Button>
        <Button size="sm" variant="secondary" onclick={() => setSimulatedLocation(-7.5682, 110.7984, 'Server Lab')}>
          🏢 Kantor
        </Button>
        <Button size="sm" variant="secondary" onclick={() => setSimulatedLocation(-7.5920, 110.7600, 'Jalan Raya Luar')}>
          🛵 Luar
        </Button>
      </div>
    </div>
  </div>

  {#if subView === 'radar'}
    <div class="map-canvas sketch-card">
      <div class="map-grid-overlay"></div>

      <!-- Lingkaran Radius Geofence Rumah -->
      <div class="geofence-circle" style="top: 40%; left: 40%; width: 140px; height: 140px;"></div>

      <!-- Pin Lokasi Pengguna -->
      <button
        type="button"
        class="map-pin self-pin"
        style="top: 48%; left: 48%;"
        onclick={() => (selectedPin = `Anda (${mekaarStore.telemetry.latitude.toFixed(4)}, ${mekaarStore.telemetry.longitude.toFixed(4)}) • ${mekaarStore.telemetry.isInSafeZone ? 'Di Zona Aman' : 'Di Luar Zona Aman'}`)}
      >
        📍 Anda ({mekaarStore.telemetry.isInSafeZone ? 'Zona Aman' : 'Luar'})
      </button>

      <!-- Pin Teman Sekitar dengan jarak dinamis -->
      {#each mekaarStore.nearbyFriends as friend, idx}
        {@const dynamicDist = calculateDistance(
          mekaarStore.telemetry.latitude,
          mekaarStore.telemetry.longitude,
          friend.latitude,
          friend.longitude
        )}
        <button
          type="button"
          class="map-pin friend-pin"
          style="top: {25 + idx * 22}%; left: {28 + idx * 25}%; background: {friend.avatarColor};"
          onclick={() => (selectedPin = `${friend.displayName} • ${dynamicDist}m • ${friend.statusMessage}`)}
        >
          👤 {friend.displayName} ({dynamicDist}m)
        </button>
      {/each}

      <!-- Pin Lokasi Tersimpan -->
      {#each mekaarStore.savedPlaces as plc, idx}
        <button
          type="button"
          class="map-pin place-pin"
          style="bottom: {16 + (idx % 3) * 20}%; right: {14 + (idx % 3) * 22}%;"
          onclick={() => (selectedPin = `🏠 ${plc.name} (Radius ${plc.radiusMeters}m) • ${plc.address}`)}
        >
          🛡️ {plc.name}
        </button>
      {/each}

      <div class="map-footer-bar sketch-card">
        <div>
          <strong>Titik Terpilih:</strong> {selectedPin}
        </div>
        <span class="sketch-badge" style="background: #DCFCE7;">GPS E2EE Aktif</span>
      </div>
    </div>
  {:else if subView === 'nearby'}
    <div style="display: flex; flex-direction: column; gap: 10px;">
      {#each mekaarStore.nearbyFriends as friend (friend.id)}
        {@const dynamicDist = calculateDistance(
          mekaarStore.telemetry.latitude,
          mekaarStore.telemetry.longitude,
          friend.latitude,
          friend.longitude
        )}
        <Card>
          <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px;">
            <div style="display: flex; align-items: center; gap: 12px;">
              <SketchAvatar name={friend.displayName} color={friend.avatarColor} online={true} />
              <div>
                <strong>{friend.displayName}</strong>
                <span style="font-size: 0.8rem; color: var(--text-secondary); margin-left: 6px;">{friend.username}</span>
                <div style="font-size: 0.84rem; color: var(--text-secondary);">{friend.statusMessage}</div>
                <div style="font-size: 0.75rem; color: var(--text-muted);">
                  🧭 <strong>{dynamicDist} meter</strong> ({friend.bearingLabel}) • Terverifikasi Enkripsi E2EE
                </div>
              </div>
            </div>
            <Button
              size="sm"
              variant="secondary"
              onclick={() => {
                selectedPin = `${friend.displayName} (${dynamicDist}m)`;
                subView = 'radar';
              }}
            >
              Lihat di Peta
            </Button>
          </div>
        </Card>
      {/each}
    </div>
  {:else}
    <div style="display: flex; flex-direction: column; gap: 12px;">
      <Card>
        <div style="font-weight: 700; margin-bottom: 8px;">+ Tambah Zona Aman / Lokasi Tersimpan (SQLCipher)</div>
        <div style="display: flex; flex-direction: column; gap: 10px;">
          <div style="display: flex; gap: 8px; flex-wrap: wrap;">
            <input
              type="text"
              class="sketch-input"
              style="flex: 2; min-width: 180px;"
              placeholder="Nama tempat (mis. Posko Solo / Rumah)..."
              bind:value={newPlaceName}
            />
            <select class="sketch-input" style="flex: 1; min-width: 130px;" bind:value={newPlaceCategory}>
              <option value="safe_zone">🛡️ Zona Aman</option>
              <option value="home">🏠 Rumah</option>
              <option value="office">🏢 Kantor / Posko</option>
              <option value="favorite">⭐ Favorit</option>
            </select>
          </div>
          <input
            type="text"
            class="sketch-input"
            placeholder="Alamat lengkap lokasi..."
            bind:value={newPlaceAddress}
          />
          <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 8px;">
            <label style="font-size: 0.82rem; font-weight: 700;">
              Radius Geofence: {newPlaceRadius} meter
              <input type="range" min="50" max="500" step="25" bind:value={newPlaceRadius} style="margin-left: 8px;" />
            </label>
            <label style="font-size: 0.82rem; display: flex; align-items: center; gap: 6px;">
              <input type="checkbox" bind:checked={newPlaceAutoNotify} />
              Otomatis Notifikasi Guardian saat tiba
            </label>
            <Button variant="primary" onclick={addSavedPlace}>Simpan ke Enkripsi Lokal</Button>
          </div>
        </div>
      </Card>

      {#each mekaarStore.savedPlaces as place (place.id)}
        <Card>
          <div style="display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px;">
            <div>
              <div style="display: flex; align-items: center; gap: 8px;">
                <strong>
                  {place.category === 'home' ? '🏠' : place.category === 'office' ? '🏢' : '🛡️'} {place.name}
                </strong>
                <span class="sketch-badge" style="background: #DBEAFE;">Radius {place.radiusMeters}m</span>
                {#if place.autoNotifyGuardian}
                  <span class="sketch-badge" style="background: #DCFCE7;">Auto-Notif Guardian</span>
                {/if}
              </div>
              <div style="font-size: 0.84rem; color: var(--text-secondary); margin-top: 3px;">{place.address}</div>
              <div style="font-size: 0.74rem; font-family: var(--font-mono); color: var(--text-muted);">
                Koordinat: {place.latitude.toFixed(4)}, {place.longitude.toFixed(4)}
              </div>
            </div>
            <Button
              size="sm"
              variant="danger"
              onclick={() => deleteSavedPlace(place.id)}
            >
              Hapus
            </Button>
          </div>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<style>
  .map-container {
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
  .map-telemetry-header {
    padding: 10px 14px;
    margin-bottom: 14px;
    background: var(--bg-secondary);
  }
  .map-canvas {
    position: relative;
    height: 440px;
    background: linear-gradient(135deg, #E0F2FE 0%, #DCFCE7 55%, #FEF3C7 100%);
    overflow: hidden;
  }
  .map-grid-overlay {
    position: absolute;
    inset: 0;
    background-image: radial-gradient(#1C1B18 1px, transparent 1px);
    background-size: 28px 28px;
    opacity: 0.15;
  }
  .geofence-circle {
    position: absolute;
    border: 2px dashed #10B981;
    background: rgba(16, 185, 129, 0.15);
    border-radius: 50%;
    pointer-events: none;
    transform: translate(-50%, -50%);
  }
  .map-pin {
    position: absolute;
    padding: 6px 12px;
    border: 2px solid #1C1B18;
    border-radius: 999px;
    font-weight: 700;
    font-size: 0.8rem;
    box-shadow: var(--sketch-shadow-sm);
    cursor: pointer;
    background: #FFFFFF;
  }
  .self-pin {
    background: #2563EB;
    color: #FFFFFF;
    z-index: 5;
  }
  .place-pin {
    background: #FEF3C7;
  }
  .map-footer-bar {
    position: absolute;
    bottom: 12px;
    left: 12px;
    right: 12px;
    padding: 10px 14px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.85rem;
  }
</style>

