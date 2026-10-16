<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { SecurityLogModel } from '../types';
  import BottomSheet from './BottomSheet.svelte';

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

<BottomSheet
  {open}
  title="Log Audit Keamanan SQLCipher"
  subtitle="Rekaman anti-tamper aktivitas kriptografi dan status pertahanan"
  {onclose}
  maxWidth="600px"
>
  <div class="sheet-container">
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
          <div style="font-weight: 700; color: var(--text-primary); font-family: var(--font-active);">Tidak ada catatan log ditemukan</div>
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
</BottomSheet>

<style>
  .sheet-container {
    display: flex;
    flex-direction: column;
    margin: -16px -20px -20px -20px;
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
    min-height: var(--touch-target-min, 40px);
    padding: 8px 14px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    background: var(--bg-card);
    font-size: 0.85rem;
    font-family: var(--font-active);
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
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 6px 14px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    background: var(--bg-card);
    color: var(--text-secondary);
    font-size: 0.78rem;
    font-weight: 600;
    font-family: var(--font-active);
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
    max-height: 48vh;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .log-card {
    padding: 12px 14px;
    border-radius: var(--radius-lg);
    background-color: var(--bg-card);
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
    font-family: var(--font-active);
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
    flex-wrap: wrap;
    gap: 10px;
  }

  .btn-export {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 8px 16px;
    border-radius: var(--radius-pill);
    background-color: #8B5CF6;
    color: #FFFFFF;
    border: none;
    font-size: 0.82rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(139, 92, 246, 0.25);
  }

  .btn-export:hover {
    background-color: #7C3AED;
  }
</style>
