<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { MessageModel, ChatThemeId } from '../types';
  import AppBar from '../components/AppBar.svelte';
  import Button from '../components/Button.svelte';
  import ChatBubble from '../components/ChatBubble.svelte';
  import ChatInputBar from '../components/ChatInputBar.svelte';
  import BottomSheet from '../components/BottomSheet.svelte';
  import Dialog from '../components/Dialog.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let replyingTo = $state<MessageModel | null>(null);
  let showRoomInfo = $state(false);
  let showEmojiStore = $state(false);

  const activeConv = $derived(
    mekaarStore.conversations.find((c) => c.id === mekaarStore.selectedConversationId) ??
      mekaarStore.conversations[0]
  );

  const messages = $derived(
    activeConv ? mekaarStore.messagesByConv[activeConv.id] ?? [] : []
  );

  const themeOptions: { id: ChatThemeId; label: string }[] = [
    { id: 'classic', label: 'Biru Klasik' },
    { id: 'forest', label: 'Hijau Hutan' },
    { id: 'sunset', label: 'Senja Oranye' },
    { id: 'ocean', label: 'Samudra Sian' },
    { id: 'midnight', label: 'Malam Gelap' }
  ];

  const timerOptions = [
    { sec: 0, label: 'Nonaktif' },
    { sec: 300, label: '5 Menit' },
    { sec: 3600, label: '1 Jam' },
    { sec: 86400, label: '24 Jam' }
  ];
</script>

{#if activeConv}
  <div class="room-wrapper" data-register="room" data-chat-theme={activeConv.chatTheme}>
    <AppBar
      title={activeConv.title}
      subtitle={activeConv.isGroup
        ? `${activeConv.participants.length} Anggota • E2EE Grup`
        : activeConv.isOnline
          ? 'Online • Terenkripsi E2EE'
          : 'Terakhir dilihat baru saja'}
      showBack={true}
      onback={() => mekaarStore.closeConversation()}
    >
      {#snippet actions()}
        <Button size="sm" variant="secondary" onclick={() => mekaarStore.startCall(activeConv, false)}>
          📞 Suara
        </Button>
        <Button size="sm" variant="secondary" onclick={() => mekaarStore.startCall(activeConv, true)}>
          🎥 Video
        </Button>
        <Button size="sm" variant="secondary" onclick={() => (mekaarStore.forensicDialogOpen = true)}>
          🔐 Forensik
        </Button>
        <Button size="sm" variant="secondary" onclick={() => (showRoomInfo = true)}>
          ℹ️ Detail
        </Button>
      {/snippet}
    </AppBar>

    <div class="messages-viewport">
      <div class="e2ee-notice sketch-card">
        🔒 Pesan dan panggilan di ruang ini diamankan dengan enkripsi ujung-ke-ujung (X25519 + ChaCha20-Poly1305) melalui node <strong>{mekaarStore.serverStatus.wsEndpoint}</strong>.
      </div>

      {#each messages as msg (msg.id)}
        <ChatBubble
          message={msg}
          onreply={(m) => (replyingTo = m)}
          onreact={(msgId, emoji) => mekaarStore.toggleReaction(activeConv.id, msgId, emoji)}
        />
      {/each}
    </div>

    <ChatInputBar
      {replyingTo}
      emojiPacks={mekaarStore.emojiPacks}
      oncancelreply={() => (replyingTo = null)}
      onsend={(text, att) => {
        mekaarStore.sendMessage(activeConv.id, text, replyingTo, att);
        replyingTo = null;
      }}
      onopenemojistore={() => (showEmojiStore = true)}
    />
  </div>

  <!-- BottomSheet Detail Ruang / Grup, Tema Chat & Pesan Menghilang (group_details_screen.dart parity) -->
  <BottomSheet open={showRoomInfo} title="ℹ️ Pengaturan Ruang & Anggota" onclose={() => (showRoomInfo = false)}>
    <div style="display: flex; flex-direction: column; gap: 14px;">
      <div style="display: flex; align-items: center; gap: 12px;">
        <SketchAvatar name={activeConv.title} color={activeConv.avatarColor} size={54} isGroup={activeConv.isGroup} />
        <div>
          <h4 style="font-size: 1.05rem;">{activeConv.title}</h4>
          <p style="font-size: 0.82rem; color: var(--text-secondary);">
            {activeConv.isGroup ? 'Grup Terenkripsi' : 'Percakapan Pribadi E2EE'}
          </p>
        </div>
      </div>

      <div>
        <div style="font-size: 0.84rem; font-weight: 700; margin-bottom: 6px;">🎨 Tema Warna Ruang Obrolan:</div>
        <div style="display: flex; flex-wrap: wrap; gap: 6px;">
          {#each themeOptions as th}
            <Button
              size="sm"
              variant={activeConv.chatTheme === th.id ? 'primary' : 'secondary'}
              onclick={() => (activeConv.chatTheme = th.id)}
            >
              {th.label}
            </Button>
          {/each}
        </div>
      </div>

      <div>
        <div style="font-size: 0.84rem; font-weight: 700; margin-bottom: 6px;">⏱️ Timer Pesan Menghilang (Disappearing Messages):</div>
        <div style="display: flex; flex-wrap: wrap; gap: 6px;">
          {#each timerOptions as tm}
            <Button
              size="sm"
              variant={activeConv.disappearingSeconds === tm.sec ? 'primary' : 'secondary'}
              onclick={() => (activeConv.disappearingSeconds = tm.sec)}
            >
              {tm.label}
            </Button>
          {/each}
        </div>
      </div>

      <div>
        <div style="font-size: 0.84rem; font-weight: 700; margin-bottom: 6px;">
          👥 Daftar Partisipan ({activeConv.participants.length}):
        </div>
        <div style="display: flex; flex-direction: column; gap: 6px;">
          {#each activeConv.participants as part}
            <div class="sketch-card" style="padding: 8px 12px; display: flex; justify-content: space-between;">
              <span>{part.displayName}</span>
              <span class="sketch-badge">{part.role === 'admin' ? 'Admin' : 'Anggota'}</span>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </BottomSheet>

  <!-- Dialog Toko Emoji (emoji_store_screen.dart parity) -->
  <Dialog open={showEmojiStore} title="🛍️ Toko Paket Stiker & Emoji" onclose={() => (showEmojiStore = false)}>
    <div style="display: flex; flex-direction: column; gap: 10px;">
      {#each mekaarStore.emojiPacks as pack (pack.id)}
        <div class="sketch-card" style="padding: 12px;">
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <div>
              <strong>{pack.title}</strong>
              <span style="font-size: 0.76rem; color: var(--text-secondary); margin-left: 6px;">oleh {pack.author}</span>
            </div>
            <Button
              size="sm"
              variant={pack.installed ? 'secondary' : 'primary'}
              onclick={() => (pack.installed = !pack.installed)}
            >
              {pack.installed ? '✓ Terpasang' : '+ Pasang'}
            </Button>
          </div>
          <p style="font-size: 0.8rem; color: var(--text-secondary); margin: 4px 0 8px;">{pack.description}</p>
          <div style="font-size: 1.3rem; letter-spacing: 6px;">{pack.stickers.join(' ')}</div>
        </div>
      {/each}
    </div>
  </Dialog>
{/if}

<style>
  .room-wrapper {
    display: flex;
    flex-direction: column;
    height: calc(100vh - 68px);
    background: var(--register-bg);
  }
  .messages-viewport {
    flex: 1;
    overflow-y: auto;
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
  .e2ee-notice {
    padding: 8px 12px;
    margin-bottom: 14px;
    font-size: 0.78rem;
    text-align: center;
    background: var(--bg-elevated);
  }
</style>
