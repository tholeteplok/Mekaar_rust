<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let subView = $state<'radar' | 'nearby' | 'places'>('radar');
  let newPlaceName = $state('');
  let newPlaceAddress = $state('');
  let selectedPin = $state<string>('Teplok (Anda)');

  function addSavedPlace() {
    if (!newPlaceName.trim()) return;
    mekaarStore.savedPlaces = [
      {
        id: `plc-${Date.now()}`,
        name: newPlaceName.trim(),
        category: 'favorite',
        address: newPlaceAddress.trim() || 'Surakarta, Jawa Tengah',
        latitude: -7.5575,
        longitude: 110.8325,
        radiusMeters: 150,
        autoNotifyGuardian: true
      },
      ...mekaarStore.savedPlaces
    ];
    newPlaceName = '';
    newPlaceAddress = '';
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
  {#if subView === 'radar'}
    <div class="map-canvas sketch-card">
      <div class="map-grid-overlay"></div>

      <!-- Pin Lokasi Pengguna -->
      <button
        type="button"
        class="map-pin self-pin"
        style="top: 48%; left: 48%;"
        onclick={() => (selectedPin = 'Teplok (Anda) • -7.5568, 110.8318')}
      >
        📍 Anda
      </button>

      <!-- Pin Teman Sekitar -->
      {#each mekaarStore.nearbyFriends as friend, idx}
        <button
          type="button"
          class="map-pin friend-pin"
          style="top: {25 + idx * 22}%; left: {30 + idx * 24}%; background: {friend.avatarColor};"
          onclick={() => (selectedPin = `${friend.displayName} (${friend.distanceMeters}m • ${friend.statusMessage})`)}
        >
          👤 {friend.displayName} ({friend.distanceMeters}m)
        </button>
      {/each}

      <!-- Pin Lokasi Tersimpan -->
      {#each mekaarStore.savedPlaces as plc, idx}
        <button
          type="button"
          class="map-pin place-pin"
          style="bottom: {18 + idx * 18}%; right: {15 + idx * 20}%;"
          onclick={() => (selectedPin = `🏠 ${plc.name} (${plc.address})`)}
        >
          🛡️ {plc.name}
        </button>
      {/each}

      <div class="map-footer-bar sketch-card">
        <div>
          <strong>Titik Dipilih:</strong> {selectedPin}
        </div>
        <span class="sketch-badge" style="background: #DCFCE7;">GPS E2EE Aktif</span>
      </div>
    </div>
  {:else if subView === 'nearby'}
    <div style="display: flex; flex-direction: column; gap: 10px;">
      {#each mekaarStore.nearbyFriends as friend (friend.id)}
        <Card>
          <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px;">
            <div style="display: flex; align-items: center; gap: 12px;">
              <SketchAvatar name={friend.displayName} color={friend.avatarColor} online={true} />
              <div>
                <strong>{friend.displayName}</strong>
                <span style="font-size: 0.8rem; color: var(--text-secondary); margin-left: 6px;">{friend.username}</span>
                <div style="font-size: 0.84rem; color: var(--text-secondary);">{friend.statusMessage}</div>
                <div style="font-size: 0.75rem; color: var(--text-muted);">
                  🧭 {friend.distanceMeters} meter ({friend.bearingLabel}) • Diperbarui {friend.lastUpdated}
                </div>
              </div>
            </div>
            <Button
              size="sm"
              variant="secondary"
              onclick={() => {
                selectedPin = `${friend.displayName} (${friend.distanceMeters}m)`;
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
        <div style="font-weight: 700; margin-bottom: 8px;">+ Tambah Lokasi Tersimpan / Zona Aman Baru</div>
        <div style="display: flex; gap: 8px; flex-wrap: wrap;">
          <input
            type="text"
            class="sketch-input"
            style="flex: 1; min-width: 180px;"
            placeholder="Nama tempat (mis. Kampus / Posko)..."
            bind:value={newPlaceName}
          />
          <input
            type="text"
            class="sketch-input"
            style="flex: 1; min-width: 220px;"
            placeholder="Alamat lengkap..."
            bind:value={newPlaceAddress}
          />
          <Button variant="primary" onclick={addSavedPlace}>Simpan Lokasi</Button>
        </div>
      </Card>

      {#each mekaarStore.savedPlaces as place (place.id)}
        <Card>
          <div style="display: flex; align-items: center; justify-content: space-between;">
            <div>
              <div style="display: flex; align-items: center; gap: 8px;">
                <strong>📍 {place.name}</strong>
                <span class="sketch-badge" style="background: #DBEAFE;">Radius {place.radiusMeters}m</span>
                {#if place.autoNotifyGuardian}
                  <span class="sketch-badge" style="background: #DCFCE7;">Auto-Notif Guardian</span>
                {/if}
              </div>
              <div style="font-size: 0.84rem; color: var(--text-secondary); margin-top: 3px;">{place.address}</div>
              <div style="font-size: 0.74rem; font-family: var(--font-mono); color: var(--text-muted);">
                Koordinat: {place.latitude}, {place.longitude}
              </div>
            </div>
            <Button
              size="sm"
              variant="danger"
              onclick={() => (mekaarStore.savedPlaces = mekaarStore.savedPlaces.filter((p) => p.id !== place.id))}
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
  .map-canvas {
    position: relative;
    height: 430px;
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
