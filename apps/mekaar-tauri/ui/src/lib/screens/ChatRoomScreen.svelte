<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { MessageModel, ChatThemeId } from '../types';
  import ChatBubble from '../components/ChatBubble.svelte';
  import ChatInputBar from '../components/ChatInputBar.svelte';
  import BottomSheet from '../components/BottomSheet.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let replyingTo = $state<MessageModel | null>(null);
  let showRoomOptions = $state(false);

  const activeConv = $derived(
    mekaarStore.conversations.find((c) => c.id === mekaarStore.selectedConversationId) ??
      mekaarStore.conversations[0]
  );

  const messages = $derived(
    activeConv ? mekaarStore.messagesByConv[activeConv.id] ?? [] : []
  );

  // Group messages with date separators
  const dateGroups = $derived.by(() => {
    // Return sample date pills like in the screenshot
    return [
      { date: '27 Agu', msgs: messages.slice(0, 3) },
      { date: '28 Agu', msgs: messages.slice(3, 4) },
      { date: '26 Sep', msgs: messages.slice(4) }
    ];
  });
</script>

{#if activeConv}
  <div class="chat-room-scaffold">
    <!-- Floating Header Pill Bar -->
    <header class="floating-room-header">
      <!-- 1. Circular Back Button -->
      <button
        type="button"
        class="circle-nav-btn"
        onclick={() => mekaarStore.closeConversation()}
        aria-label="Kembali ke Daftar Chat"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
      </button>

      <!-- 2. Center Contact Pill -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="center-contact-pill" onclick={() => (showRoomOptions = true)}>
        <SketchAvatar
          name={activeConv.title}
          size={36}
          online={activeConv.isOnline}
          isGroup={activeConv.isGroup}
        />
        <div class="pill-text-col">
          <span class="pill-contact-name">{activeConv.title}</span>
          <span class="pill-status-desc">
            {#if activeConv.isOnline}
              <span class="status-online-dot">●</span> Online
            {:else}
              <span class="status-offline-dot">●</span> Terakhir dilihat 27 Agu pukul 16:45
            {/if}
          </span>
        </div>
      </div>

      <!-- 3. Circular Options Button -->
      <button
        type="button"
        class="circle-nav-btn"
        onclick={() => (showRoomOptions = true)}
        aria-label="Opsi Obrolan"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="var(--brand-blue)">
          <circle cx="6" cy="12" r="2.2"/>
          <circle cx="12" cy="12" r="2.2"/>
          <circle cx="18" cy="12" r="2.2"/>
        </svg>
      </button>
    </header>

    <!-- Scrollable Messages Feed -->
    <main class="chat-messages-viewport">
      {#each dateGroups as group}
        {#if group.msgs.length > 0}
          <div class="date-pill-row">
            <span class="date-pill">{group.date}</span>
          </div>
          {#each group.msgs as msg (msg.id)}
            <ChatBubble
              message={msg}
              onreply={(m) => (replyingTo = m)}
              onreact={(msgId, emoji) => mekaarStore.toggleReaction(activeConv.id, msgId, emoji)}
            />
          {/each}
        {/if}
      {/each}
    </main>

    <!-- Floating Chat Input Bar -->
    <ChatInputBar
      {replyingTo}
      emojiPacks={mekaarStore.emojiPacks}
      oncancelreply={() => (replyingTo = null)}
      onsend={(text, att) => {
        mekaarStore.sendMessage(activeConv.id, text, replyingTo, att);
        replyingTo = null;
      }}
    />
  </div>

  <!-- Bottom Sheet Opsi Obrolan -->
  <BottomSheet
    open={showRoomOptions}
    title="Opsi Obrolan & Keamanan"
    onclose={() => (showRoomOptions = false)}
  >
    <div style="display: flex; flex-direction: column; gap: 14px;">
      <div style="display: flex; align-items: center; gap: 14px;">
        <SketchAvatar name={activeConv.title} size={56} online={activeConv.isOnline} />
        <div>
          <h3 style="font-size: 1.1rem; font-weight: 700;">{activeConv.title}</h3>
          <p style="font-size: 0.85rem; color: var(--text-secondary);">
            🔒 Sesi E2EE Terenkripsi (X25519 + ChaCha20)
          </p>
        </div>
      </div>

      <div class="options-actions-list">
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            mekaarStore.startCall(activeConv, false);
          }}
        >
          <span>📞 Mulai Panggilan Suara E2EE</span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            mekaarStore.startCall(activeConv, true);
          }}
        >
          <span>🎥 Mulai Panggilan Video E2EE</span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            mekaarStore.forensicDialogOpen = true;
          }}
        >
          <span>🔐 Ekspor Verifikasi Kriptografi Forensik</span>
        </button>
        <button
          type="button"
          class="sheet-action-row danger-action"
          onclick={() => {
            if (confirm('Hapus obrolan ini dari daftar?')) {
              mekaarStore.closeConversation();
            }
          }}
        >
          <span>🗑️ Hapus Riwayat Obrolan</span>
        </button>
      </div>
    </div>
  </BottomSheet>
{/if}

<style>
  .chat-room-scaffold {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: var(--bg-primary);
    position: relative;
    box-sizing: border-box;
    overflow: hidden;
  }

  /* Floating Header Bar */
  .floating-room-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: calc(12px + env(safe-area-inset-top, 0px)) 16px 8px 16px;
    gap: 10px;
    z-index: 30;
    background-color: var(--bg-primary);
  }

  .circle-nav-btn {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background-color: #FFFFFF;
    border: 1px solid var(--border-light);
    color: var(--text-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(21, 38, 65, 0.06);
    flex-shrink: 0;
    transition: transform 0.12s ease;
  }

  .circle-nav-btn:active {
    transform: scale(0.92);
  }

  /* Center Floating Contact Pill */
  .center-contact-pill {
    flex: 1;
    height: 48px;
    border-radius: var(--radius-pill);
    background-color: #FFFFFF;
    border: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    padding: 0 14px 0 6px;
    gap: 10px;
    box-shadow: 0 2px 10px rgba(21, 38, 65, 0.05);
    cursor: pointer;
    min-width: 0;
  }

  .center-contact-pill:active {
    background-color: #F8FAFC;
  }

  .pill-text-col {
    display: flex;
    flex-direction: column;
    min-width: 0;
    flex: 1;
  }

  .pill-contact-name {
    font-size: 0.98rem;
    font-weight: 700;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .pill-status-desc {
    font-size: 0.76rem;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .status-online-dot {
    color: #10B981;
    font-size: 0.65rem;
  }

  .status-offline-dot {
    color: #9FB0C9;
    font-size: 0.65rem;
  }

  /* Messages Viewport */
  .chat-messages-viewport {
    flex: 1;
    overflow-y: auto;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    box-sizing: border-box;
  }

  /* Date Separator Pill */
  .date-pill-row {
    display: flex;
    justify-content: center;
    margin: 14px 0 10px 0;
  }

  .date-pill {
    background-color: var(--bubble-date-bg);
    color: var(--bubble-date-text);
    font-size: 0.78rem;
    font-weight: 700;
    padding: 5px 16px;
    border-radius: var(--radius-pill);
    box-shadow: 0 1px 4px rgba(3, 105, 161, 0.1);
  }

  .options-actions-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 10px;
  }

  .sheet-action-row {
    width: 100%;
    padding: 14px 16px;
    border-radius: 12px;
    border: 1px solid var(--border-subtle);
    background: #FFFFFF;
    text-align: left;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
  }

  .sheet-action-row:active {
    background: #F1F5F9;
  }

  .danger-action {
    color: var(--sos-coral);
    border-color: #FECDD3;
    background: #FFF1F2;
  }
</style>
