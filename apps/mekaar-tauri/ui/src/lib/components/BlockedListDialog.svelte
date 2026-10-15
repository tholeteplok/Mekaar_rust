<script lang="ts">
  import { mekaarStore } from '../stores';

  let {
    open = false,
    onclose
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();
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
      aria-label="Daftar Pengguna Diblokir"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="sheet-drag-handle"></div>

      <div class="sheet-header">
        <div style="display: flex; align-items: center; gap: 10px;">
          <div class="header-icon-circle">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#EA580C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
            </svg>
          </div>
          <div>
            <h3 class="sheet-title">Daftar Kontak Diblokir</h3>
            <p class="sheet-sub">Pengguna yang diblokir tidak dapat mengirim pesan atau melacak lokasi Anda</p>
          </div>
        </div>
        <button type="button" class="close-x-btn" onclick={onclose} aria-label="Tutup">✕</button>
      </div>

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
    max-width: 520px;
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
    background-color: #FFEDD5;
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
    background-color: #FFFFFF;
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
  }

  .user-name {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .user-handle {
    font-size: 0.76rem;
    color: var(--text-secondary);
    font-family: var(--font-mono);
  }

  .block-meta {
    font-size: 0.72rem;
    color: #EA580C;
    margin-top: 2px;
  }

  .btn-unblock {
    padding: 6px 14px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-primary);
    font-size: 0.78rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-unblock:hover {
    background-color: var(--bg-primary);
    border-color: var(--brand-blue);
    color: var(--brand-blue);
  }

  .empty-state {
    padding: 36px 20px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .sheet-info-notice {
    padding: 10px 14px;
    background-color: var(--bg-primary);
    border-radius: var(--radius-md);
    font-size: 0.78rem;
    color: var(--text-secondary);
    line-height: 1.45;
  }
</style>
