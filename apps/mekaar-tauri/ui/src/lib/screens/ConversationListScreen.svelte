<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';
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
  <div class="conversations-feed">
    {#each filteredConversations as conv (conv.id)}
      <Card
        interactive
        withTicks={false}
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
              <span class="chat-tile-sub" style="display: flex; align-items: center; gap: 4px;">
                {#if conv.subtitle.includes('🔒')}
                  <LockKey size={13} weight="bold" color="var(--brand-teal, #0D9488)" />
                {:else if conv.isGroup}
                  <Users size={13} weight="bold" color="var(--text-secondary, #64748B)" />
                {/if}
                <span>{conv.subtitle.replace(/^🔒\s*/, '')}</span>
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
        <div style="margin-bottom: 8px; display: flex; justify-content: center;">
          <ChatCircleDots size={48} weight="duotone" color="var(--brand-blue, #0284C7)" />
        </div>
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
