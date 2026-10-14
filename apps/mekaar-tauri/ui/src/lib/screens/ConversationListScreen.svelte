<script lang="ts">
  import { onMount } from 'svelte';
  import { chatStore, vaultStore } from '../stores';
  import { AppBar, Card, Button, ForensicExportDialog } from '../components';

  interface Props {
    onselectroom: (roomId: string) => void;
    onopensos: () => void;
    onlock: () => void;
  }

  let { onselectroom, onopensos, onlock }: Props = $props();

  let searchQuery = $state('');
  let showForensicDialog = $state(false);

  onMount(() => {
    chatStore.loadConversations();
  });

  const filteredConversations = $derived(
    chatStore.conversations.filter((c) =>
      c.peerName.toLowerCase().includes(searchQuery.toLowerCase()) ||
      c.lastMessage.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  function formatTime(ts: number) {
    const d = new Date(ts);
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
</script>

<div class="conv-list-screen">
  <AppBar
    title="MEKAAR"
    subtitle={`Kunci: ${vaultStore.identityFingerprint}`}
  >
    {#snippet actions()}
      <Button
        variant="secondary"
        size="sm"
        onclick={() => showForensicDialog = true}
      >
        📋 Audit
      </Button>
      <Button
        variant="danger"
        size="sm"
        onclick={onopensos}
      >
        🚨 SOS
      </Button>
      <Button
        variant="ghost"
        size="sm"
        onclick={onlock}
      >
        🔒 Kunci
      </Button>
    {/snippet}
  </AppBar>

  <div class="conv-content">
    <!-- Decoy Duress Banner if in duress mode -->
    {#if vaultStore.isDuress}
      <div class="duress-alert-box">
        🛡️ <strong>MODUS BRANKAS UMPAN AKTIF:</strong> Riwayat chat rahasia asli disembunyikan.
      </div>
    {/if}

    <!-- Search input -->
    <div class="search-wrapper">
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
        <input
          type="text"
          class="search-input"
          placeholder="Cari obrolan rahasia atau kontak..."
          bind:value={searchQuery}
        />
        {#if searchQuery}
          <button type="button" class="clear-btn" onclick={() => searchQuery = ''}>×</button>
        {/if}
      </div>
    </div>

    <!-- Conversation Cards List -->
    <div class="conv-items">
      {#if chatStore.isLoading}
        <div class="empty-state">Memuat obrolan terenkripsi...</div>
      {:else if filteredConversations.length === 0}
        <div class="empty-state">Tidak ada percakapan ditemukan.</div>
      {:else}
        {#each filteredConversations as conv (conv.id)}
          <Card
            variant="default"
            padding="md"
            interactive
            onclick={() => onselectroom(conv.id)}
          >
            <div class="conv-card-content">
              <!-- Avatar circle -->
              <div class="avatar-circle">
                {conv.peerName.charAt(0).toUpperCase()}
              </div>

              <!-- Chat Info -->
              <div class="conv-main-info">
                <div class="conv-header-row">
                  <span class="peer-name">{conv.peerName}</span>
                  <span class="conv-time">{formatTime(conv.lastTimestamp)}</span>
                </div>

                <div class="conv-message-row">
                  <p class="last-message">{conv.lastMessage}</p>
                  {#if conv.unreadCount > 0}
                    <span class="unread-badge">{conv.unreadCount}</span>
                  {/if}
                </div>

                <div class="conv-footer-row">
                  <span class="ratchet-tag">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                      <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
                      <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
                    </svg>
                    Double Ratchet E2EE
                  </span>
                </div>
              </div>
            </div>
          </Card>
        {/each}
      {/if}
    </div>
  </div>

  <ForensicExportDialog bind:open={showForensicDialog} />
</div>

<style>
  .conv-list-screen {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .conv-content {
    max-width: 800px;
    width: 100%;
    margin: 0 auto;
    padding: var(--space-4) var(--space-4) var(--space-12);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .duress-alert-box {
    padding: var(--space-3) var(--space-4);
    background-color: rgba(251, 191, 36, 0.15);
    border: 1px solid var(--warn-amber);
    border-radius: var(--radius-md);
    color: #B45309;
    font-size: var(--font-label-size);
  }

  .search-wrapper {
    position: relative;
    width: 100%;
  }

  .search-box {
    display: flex;
    align-items: center;
    background-color: var(--surface-card);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-pill);
    padding: 0 var(--space-4);
    height: 48px;
    gap: var(--space-2);
    box-shadow: var(--shadow-soft);
    transition: border-color var(--motion-fast);
  }

  .search-box:focus-within {
    border-color: var(--border-focus);
  }

  .search-icon {
    width: 20px;
    height: 20px;
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    font-family: var(--font-family);
    font-size: var(--font-body-size);
    color: var(--text-primary);
    outline: none;
  }

  .clear-btn {
    border: none;
    background: transparent;
    font-size: 18px;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .conv-items {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .empty-state {
    text-align: center;
    padding: var(--space-8);
    color: var(--text-secondary);
    font-size: var(--font-body-size);
  }

  .conv-card-content {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .avatar-circle {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-light-blue);
    color: var(--brand-blue);
    font-size: var(--font-h2-size);
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .conv-main-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .conv-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
  }

  .peer-name {
    font-size: var(--font-body-size);
    font-weight: 700;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .conv-time {
    font-size: var(--font-caption-size);
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .conv-message-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
  }

  .last-message {
    font-size: var(--font-label-size);
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .unread-badge {
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: var(--font-caption-size);
    font-weight: 700;
    padding: 2px 7px;
    border-radius: var(--radius-pill);
    line-height: 1;
  }

  .conv-footer-row {
    display: flex;
    align-items: center;
    margin-top: 2px;
  }

  .ratchet-tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: var(--font-caption-size);
    font-weight: 600;
    color: var(--safe-teal);
  }

  .ratchet-tag svg {
    width: 12px;
    height: 12px;
  }
</style>
