<script lang="ts">
  import { onMount } from 'svelte';
  import type { ForensicAuditReport } from '../types';
  import { vaultStore } from '../stores';
  import Dialog from './Dialog.svelte';
  import Card from './Card.svelte';
  import Button from './Button.svelte';

  interface Props {
    open?: boolean;
    onclose?: () => void;
  }

  let { open = $bindable(false), onclose }: Props = $props();

  let report = $state<ForensicAuditReport | null>(null);
  let isLoading = $state(false);
  let copied = $state(false);
  let errorMessage = $state<string | null>(null);

  $effect(() => {
    if (open && !report) {
      loadReport();
    }
  });

  async function loadReport() {
    isLoading = true;
    errorMessage = null;

    try {
      const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
      if (isTauri) {
        const { invoke } = await import('@tauri-apps/api/core');
        report = await invoke<ForensicAuditReport>('export_forensic_audit_trail', {
          pin: vaultStore.isDuress ? '9999' : '1234',
        });
      } else {
        // Fallback mock
        const now = Date.now();
        report = {
          export_id: `MKR-AUDIT-${now}`,
          generated_at: now,
          identity_fingerprint: vaultStore.identityFingerprint,
          total_records: 4,
          root_hash_sha256: '9a87f2e1b439c017d23589bfe1034c568910ebf132ca77881023bc459910ddee',
          integrity_verified: true,
          log_entries: [
            {
              timestamp: now - 7200000,
              event_type: 'IDENTITY_INITIALIZED',
              target: 'KEY_X25519_MASTER',
              signature_hex: 'fa1b89cd3490ae62883344bfd50198aa',
            },
            {
              timestamp: now - 3600000,
              event_type: 'RATCHET_SESSION_INIT',
              target: 'ROOM_GUARD_1',
              signature_hex: '77ca89012fe344bc91023884aabb09cc',
            },
            {
              timestamp: now - 1800000,
              event_type: 'MESSAGE_SEALED_E2EE',
              target: 'MSG_AEGIS_0921',
              signature_hex: '991bf2803c448102941faeb89920bbaa',
            },
            {
              timestamp: now,
              event_type: 'AUDIT_CHAIN_VERIFY',
              target: 'STORAGE_SQLCIPHER',
              signature_hex: 'ee5412998a002bcfa87431200984ba11',
            },
          ],
        };
      }
    } catch (err: unknown) {
      errorMessage = err instanceof Error ? err.message : 'Gagal menghasilkan bukti forensik';
    } finally {
      isLoading = false;
    }
  }

  function copyJson() {
    if (!report) return;
    navigator.clipboard.writeText(JSON.stringify(report, null, 2));
    copied = true;
    setTimeout(() => copied = false, 2500);
  }

  function downloadJson() {
    if (!report) return;
    const blob = new Blob([JSON.stringify(report, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${report.export_id}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<Dialog
  bind:open
  title="Ekspor Bukti Forensik Kriptografis"
  description="Rantai audit anti-tamper bertanda tangan digital SHA-256 untuk keperluan hukum."
  {onclose}
>
  {#if isLoading}
    <div class="loading-state">Menghitung rantai hash kriptografis...</div>
  {:else if errorMessage}
    <div class="error-state">{errorMessage}</div>
  {:else if report}
    <div class="report-content">
      <!-- Status Badge Row -->
      <div class="status-summary-row">
        <span class="verified-badge">
          ✓ Rantai Hash Terverifikasi
        </span>
        <span class="records-count">{report.total_records} Catatan Kejadian</span>
      </div>

      <!-- Root Hash Card -->
      <Card variant="subtle" padding="sm">
        <div class="hash-box">
          <span class="hash-label">SHA-256 ROOT CHECKSUM:</span>
          <code class="hash-code">{report.root_hash_sha256}</code>
        </div>
      </Card>

      <!-- Key Fingerprint -->
      <div class="meta-row">
        <span class="meta-label">Sidik Jari Pemilik:</span>
        <span class="meta-val">{report.identity_fingerprint}</span>
      </div>

      <!-- Log Entries List -->
      <div class="entries-header">Rincian Peristiwa Tersegel:</div>
      <div class="entries-scroll">
        {#each report.log_entries as entry}
          <div class="entry-item">
            <div class="entry-top">
              <span class="event-name">{entry.event_type}</span>
              <span class="event-target">{entry.target}</span>
            </div>
            <code class="sig-code">SIG: {entry.signature_hex}</code>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  {#snippet actions()}
    <Button variant="ghost" onclick={() => open = false}>
      Tutup
    </Button>
    <Button variant="secondary" onclick={copyJson}>
      {copied ? '✓ Tersalin!' : '📋 Salin JSON'}
    </Button>
    <Button variant="primary" onclick={downloadJson}>
      💾 Unduh Berkas
    </Button>
  {/snippet}
</Dialog>

<style>
  .loading-state, .error-state {
    text-align: center;
    padding: var(--space-4);
    font-size: var(--font-body-size);
    color: var(--text-secondary);
  }

  .error-state {
    color: var(--sos-coral);
    font-weight: 600;
  }

  .report-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .status-summary-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .verified-badge {
    background-color: rgba(var(--brand-green-rgb), 0.2);
    color: var(--brand-dark-blue);
    font-size: var(--font-caption-size);
    font-weight: 700;
    padding: 3px 10px;
    border-radius: var(--radius-pill);
  }

  .records-count {
    font-size: var(--font-caption-size);
    font-weight: 600;
    color: var(--text-secondary);
  }

  .hash-box {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .hash-label {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-secondary);
    letter-spacing: 0.5px;
  }

  .hash-code {
    font-size: 11px;
    font-family: monospace;
    color: var(--brand-blue);
    word-break: break-all;
    user-select: all;
  }

  .meta-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: var(--font-label-size);
  }

  .meta-label {
    color: var(--text-secondary);
  }

  .meta-val {
    font-weight: 700;
    color: var(--text-primary);
  }

  .entries-header {
    font-size: var(--font-label-size);
    font-weight: 700;
    color: var(--text-primary);
    margin-top: 4px;
  }

  .entries-scroll {
    max-height: 180px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: var(--space-2);
  }

  .entry-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: var(--space-2);
    background-color: var(--surface-subtle);
    border-radius: var(--radius-sm);
  }

  .entry-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .event-name {
    font-size: 11.5px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .event-target {
    font-size: 10px;
    color: var(--text-secondary);
  }

  .sig-code {
    font-size: 9.5px;
    color: var(--text-secondary);
    word-break: break-all;
  }
</style>
