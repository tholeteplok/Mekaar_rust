<script lang="ts">
  import { mekaarStore } from '../stores';
  import BottomSheet from './BottomSheet.svelte';

  let { open = false, onclose }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  let toastMessage = $state('');
  let toastType = $state<'success' | 'info' | 'warning'>('success');
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  const pendingRequests = $derived(
    mekaarStore.chatRequests.filter((r) => r.status === 'pending')
  );

  function showToast(msg: string, type: 'success' | 'info' | 'warning') {
    toastMessage = msg;
    toastType = type;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toastMessage = '';
    }, 3000);
  }

  function handleAccept(reqId: string) {
    mekaarStore.respondChatRequest(reqId, true);
    showToast('Undangan chat disetujui! Ruang obrolan telah dibuka.', 'success');
  }

  function handleReject(reqId: string) {
    mekaarStore.respondChatRequest(reqId, false);
    showToast('Undangan chat ditolak.', 'info');
  }

  function handleRejectAndBlock(reqId: string) {
    mekaarStore.rejectAndBlockChatRequest(reqId);
    showToast('Undangan ditolak dan pengundang telah diblokir.', 'warning');
  }
</script>

<BottomSheet
  {open}
  title="Permintaan Chat Masuk"
  subtitle={pendingRequests.length > 0 ? `${pendingRequests.length} undangan menunggu respon` : 'Kotak masuk undangan aman'}
  {onclose}
  maxWidth="500px"
>
  <div class="req-dialog-content">
    <!-- Feedback Toast Banner -->
    {#if toastMessage}
      <div class="toast-banner {toastType}">
        <span>{toastMessage}</span>
      </div>
    {/if}

    <!-- Requests List / Empty State -->
    <div class="req-body-scroll">
      {#if pendingRequests.length === 0}
        <div class="empty-requests-view">
          <div class="empty-icon-circle">😴</div>
          <h4>Tidak Ada Permintaan</h4>
          <p>Semua pesan masuk dari pengirim terverifikasi.</p>
        </div>
      {:else}
        <div class="requests-card-list">
          {#each pendingRequests as req (req.id)}
            <div class="request-card">
              <!-- Sender Header Row -->
              <div class="sender-header-row">
                <div
                  class="sender-avatar"
                  style="background-color: {req.avatarColor || '#DDD6FE'};"
                >
                  {req.fromDisplayName.charAt(0).toUpperCase()}
                </div>
                <div class="sender-meta">
                  <div class="sender-name-row">
                    <span class="sender-name">{req.fromDisplayName}</span>
                    <span class="sender-time">{req.timestamp}</span>
                  </div>
                  <span class="sender-sub">Ingin terhubung via Username <b>{req.fromUsername}</b></span>
                </div>
              </div>

              <!-- Invitation Note Card (Paritas KETERANGAN UNDANGAN Flutter) -->
              <div class="invitation-note-box">
                <span class="note-label">KETERANGAN UNDANGAN:</span>
                <p class="note-content">"{req.introMessage}"</p>
              </div>

              <!-- Fingerprint Tag -->
              <div class="fingerprint-tag-row">
                <span class="fp-badge">🛡️ {req.fingerprint}</span>
              </div>

              <!-- 3 Action Buttons (Paritas Flutter) -->
              <div class="action-buttons-group">
                <!-- 1. Tombol TERIMA (Teal/Hijau Accent) -->
                <button
                  type="button"
                  class="btn-accept"
                  onclick={() => handleAccept(req.id)}
                >
                  <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="20 6 9 17 4 12"></polyline>
                  </svg>
                  Terima Undangan
                </button>

                <div class="secondary-actions-row">
                  <!-- 2. Tombol TOLAK (Abu-abu Outlined) -->
                  <button
                    type="button"
                    class="btn-reject"
                    onclick={() => handleReject(req.id)}
                  >
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <line x1="18" y1="6" x2="6" y2="18"></line>
                      <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                    Tolak
                  </button>

                  <!-- 3. Tombol TOLAK & BLOKIR (Merah Outlined) -->
                  <button
                    type="button"
                    class="btn-block"
                    onclick={() => handleRejectAndBlock(req.id)}
                  >
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="10"></circle>
                      <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
                    </svg>
                    Tolak & Blokir
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</BottomSheet>

<style>
  .req-dialog-content {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .toast-banner {
    padding: 8px 12px;
    border-radius: var(--radius-md);
    font-size: 0.8rem;
    font-weight: 600;
    font-family: var(--font-active);
  }

  .toast-banner.success {
    background-color: var(--brand-teal-light, rgba(16, 185, 129, 0.12));
    color: var(--brand-teal, #0D9488);
    border: 1px solid rgba(13, 148, 136, 0.25);
  }

  .toast-banner.info {
    background-color: var(--brand-blue-subtle, rgba(2, 132, 199, 0.12));
    color: var(--brand-blue, #0284C7);
    border: 1px solid rgba(2, 132, 199, 0.25);
  }

  .toast-banner.warning {
    background-color: rgba(245, 158, 11, 0.12);
    color: #D97706;
    border: 1px solid rgba(245, 158, 11, 0.25);
  }

  .req-body-scroll {
    overflow-y: auto;
    max-height: 60vh;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  /* Empty State */
  .empty-requests-view {
    padding: 44px 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .empty-icon-circle {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    background-color: var(--bg-primary);
    font-size: 1.8rem;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
  }

  .empty-requests-view h4 {
    font-size: 1rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary);
    margin: 0 0 4px 0;
  }

  .empty-requests-view p {
    font-size: 0.82rem;
    color: var(--text-secondary);
    margin: 0;
  }

  /* Requests List */
  .requests-card-list {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .request-card {
    background-color: var(--bg-card);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-lg);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.03);
  }

  .sender-header-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .sender-avatar {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.15rem;
    font-weight: 800;
    color: #1E293B;
    flex-shrink: 0;
  }

  .sender-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .sender-name-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .sender-name {
    font-size: 0.95rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
  }

  .sender-time {
    font-size: 0.74rem;
    color: var(--text-secondary);
  }

  .sender-sub {
    font-size: 0.78rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .invitation-note-box {
    background-color: var(--bg-primary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-md);
    padding: 10px 12px;
  }

  .note-label {
    font-size: 0.7rem;
    font-weight: 800;
    color: var(--brand-blue);
    letter-spacing: 0.8px;
    display: block;
    margin-bottom: 3px;
  }

  .note-content {
    font-size: 0.84rem;
    color: var(--text-primary);
    font-style: italic;
    line-height: 1.4;
    margin: 0;
  }

  .fingerprint-tag-row {
    display: flex;
    align-items: center;
  }

  .fp-badge {
    background-color: var(--brand-blue-subtle);
    color: var(--brand-blue);
    font-size: 0.72rem;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: var(--radius-sm);
  }

  /* Actions */
  .action-buttons-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 4px;
  }

  .btn-accept {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    border-radius: var(--radius-pill);
    background-color: var(--brand-teal, #0D9488);
    color: #FFFFFF;
    font-size: 0.86rem;
    font-weight: 700;
    font-family: var(--font-active);
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    box-shadow: 0 2px 8px rgba(13, 148, 136, 0.25);
    transition: all 0.15s ease;
  }

  .btn-accept:hover {
    filter: brightness(1.08);
  }

  .btn-accept:active {
    transform: scale(0.98);
  }

  .secondary-actions-row {
    display: flex;
    gap: 8px;
  }

  .btn-reject {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-secondary);
    font-size: 0.82rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    transition: all 0.15s ease;
  }

  .btn-reject:hover {
    background-color: var(--bg-primary);
    color: var(--text-primary);
  }

  .btn-block {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid rgba(239, 68, 68, 0.4);
    color: #DC2626;
    font-size: 0.82rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    transition: all 0.15s ease;
  }

  .btn-block:hover {
    background-color: rgba(239, 68, 68, 0.08);
    border-color: #DC2626;
  }
</style>
