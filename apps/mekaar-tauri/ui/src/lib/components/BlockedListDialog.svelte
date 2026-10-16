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
</script>

<BottomSheet
  {open}
  title="Daftar Kontak Diblokir"
  subtitle="Pengguna yang diblokir tidak dapat mengirim pesan atau melacak lokasi Anda"
  maxWidth="min(560px, 100%)"
  {onclose}
>
  <div class="sheet-body">
    {#if mekaarStore.blockedUsers.length > 0}
      <div class="blocked-list">
        {#each mekaarStore.blockedUsers as user (user.id)}
          <div class="blocked-card">
            <div class="user-avatar-circle">🚫</div>
            <div class="user-info">
              <div class="user-name">{user.displayName}</div>
              <div class="user-handle">{user.username}</div>
              <div class="block-meta">
                Diblokir: {user.blockedAt} • {user.reason}
              </div>
            </div>
            <button
              type="button"
              class="btn-unblock"
              onclick={() => mekaarStore.unblockUser(user.id)}
            >
              Buka Blokir
            </button>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div style="font-size: 2.2rem; margin-bottom: 6px;">🕊️</div>
        <div style="font-weight: 700; color: var(--text-primary);">Daftar Blokir Bersih</div>
        <div style="font-size: 0.78rem; color: var(--text-secondary);">Tidak ada kontak yang saat ini Anda blokir.</div>
      </div>
    {/if}

    <div class="sheet-info-notice">
      ℹ️ Kontak yang dibuka blokirnya akan dapat kembali mengirimkan permintaan obrolan baru kepada Anda.
    </div>
  </div>
</BottomSheet>

<style>
  .sheet-body {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-top: 4px;
  }

  .blocked-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .blocked-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-light);
    background-color: var(--bg-surface);
  }

  .user-avatar-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: #F1F5F9;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.15rem;
    flex-shrink: 0;
  }

  .user-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 0.92rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .user-handle {
    font-size: 0.78rem;
    color: var(--text-secondary);
    font-family: var(--font-mono);
  }

  .block-meta {
    font-size: 0.72rem;
    color: #EA580C;
    margin-top: 2px;
  }

  .btn-unblock {
    min-height: var(--touch-target-min, 40px);
    padding: 6px 14px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-primary);
    font-size: 0.82rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .btn-unblock:hover {
    background-color: var(--bg-primary);
    border-color: var(--brand-blue);
    color: var(--brand-blue);
  }

  .empty-state {
    padding: 32px 20px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .sheet-info-notice {
    padding: 10px 14px;
    background-color: var(--bg-secondary);
    border-radius: var(--radius-md);
    font-size: 0.8rem;
    color: var(--text-secondary);
    line-height: 1.45;
  }
</style>
