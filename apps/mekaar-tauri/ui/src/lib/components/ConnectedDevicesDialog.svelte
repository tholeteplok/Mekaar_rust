<script lang="ts">
  import { mekaarStore } from '../stores';

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

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={onclose}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="modal-sheet"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-label="Perangkat Terhubung"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="sheet-drag-handle"></div>

      <div class="sheet-header">
        <div style="display: flex; align-items: center; gap: 10px;">
          <div class="header-icon-circle">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
              <line x1="8" y1="21" x2="16" y2="21"></line>
              <line x1="12" y1="17" x2="12" y2="21"></line>
            </svg>
          </div>
          <div>
            <h3 class="sheet-title">Perangkat Terhubung</h3>
            <p class="sheet-sub">Manajemen sesi aktif terenkripsi E2EE antar-node</p>
          </div>
        </div>
        <button type="button" class="close-x-btn" onclick={onclose} aria-label="Tutup">✕</button>
      </div>

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
          🛡️ Setiap sesi memiliki pasangan kunci publik unik yang diverifikasi silang dengan relay teplok-pc.
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(15, 23, 42, 0.45);
    backdrop-filter: blur(4px);
    z-index: 999;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }

  .modal-sheet {
    background-color: var(--bg-card);
    width: 100%;
    max-width: 540px;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    box-shadow: 0 -8px 32px rgba(15, 23, 42, 0.16);
    display: flex;
    flex-direction: column;
    max-height: 85vh;
    overflow-y: auto;
    animation: slideUp 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideUp {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }

  .sheet-drag-handle {
    width: 36px;
    height: 4px;
    background-color: var(--border-light);
    border-radius: var(--radius-pill);
    margin: 10px auto 4px auto;
  }

  .sheet-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px 14px 20px;
    border-bottom: 1px solid var(--border-light);
  }

  .header-icon-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: var(--brand-blue-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .sheet-title {
    font-size: 1.05rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
  }

  .sheet-sub {
    font-size: 0.78rem;
    color: var(--text-secondary);
    margin: 2px 0 0 0;
  }

  .close-x-btn {
    background: none;
    border: none;
    font-size: 1.15rem;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 50%;
  }

  .sheet-body {
    padding: 18px 20px 24px 20px;
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
    background-color: #FFFFFF;
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
    padding: 6px 14px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid #FDA4AF;
    color: #E11D48;
    font-size: 0.78rem;
    font-weight: 700;
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
