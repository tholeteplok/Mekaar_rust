<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';

  let selectedSegment = $state<'semua' | 'guardian' | 'arsip'>('semua');
  let isSearchActive = $state(false);
  let searchQuery = $state('');
  let showNewChatDialog = $state(false);
  let newContactTarget = $state('');

  const filteredConversations = $derived(
    mekaarStore.conversations.filter((c) => {
      if (mekaarStore.isDuressActive && c.isVaultHidden) return false;
      if (selectedSegment === 'guardian' && !c.isGuardian) return false;
      if (selectedSegment === 'arsip' && !c.isArchived) return false;
      if (selectedSegment === 'semua' && c.isArchived) return false;
      if (!searchQuery) return true;
      return (
        c.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.subtitle.toLowerCase().includes(searchQuery.toLowerCase())
      );
    })
  );

  function handleStartNewChat() {
    if (!newContactTarget.trim()) return;
    const clean = newContactTarget.trim();
    const newId = `conv-${Date.now()}`;
    mekaarStore.conversations = [
      {
        id: newId,
        title: clean,
        subtitle: '🔒 Sesi E2EE Baru',
        lastTimestamp: 'Baru',
        avatarColor: '#EFF6FF',
        unreadCount: 0,
        isPinned: false,
        isGroup: false,
        isOnline: true,
        isGuardian: false,
        disappearingSeconds: 0
      },
      ...mekaarStore.conversations
    ];
    newContactTarget = '';
    showNewChatDialog = false;
    mekaarStore.openConversation(newId);
  }
</script>

<AppBar
  title="Pesan"
  showAegisBadge={true}
>
  {#snippet actions()}
    <!-- Guardian Icon Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (selectedSegment = 'guardian')}
      title="Filter Guardian"
      aria-label="Guardian"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
        <circle cx="12" cy="11" r="3"/>
      </svg>
    </button>

    <!-- Search Icon Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (isSearchActive = !isSearchActive)}
      title="Cari Percakapan"
      aria-label="Cari"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
    </button>
  {/snippet}
</AppBar>

<div class="chat-list-container">
  {#if isSearchActive}
    <div class="search-bar-wrap">
      <input
        type="text"
        class="search-input"
        placeholder="Cari obrolan..."
        bind:value={searchQuery}
      />
    </div>
  {/if}

  <!-- Sliding Segment Bar [Semua | Guardian | Arsip] -->
  <div class="segment-pill-bar">
    <button
      type="button"
      class="segment-btn {selectedSegment === 'semua' ? 'active' : ''}"
      onclick={() => (selectedSegment = 'semua')}
    >
      Semua
    </button>
    <button
      type="button"
      class="segment-btn {selectedSegment === 'guardian' ? 'active' : ''}"
      onclick={() => (selectedSegment = 'guardian')}
    >
      Guardian
    </button>
    <button
      type="button"
      class="segment-btn {selectedSegment === 'arsip' ? 'active' : ''}"
      onclick={() => (selectedSegment = 'arsip')}
    >
      Arsip
    </button>
  </div>

  <!-- Conversations List -->
  <div class="conversations-feed">
    {#each filteredConversations as conv (conv.id)}
      <Card
        interactive
        withTicks={true}
        padding="16px"
        onclick={() => mekaarStore.openConversation(conv.id)}
      >
        <div class="chat-tile-content">
          <SketchAvatar
            name={conv.title}
            size={52}
            online={conv.isOnline}
            isGuardian={conv.isGuardian}
            isGroup={conv.isGroup}
          />
          <div class="chat-tile-text">
            <div class="chat-tile-row-top">
              <span class="chat-tile-name">{conv.title}</span>
              <span class="chat-tile-date">{conv.lastTimestamp}</span>
            </div>
            <div class="chat-tile-row-bottom">
              <span class="chat-tile-sub">
                {#if conv.subtitle.includes('🔒')}
                  🔒
                {:else if conv.isGroup}
                  👥
                {/if}
                {conv.subtitle.replace(/^🔒\s*/, '')}
              </span>
              {#if conv.unreadCount > 0}
                <span class="unread-badge">{conv.unreadCount}</span>
              {/if}
            </div>
          </div>
        </div>
      </Card>
    {:else}
      <div class="empty-feed">
        <div style="font-size: 2.2rem; margin-bottom: 8px;">💬</div>
        <p style="font-weight: 600; color: var(--text-primary);">Belum Ada Obrolan</p>
        <p style="font-size: 0.85rem; color: var(--text-secondary); margin-top: 4px;">
          Gunakan tombol '+' di kanan bawah untuk memulai obrolan baru terenkripsi.
        </p>
      </div>
    {/each}
  </div>
</div>

<!-- Modal Mulai Chat Baru -->
<Dialog
  open={showNewChatDialog}
  title="Mulai Chat Baru"
  onclose={() => (showNewChatDialog = false)}
>
  <div style="display: flex; flex-direction: column; gap: 14px;">
    <p style="font-size: 0.88rem; color: var(--text-secondary);">
      Masukkan username atau email teman Anda untuk obrolan 1-on-1.
    </p>
    <input
      type="text"
      class="sketch-input"
      placeholder="Username (@teplok) atau Email..."
      bind:value={newContactTarget}
      onkeydown={(e) => e.key === 'Enter' && handleStartNewChat()}
    />
    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
      <button type="button" class="btn-secondary" onclick={() => (showNewChatDialog = false)}>
        Batal
      </button>
      <button type="button" class="btn-primary" onclick={handleStartNewChat}>
        Mulai Chat
      </button>
    </div>
  </div>
</Dialog>

<style>
  .chat-list-container {
    padding: 0 16px 20px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .icon-action-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .icon-action-btn:active {
    background-color: var(--brand-blue-subtle);
  }

  .search-bar-wrap {
    margin-bottom: 2px;
  }

  .search-input {
    width: 100%;
    padding: 10px 16px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background-color: #FFFFFF;
    font-size: 0.9rem;
    color: var(--text-primary);
    box-sizing: border-box;
    outline: none;
  }

  .search-input:focus {
    border-color: var(--brand-blue);
  }

  /* Sliding Segment Bar */
  .segment-pill-bar {
    display: flex;
    background-color: #F1F5F9;
    padding: 4px;
    border-radius: var(--radius-pill);
    gap: 4px;
  }

  .segment-btn {
    flex: 1;
    padding: 9px 16px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 0.92rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.18s cubic-bezier(0.4, 0, 0.2, 1);
    text-align: center;
  }

  .segment-btn.active {
    background-color: var(--brand-blue);
    color: #FFFFFF;
    box-shadow: 0 2px 8px rgba(19, 108, 252, 0.28);
  }

  /* Feed of Conversations */
  .conversations-feed {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 4px;
  }

  .chat-tile-content {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .chat-tile-text {
    flex: 1;
    min-width: 0;
  }

  .chat-tile-row-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 4px;
  }

  .chat-tile-name {
    font-weight: 700;
    font-size: 1.05rem;
    color: var(--text-primary);
    letter-spacing: -0.1px;
  }

  .chat-tile-date {
    font-size: 0.82rem;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .chat-tile-row-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .chat-tile-sub {
    font-size: 0.88rem;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .unread-badge {
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.72rem;
    font-weight: 700;
    padding: 2px 7px;
    border-radius: 999px;
  }

  .empty-feed {
    padding: 48px 24px;
    text-align: center;
  }
</style>
