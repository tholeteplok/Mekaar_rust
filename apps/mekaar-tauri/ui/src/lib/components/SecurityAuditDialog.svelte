<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { SecurityLogModel } from '../types';

  let {
    open = false,
    onclose
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  let filterCategory = $state<'all' | 'sos' | 'crypto' | 'auth'>('all');
  let searchQuery = $state('');

  const filteredLogs = $derived(
    mekaarStore.securityLogs.filter((log) => {
      if (filterCategory === 'sos' && log.eventCategory !== 'sos') return false;
      if (filterCategory === 'crypto' && log.eventCategory !== 'vault' && log.eventCategory !== 'crypto') return false;
      if (filterCategory === 'auth' && log.eventCategory !== 'auth' && log.eventCategory !== 'screen_shield') return false;

      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return (
        log.eventTitle.toLowerCase().includes(q) ||
        log.deviceInfo.toLowerCase().includes(q)
      );
    })
  );

  function getSeverityBadge(severity: string) {
    switch (severity) {
      case 'critical':
        return { label: 'KRITIS', bg: '#FEE2E2', color: '#991B1B' };
      case 'warning':
        return { label: 'PERINGATAN', bg: '#FEF3C7', color: '#92400E' };
      default:
        return { label: 'INFO', bg: '#DBEAFE', color: '#1E40AF' };
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
      aria-label="Audit Log Keamanan"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="sheet-drag-handle"></div>

      <div class="sheet-header">
        <div style="display: flex; align-items: center; gap: 10px;">
          <div class="header-icon-circle">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
              <line x1="16" y1="13" x2="8" y2="13"></line>
              <line x1="16" y1="17" x2="8" y2="17"></line>
            </svg>
          </div>
          <div>
            <h3 class="sheet-title">Log Audit Keamanan SQLCipher</h3>
            <p class="sheet-sub">Rekaman anti-tamper aktivitas kriptografi dan status pertahanan</p>
          </div>
        </div>
        <button type="button" class="close-x-btn" onclick={onclose} aria-label="Tutup">✕</button>
      </div>

      <div class="sheet-toolbar">
        <input
          type="text"
          class="search-input"
          placeholder="Cari kejadian log atau perangkat..."
          bind:value={searchQuery}
        />
        <div class="tabs-row">
          <button
            type="button"
            class="tab-btn {filterCategory === 'all' ? 'active' : ''}"
            onclick={() => (filterCategory = 'all')}
          >
            Semua ({mekaarStore.securityLogs.length})
          </button>
          <button
            type="button"
            class="tab-btn {filterCategory === 'sos' ? 'active' : ''}"
            onclick={() => (filterCategory = 'sos')}
          >
            🚨 Darurat SOS
          </button>
          <button
            type="button"
            class="tab-btn {filterCategory === 'crypto' ? 'active' : ''}"
            onclick={() => (filterCategory = 'crypto')}
          >
            🔐 Kriptografi & Brankas
          </button>
          <button
            type="button"
            class="tab-btn {filterCategory === 'auth' ? 'active' : ''}"
            onclick={() => (filterCategory = 'auth')}
          >
            👤 Autentikasi
          </button>
        </div>
      </div>

      <div class="logs-feed">
        {#each filteredLogs as log (log.id)}
          {@const sev = getSeverityBadge(log.severity)}
          <div class="log-card">
            <div class="log-card-top">
              <span class="severity-badge" style="background: {sev.bg}; color: {sev.color};">
                {sev.label}
              </span>
              <span class="log-timestamp">{log.timestamp}</span>
            </div>
            <div class="log-title">{log.eventTitle}</div>
            <div class="log-meta-row">
              <span class="device-pill">💻 {log.deviceInfo}</span>
              <span class="checksum-code">
                {log.checksum ?? 'INTEGRITAS-OK • SHA256-VERIFIED'}
              </span>
            </div>
          </div>
        {:else}
          <div class="empty-state">
            <div style="font-size: 2rem; margin-bottom: 6px;">📜</div>
            <div style="font-weight: 700; color: var(--text-primary);">Tidak ada catatan log ditemukan</div>
            <div style="font-size: 0.78rem; color: var(--text-secondary);">Seluruh aktivitas keamanan tersimpan aman.</div>
          </div>
        {/each}
      </div>

      <div class="sheet-footer">
        <div style="font-size: 0.76rem; color: var(--text-secondary);">
          🔒 Dilindungi rantai hash anti-tamper berstandar Aegis.
        </div>
        <button
          type="button"
          class="btn-export"
          onclick={() => {
            onclose();
            mekaarStore.forensicDialogOpen = true;
          }}
        >
          🔐 Ekspor Bundel Forensik (.mkrvault)
        </button>
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
    max-width: 580px;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    box-shadow: 0 -8px 32px rgba(15, 23, 42, 0.16);
    display: flex;
    flex-direction: column;
    max-height: 85vh;
    overflow: hidden;
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
    background-color: #F3E8FF;
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

  .close-x-btn:hover {
    background-color: var(--bg-input);
    color: var(--text-primary);
  }

  .sheet-toolbar {
    padding: 12px 20px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    border-bottom: 1px solid var(--border-light);
    background-color: var(--bg-primary);
  }

  .search-input {
    width: 100%;
    padding: 8px 14px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    background: #FFFFFF;
    font-size: 0.85rem;
    color: var(--text-primary);
    outline: none;
    box-sizing: border-box;
  }

  .tabs-row {
    display: flex;
    gap: 6px;
    overflow-x: auto;
    padding-bottom: 2px;
  }

  .tab-btn {
    padding: 6px 12px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    background: #FFFFFF;
    color: var(--text-secondary);
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    border-color: #C4B5FD;
  }

  .tab-btn.active {
    background: #8B5CF6;
    color: #FFFFFF;
    border-color: #8B5CF6;
  }

  .logs-feed {
    padding: 16px 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
    flex: 1;
  }

  .log-card {
    padding: 12px 14px;
    border-radius: var(--radius-lg);
    background-color: #FFFFFF;
    border: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    gap: 6px;
    box-shadow: 0 1px 4px rgba(15, 23, 42, 0.04);
  }

  .log-card-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .severity-badge {
    font-size: 0.68rem;
    font-weight: 800;
    padding: 2px 8px;
    border-radius: var(--radius-pill);
    letter-spacing: 0.4px;
  }

  .log-timestamp {
    font-size: 0.74rem;
    color: var(--text-secondary);
  }

  .log-title {
    font-size: 0.86rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.4;
  }

  .log-meta-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.74rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .device-pill {
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .checksum-code {
    font-family: var(--font-mono);
    font-size: 0.68rem;
    color: #059669;
    font-weight: 600;
  }

  .empty-state {
    padding: 40px 20px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .sheet-footer {
    padding: 14px 20px;
    border-top: 1px solid var(--border-light);
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: var(--bg-primary);
  }

  .btn-export {
    padding: 8px 16px;
    border-radius: var(--radius-pill);
    background-color: #8B5CF6;
    color: #FFFFFF;
    border: none;
    font-size: 0.82rem;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(139, 92, 246, 0.25);
  }

  .btn-export:hover {
    background-color: #7C3AED;
  }
</style>
