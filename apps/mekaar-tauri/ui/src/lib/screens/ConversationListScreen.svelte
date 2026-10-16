<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import {
    Compass,
    UsersThree,
    MagnifyingGlass,
    NavigationArrow,
    LockKey,
    Users,
    ChatCircleDots
  } from 'phosphor-svelte';

  let selectedSegment = $state<'semua' | 'guardian' | 'arsip'>('semua');
  let isSearchActive = $state(false);
  let searchQuery = $state('');
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
        isMuted: false,
        isTyping: false,
        isGuardian: false,
        disappearingSeconds: 0,
        chatTheme: 'classic',
        participants: [
          { userId: mekaarStore.currentUser.id, displayName: mekaarStore.currentUser.displayName, role: 'admin', muted: false },
          { userId: `usr-${clean.toLowerCase().replace('@', '')}`, displayName: clean, role: 'member', muted: false }
        ]
      },
      ...mekaarStore.conversations
    ];
    newContactTarget = '';
    mekaarStore.showNewChatModal = false;
    mekaarStore.openConversation(newId);
  }
</script>

<AppBar
  title="Pesan"
  showAegisBadge={true}
>
  {#snippet actions()}
    <!-- Map / Radar Icon Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => mekaarStore.setTab('map')}
      title="Peta & Radar Teman Sekitar"
      aria-label="Peta Radar"
    >
      <Compass size={22} weight="bold" color="var(--brand-blue, #0284C7)" />
    </button>

    <!-- Guardian Hub Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => mekaarStore.setTab('guardian')}
      title="Guardian & Kawal Perjalanan"
      aria-label="Guardian Hub"
    >
      <UsersThree size={22} weight="bold" color="var(--brand-blue, #0284C7)" />
    </button>

    <!-- Search Icon Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (isSearchActive = !isSearchActive)}
      title="Cari Percakapan"
      aria-label="Cari"
    >
      <MagnifyingGlass size={22} weight="bold" color="var(--brand-blue, #0284C7)" />
    </button>
  {/snippet}
</AppBar>

<div class="chat-list-container">
  <!-- Ongoing Trip Banner -->
  {#if mekaarStore.trips.some((t) => t.status === 'ongoing')}
    {@const ongoing = mekaarStore.trips.find((t) => t.status === 'ongoing')}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="ongoing-trip-banner"
      onclick={() => mekaarStore.setTab('guardian')}
    >
      <div style="display: flex; align-items: center; gap: 8px;">
        <span class="trip-pulse-dot"></span>
        <div>
          <div style="font-size: 0.85rem; font-weight: 700; color: #065F46; display: flex; align-items: center; gap: 6px;">
            <NavigationArrow size={16} weight="fill" color="#059669" />
            <span>Kawal Aktif: Menuju {ongoing?.destinationName}</span>
          </div>
          <div style="font-size: 0.76rem; color: #047857;">ETA {ongoing?.etaMinutes} mnt • Dikawal {ongoing?.activeGuardians.join(', ')}</div>
        </div>
      </div>
      <span class="sketch-badge" style="background: #10B981; color: #FFFFFF; font-size: 0.72rem;">Pantau ➔</span>
    </div>
  {/if}

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
  {#if filteredConversations.length > 0}
    <div class="conversations-flat-card">
      {#each filteredConversations as conv (conv.id)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <div
          role="button"
          tabindex="0"
          class="chat-tile-row"
          onclick={() => mekaarStore.openConversation(conv.id)}
          onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && mekaarStore.openConversation(conv.id)}
        >
          <div class="chat-tile-avatar-wrap">
            <SketchAvatar
              name={conv.title}
              size={50}
              online={conv.isOnline}
              isGuardian={conv.isGuardian}
              isGroup={conv.isGroup}
            />
          </div>
          <div class="chat-tile-body">
            <div class="chat-tile-row-top">
              <span class="chat-tile-name">{conv.title}</span>
              <span class="chat-tile-date {conv.unreadCount > 0 ? 'unread' : ''}">{conv.lastTimestamp}</span>
            </div>
            <div class="chat-tile-row-bottom">
              <span class="chat-tile-sub">
                {#if conv.subtitle.includes('🔒')}
                  <LockKey size={14} weight="bold" color="var(--brand-teal, #0D9488)" />
                {:else if conv.isGroup}
                  <Users size={14} weight="bold" color="var(--text-secondary, #64748B)" />
                {/if}
                <span class="chat-tile-sub-text">{conv.subtitle.replace(/^🔒\s*/, '')}</span>
              </span>
              {#if conv.unreadCount > 0}
                <span class="unread-badge">{conv.unreadCount}</span>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="conversations-empty-wrap">
      {#if searchQuery.trim()}
        <EmptyState
          pose="confused"
          title="Chat tidak ditemukan"
          message="Tidak ada chat yang cocok dengan '{searchQuery.trim()}'."
        />
      {:else if selectedSegment === 'guardian'}
        <EmptyState
          pose="shield"
          title="Belum ada chat Guardian"
          message="Chat dengan kontak Guardian Anda akan muncul di filter ini."
        />
      {:else if selectedSegment === 'arsip'}
        <EmptyState
          pose="huft"
          title="Tidak ada chat diarsipkan"
          message="Chat yang diarsipkan akan muncul di sini."
        />
      {:else}
        <EmptyState
          pose="huft"
          title="Belum ada obrolan"
          message="Mulai percakapan pertamamu dengan teman atau kontak terpercaya."
          actionLabel="Mulai obrolan"
          actionIcon={ChatCircleDots}
          onaction={() => (mekaarStore.showNewChatModal = true)}
        />
      {/if}
    </div>
  {/if}
</div>

<!-- Modal Mulai Chat Baru -->
<Dialog
  open={mekaarStore.showNewChatModal}
  title="Mulai Chat Baru"
  onclose={() => (mekaarStore.showNewChatModal = false)}
>
  <div style="display: flex; flex-direction: column; gap: 14px;">
    <p style="font-size: 0.88rem; color: var(--text-secondary);">
      Masukkan username teman Anda (@username) untuk memulai obrolan terenkripsi.
    </p>
    <input
      type="text"
      class="sketch-input"
      placeholder="Username (@teman)..."
      bind:value={newContactTarget}
      onkeydown={(e) => e.key === 'Enter' && handleStartNewChat()}
    />
    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
      <button type="button" class="btn-secondary" onclick={() => (mekaarStore.showNewChatModal = false)}>
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
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: 0 16px 24px 16px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .ongoing-trip-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    background: #ECFDF5;
    border: 1.5px solid #10B981;
    border-radius: var(--radius-lg);
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(16, 185, 129, 0.12);
  }

  .trip-pulse-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #10B981;
    box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.3);
  }

  .icon-action-btn {
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    border-radius: 50%;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.15s ease, transform 0.1s ease;
  }

  .icon-action-btn:active {
    transform: scale(0.92);
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

  /* Flat Unified Card Container (Signal & Telegram Parity) */
  .conversations-flat-card {
    display: flex;
    flex-direction: column;
    background: var(--bg-surface);
    border-radius: var(--radius-card);
    border: 0.75px solid var(--border-subtle);
    box-shadow: var(--shadow-card);
    overflow: hidden;
    margin-top: 4px;
  }

  .chat-tile-row {
    display: flex;
    align-items: center;
    padding: 0 16px;
    cursor: pointer;
    transition: background-color 0.15s ease;
    background-color: transparent;
    user-select: none;
    outline: none;
  }

  .chat-tile-row:hover {
    background-color: rgba(19, 108, 252, 0.04);
  }

  .chat-tile-row:active {
    background-color: rgba(19, 108, 252, 0.08);
  }

  .chat-tile-avatar-wrap {
    padding: 12px 0;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .chat-tile-body {
    flex: 1;
    min-width: 0;
    padding: 14px 0 14px 14px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    border-bottom: 0.75px solid var(--border-light);
  }

  /* Inset divider: baris terakhir tidak memiliki garis pemisah */
  .chat-tile-row:last-child .chat-tile-body {
    border-bottom: none;
  }

  .chat-tile-row-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin-bottom: 3px;
  }

  .chat-tile-name {
    font-family: var(--font-active);
    font-weight: 700;
    font-size: 1.02rem;
    color: var(--text-primary);
    letter-spacing: -0.15px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .chat-tile-date {
    font-size: 0.8rem;
    color: var(--text-muted);
    font-weight: 500;
    flex-shrink: 0;
  }

  .chat-tile-date.unread {
    color: var(--brand-blue);
    font-weight: 700;
  }

  .chat-tile-row-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .chat-tile-sub {
    font-size: 0.88rem;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 5px;
    min-width: 0;
    flex: 1;
  }

  .chat-tile-sub-text {
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
    border-radius: var(--radius-pill);
    min-width: 18px;
    text-align: center;
    box-shadow: 0 1px 4px rgba(19, 108, 252, 0.3);
    flex-shrink: 0;
  }

  .conversations-empty-wrap {
    margin-top: 20px;
    display: flex;
    justify-content: center;
  }
</style>
