<script lang="ts">
  import { mekaarStore } from '../stores';
  import BottomSheet from './BottomSheet.svelte';

  let {
    open = false,
    onclose
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  $effect(() => {
    if (open) {
      mekaarStore.loadConnectedDevices();
    }
  });

  function getPlatformIcon(platform: string) {
    switch (platform) {
      case 'android':
        return '📱';
      case 'desktop':
        return '💻';
      case 'web':
        return '🌐';
      default:
        return '🔒';
    }
  }
</script>

<BottomSheet
  {open}
  title="Perangkat Terhubung"
  subtitle="Manajemen sesi aktif terenkripsi E2EE antar-node"
  {onclose}
  maxWidth="540px"
>
  <div class="sheet-body">
    <div class="devices-list">
      {#each mekaarStore.connectedDevices as dev (dev.id)}
        <div class="device-card {dev.isCurrent ? 'current' : ''}">
          <div class="device-icon-box">
            {getPlatformIcon(dev.platform)}
          </div>
          <div class="device-info">
            <div class="device-name-row">
              <span class="device-name">{dev.deviceName}</span>
              {#if dev.isCurrent}
                <span class="current-badge">Perangkat Ini</span>
              {/if}
            </div>
            <div class="device-ip">{dev.ipAddress}</div>
            <div class="device-active-time">Aktif: {dev.lastActive}</div>
          </div>
          {#if !dev.isCurrent}
            <button
              type="button"
              class="btn-revoke"
              onclick={() => mekaarStore.revokeConnectedDevice(dev.id)}
            >
              Cabut Izin
            </button>
          {/if}
        </div>
      {/each}
    </div>

    <div class="security-notice">
      🛡️ Setiap sesi memiliki pasangan kunci publik unik yang diverifikasi silang dengan Mekaar Mesh Network.
    </div>
  </div>
</BottomSheet>

<style>
  .sheet-body {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .devices-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .device-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-light);
    background-color: var(--bg-card);
    transition: all 0.15s ease;
  }

  .device-card.current {
    border-color: var(--brand-blue);
    background-color: var(--brand-blue-subtle);
  }

  .device-icon-box {
    width: 42px;
    height: 42px;
    border-radius: 50%;
    background-color: var(--bg-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .device-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }

  .device-name-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .device-name {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--text-primary);
    font-family: var(--font-active);
  }

  .current-badge {
    font-size: 0.68rem;
    font-weight: 800;
    background-color: #059669;
    color: #FFFFFF;
    padding: 2px 8px;
    border-radius: var(--radius-pill);
    letter-spacing: 0.3px;
  }

  .device-ip {
    font-family: var(--font-mono);
    font-size: 0.74rem;
    color: var(--text-secondary);
  }

  .device-active-time {
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .btn-revoke {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 6px 14px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid #FDA4AF;
    color: #E11D48;
    font-size: 0.78rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-revoke:hover {
    background-color: #FFE4E6;
  }

  .security-notice {
    padding: 10px 14px;
    background-color: var(--bg-primary);
    border-radius: var(--radius-md);
    font-size: 0.78rem;
    color: var(--text-secondary);
    line-height: 1.45;
  }
</style>
