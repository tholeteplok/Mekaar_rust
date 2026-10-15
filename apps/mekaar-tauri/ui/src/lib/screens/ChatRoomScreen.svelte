<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { MessageModel, ChatThemeId } from '../types';
  import ChatBubble from '../components/ChatBubble.svelte';
  import ChatInputBar from '../components/ChatInputBar.svelte';
  import BottomSheet from '../components/BottomSheet.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import StegoDecoderDialog from '../components/StegoDecoderDialog.svelte';
  import SecureMediaVaultDialog from '../components/SecureMediaVaultDialog.svelte';
  import MediaLightboxModal from '../components/MediaLightboxModal.svelte';
  import Dialog from '../components/Dialog.svelte';
  import ContactDetailSheet from '../components/ContactDetailSheet.svelte';
  import ChatRoomPrivacySheet from '../components/ChatRoomPrivacySheet.svelte';
  import {
    CaretLeft,
    MagnifyingGlass,
    Timer,
    Phone,
    VideoCamera,
    ShieldCheck,
    LockKey,
    DotsThreeVertical,
    CaretUp,
    CaretDown,
    X,
    CornersOut,
    PhoneDisconnect,
    PushPin,
    UserCheck,
    FolderSimpleLock,
    Key,
    Trash,
    Prohibit,
    Check
  } from 'phosphor-svelte';

  let replyingTo = $state<MessageModel | null>(null);
  let editingMessage = $state<MessageModel | null>(null);
  let showRoomOptions = $state(false);
  let showContactDetail = $state(false);
  let showPrivacySheet = $state(false);
  let showStegoDecoder = $state(false);
  let selectedStegoSrc = $state('');
  let selectedStegoName = $state('');
  let showMediaVault = $state(false);

  // Revoke Dialog State
  let revokeTarget = $state<MessageModel | null>(null);
  let showRevokeDialog = $state(false);

  function handleLeaveRoom() {
    if (!activeConv) return;
    if (activeConv.isBurnOnExit) {
      mekaarStore.executeBurnOnExit(activeConv.id);
    }
    mekaarStore.closeConversation();
  }

  // Disappearing Timer Modal State
  let showDisappearingModal = $state(false);

  // In-Chat Search State
  let isSearchOpen = $state(false);
  let searchQuery = $state('');
  let currentMatchIndex = $state(0);
  let targetJumpId = $state('');

  // Lightbox Modal state
  let lightboxState = $state({
    open: false,
    src: '',
    name: '',
    sizeLabel: '',
    isStego: false
  });

  // Scroll to bottom FAB
  let viewportEl: HTMLElement | null = $state(null);
  let showScrollBottomFab = $state(false);

  function handleViewportScroll(e: Event) {
    const el = e.currentTarget as HTMLElement;
    const distanceFromBottom = el.scrollHeight - el.scrollTop - el.clientHeight;
    showScrollBottomFab = distanceFromBottom > 160;
  }

  function scrollToBottom() {
    if (viewportEl) {
      viewportEl.scrollTo({ top: viewportEl.scrollHeight, behavior: 'smooth' });
    }
  }

  function jumpToMessage(msgId: string) {
    targetJumpId = msgId;
    const el = document.getElementById(`bubble-${msgId}`);
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
    setTimeout(() => {
      if (targetJumpId === msgId) targetJumpId = '';
    }, 2200);
  }

  const activeConv = $derived(
    mekaarStore.conversations.find((c) => c.id === mekaarStore.selectedConversationId) ??
      mekaarStore.conversations[0]
  );

  const messages = $derived(
    activeConv ? mekaarStore.messagesByConv[activeConv.id] ?? [] : []
  );

  // Pinned Message lookup
  const pinnedMessage = $derived(
    activeConv?.pinnedMessageId ? messages.find((m) => m.id === activeConv.pinnedMessageId) : null
  );

  // Search matches
  const matchedMessageIds = $derived.by(() => {
    if (!searchQuery.trim()) return [];
    const q = searchQuery.trim().toLowerCase();
    return messages.filter((m) => !m.isRevoked && m.text.toLowerCase().includes(q)).map((m) => m.id);
  });

  function jumpToNextSearchMatch() {
    if (matchedMessageIds.length === 0) return;
    currentMatchIndex = (currentMatchIndex + 1) % matchedMessageIds.length;
    jumpToMessage(matchedMessageIds[currentMatchIndex]);
  }

  function jumpToPrevSearchMatch() {
    if (matchedMessageIds.length === 0) return;
    currentMatchIndex = (currentMatchIndex - 1 + matchedMessageIds.length) % matchedMessageIds.length;
    jumpToMessage(matchedMessageIds[currentMatchIndex]);
  }

  function formatDateLabel(rawTimestamp: string): string {
    if (!rawTimestamp) return 'Hari Ini';
    const d = new Date(rawTimestamp);
    if (!isNaN(d.getTime())) {
      const now = new Date();
      const isToday = d.toDateString() === now.toDateString();
      if (isToday) return 'Hari Ini';
      const yesterday = new Date();
      yesterday.setDate(now.getDate() - 1);
      if (d.toDateString() === yesterday.toDateString()) return 'Kemarin';

      const months = ['Jan', 'Feb', 'Mar', 'Apr', 'Mei', 'Jun', 'Jul', 'Agu', 'Sep', 'Okt', 'Nov', 'Des'];
      return `${d.getDate()} ${months[d.getMonth()]}`;
    }
    return rawTimestamp.includes(':') ? 'Hari Ini' : rawTimestamp;
  }

  // Group messages with date separators dynamically
  const dateGroups = $derived.by(() => {
    if (messages.length === 0) return [];
    const groups: { date: string; msgs: MessageModel[] }[] = [];
    let currentLabel = '';
    let currentMsgs: MessageModel[] = [];

    for (const msg of messages) {
      const label = formatDateLabel(msg.timestamp);
      if (label !== currentLabel) {
        if (currentMsgs.length > 0) {
          groups.push({ date: currentLabel, msgs: currentMsgs });
        }
        currentLabel = label;
        currentMsgs = [msg];
      } else {
        currentMsgs.push(msg);
      }
    }
    if (currentMsgs.length > 0) {
      groups.push({ date: currentLabel, msgs: currentMsgs });
    }
    return groups;
  });

  const disappearingDurations = [
    { label: 'Nonaktif', sec: 0 },
    { label: '30 Detik', sec: 30 },
    { label: '5 Menit', sec: 300 },
    { label: '1 Jam', sec: 3600 },
    { label: '1 Hari', sec: 86400 },
    { label: '1 Minggu', sec: 604800 }
  ];

  const peerContact = $derived.by(() => {
    if (!activeConv || activeConv.isGroup) return null;
    const peerParticipant = activeConv.participants?.find((p) => p.userId !== mekaarStore.currentUser.id);
    if (peerParticipant) {
      const byId = mekaarStore.getContactByUserId(peerParticipant.userId);
      if (byId) return byId;
    }
    return (
      mekaarStore.contacts.find((c) => c.displayName === activeConv.title) ?? {
        id: `usr-${activeConv.id}`,
        username: `@${activeConv.title.toLowerCase().replace(/\s+/g, '')}`,
        displayName: activeConv.title,
        avatarColor: activeConv.avatarColor,
        fingerprint: 'ED25519: VERIFIED-AEGIS',
        isOnline: activeConv.isOnline,
        isGuardian: !!activeConv.isGuardian,
        isBlocked: false,
        notes: 'Sesi obrolan E2EE langsung'
      }
    );
  });
</script>

{#if activeConv}
  <div class="chat-room-scaffold">
    <!-- Chat App Bar (Signal & Telegram Modern Parity) -->
    <header class="chat-appbar-container">
      <!-- 1. Tombol Kembali -->
      <button
        type="button"
        class="chat-appbar-action-btn"
        onclick={handleLeaveRoom}
        aria-label="Kembali ke Daftar Chat"
        title="Kembali"
      >
        <CaretLeft size={22} weight="bold" />
      </button>

      <!-- 2. Profil Kontak Terpusat & Ramping (flex: 1) -->
      <button
        type="button"
        class="chat-contact-profile-btn"
        onclick={() => {
          if (!activeConv.isGroup && peerContact) {
            showContactDetail = true;
          } else {
            showRoomOptions = true;
          }
        }}
        aria-label="Detail Kontak {activeConv.title}"
      >
        <SketchAvatar
          name={activeConv.title}
          size={38}
          online={activeConv.isOnline}
          isGroup={activeConv.isGroup}
        />
        <div class="chat-profile-text-col">
          <div class="chat-contact-name-row">
            <span class="chat-contact-title">{activeConv.title}</span>
            {#if activeConv.isScreenProtected}
              <span class="header-indicator-pill shield" title="Proteksi Layar & Konten Aktif">
                <ShieldCheck size={12} weight="bold" />
                <span>Aegis</span>
              </span>
            {/if}
            {#if activeConv.disappearingSeconds > 0}
              <span class="header-indicator-pill timer" title="Pesan Lenyap Aktif">
                <Timer size={12} weight="fill" />
                <span>{activeConv.disappearingSeconds >= 60 ? `${Math.round(activeConv.disappearingSeconds / 60)}m` : `${activeConv.disappearingSeconds}d`}</span>
              </span>
            {/if}
          </div>
          <span class="chat-contact-status">
            {#if activeConv.isTyping}
              <span style="color: var(--brand-blue); font-weight: 600;">sedang mengetik...</span>
            {:else if activeConv.isOnline}
              <span style="color: var(--brand-teal); font-weight: 600;">Online</span>
            {:else}
              <span>Offline</span>
            {/if}
          </span>
        </div>
      </button>

      <!-- 3. Aksi Cepat Kanan (Maksimal 2 tombol: Panggilan & Opsi) -->
      <div class="chat-appbar-actions">
        <button
          type="button"
          class="chat-appbar-action-btn"
          onclick={() => mekaarStore.startCall(activeConv, false)}
          aria-label="Panggilan Suara"
          title="Panggilan Suara E2EE"
        >
          <Phone size={21} weight="bold" color="var(--brand-blue)" />
        </button>

        <button
          type="button"
          class="chat-appbar-action-btn"
          onclick={() => (showRoomOptions = true)}
          aria-label="Opsi Obrolan"
          title="Opsi Obrolan"
        >
          <DotsThreeVertical size={21} weight="bold" color="var(--text-primary)" />
        </button>
      </div>
    </header>

    <!-- In-Chat Search Bar (Dropdown Under Header) -->
    {#if isSearchOpen}
      <div class="in-chat-search-bar">
        <div class="search-input-field">
          <MagnifyingGlass size={16} weight="bold" color="#64748B" />
          <input
            type="text"
            class="in-chat-search-input"
            placeholder="Cari kata kunci dalam obrolan..."
            bind:value={searchQuery}
            onkeydown={(e) => {
              if (e.key === 'Enter') jumpToNextSearchMatch();
            }}
          />
        </div>

        {#if searchQuery.trim()}
          <span class="search-match-counter">
            {matchedMessageIds.length > 0 ? `${currentMatchIndex + 1} dari ${matchedMessageIds.length}` : '0 cocok'}
          </span>
          <div class="search-nav-arrows">
            <button
              type="button"
              class="search-nav-btn"
              onclick={jumpToPrevSearchMatch}
              disabled={matchedMessageIds.length === 0}
              title="Kecocokan Sebelumnya"
            >
              <CaretUp size={14} weight="bold" />
            </button>
            <button
              type="button"
              class="search-nav-btn"
              onclick={jumpToNextSearchMatch}
              disabled={matchedMessageIds.length === 0}
              title="Kecocokan Berikutnya"
            >
              <CaretDown size={14} weight="bold" />
            </button>
          </div>
        {/if}

        <button
          type="button"
          class="close-search-btn"
          onclick={() => {
            isSearchOpen = false;
            searchQuery = '';
            targetJumpId = '';
          }}
          title="Tutup Pencarian"
        >
          <X size={16} weight="bold" />
        </button>
      </div>
    {/if}

    <!-- Active Call Banner (when minimized and calling with this conversation's peer) -->
    {#if mekaarStore.activeCall && mekaarStore.activeCall.isMinimized && mekaarStore.activeCall.conversationId === activeConv.id}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="in-chat-call-banner"
        onclick={() => mekaarStore.toggleCallMinimize()}
        title="Panggilan aktif • Klik untuk memperbesar layar panggilan"
      >
        <div class="call-banner-left">
          <span class="call-banner-pulse"></span>
          <span class="call-banner-title" style="display: inline-flex; align-items: center; gap: 6px;">
            {#if mekaarStore.activeCall.isVideo}
              <VideoCamera size={16} weight="fill" /> Panggilan Video
            {:else}
              <Phone size={16} weight="fill" /> Panggilan Suara
            {/if} Berlangsung
          </span>
          <span class="call-banner-timer">
            {Math.floor(mekaarStore.activeCall.durationSeconds / 60).toString().padStart(2, '0')}:{(mekaarStore.activeCall.durationSeconds % 60).toString().padStart(2, '0')}
          </span>
        </div>
        <div class="call-banner-right">
          <button
            type="button"
            class="call-banner-btn btn-maximize-call"
            onclick={(e) => {
              e.stopPropagation();
              mekaarStore.toggleCallMinimize();
            }}
            style="display: inline-flex; align-items: center; gap: 4px;"
          >
            <span>Perbesar</span>
            <CornersOut size={14} weight="bold" />
          </button>
          <button
            type="button"
            class="call-banner-btn btn-end-call-banner"
            onclick={(e) => {
              e.stopPropagation();
              mekaarStore.endCall();
            }}
            title="Akhiri Panggilan"
            style="display: inline-flex; align-items: center; gap: 4px;"
          >
            <PhoneDisconnect size={14} weight="bold" />
            <span>Akhiri</span>
          </button>
        </div>
      </div>
    {/if}

    <!-- Pinned Message Banner -->
    {#if pinnedMessage}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="pinned-message-bar"
        onclick={() => jumpToMessage(pinnedMessage.id)}
        title="Klik untuk melompat ke pesan yang disematkan"
      >
        <span class="pinned-icon" style="display: flex; align-items: center;">
          <PushPin size={18} weight="fill" color="var(--brand-blue)" />
        </span>
        <div class="pinned-text-wrap">
          <span class="pinned-label">Pesan Disematkan • {pinnedMessage.senderName}</span>
          <span class="pinned-preview">{pinnedMessage.text || (pinnedMessage.attachment ? `[Lampiran ${pinnedMessage.attachment.name}]` : 'Pesan')}</span>
        </div>
        <button
          type="button"
          class="unpin-btn"
          onclick={(e) => {
            e.stopPropagation();
            mekaarStore.pinMessage(activeConv.id, null);
          }}
          title="Lepas Sematan"
        >
          <X size={14} weight="bold" />
        </button>
      </div>
    {/if}

    <!-- Scrollable Messages Feed with Scroll FAB -->
    <main
      class="chat-messages-viewport"
      bind:this={viewportEl}
      onscroll={handleViewportScroll}
    >
      {#each dateGroups as group}
        {#if group.msgs.length > 0}
          <div class="date-pill-row">
            <span class="date-pill">{group.date}</span>
          </div>
          {#each group.msgs as msg (msg.id)}
            <ChatBubble
              message={msg}
              searchQuery={searchQuery}
              isTargetJump={targetJumpId === msg.id}
              onreply={(m) => (replyingTo = m)}
              onreact={(msgId, emoji) => mekaarStore.toggleReaction(activeConv.id, msgId, emoji)}
              onedit={(m) => {
                editingMessage = m;
                replyingTo = null;
              }}
              onrevoke={(m) => {
                revokeTarget = m;
                showRevokeDialog = true;
              }}
              onpin={(m) => {
                const currentPinned = activeConv.pinnedMessageId === m.id ? null : m.id;
                mekaarStore.pinMessage(activeConv.id, currentPinned);
              }}
              onopenlightbox={(src, name, sizeLabel, isStego) => {
                lightboxState = {
                  open: true,
                  src,
                  name,
                  sizeLabel: sizeLabel || '',
                  isStego: !!isStego
                };
              }}
              onopenstego={(src, name) => {
                selectedStegoSrc = src;
                selectedStegoName = name;
                showStegoDecoder = true;
              }}
              onopenmap={(lat, lng) => {
                mekaarStore.selectedConversationId = null;
                mekaarStore.activeTab = 'map';
              }}
              onrequestguardian={(name, lat, lng) => {
                mekaarStore.selectedConversationId = null;
                mekaarStore.activeTab = 'guardian';
              }}
              onstoplive={(msgId) => {
                mekaarStore.stopLiveLocation(activeConv.id, msgId);
              }}
              isForwardingProtected={activeConv.isForwardingProtected}
            />
          {/each}
        {/if}
      {/each}
    </main>

    <!-- Scroll To Bottom Floating Action Button -->
    {#if showScrollBottomFab}
      <button
        type="button"
        class="scroll-bottom-fab"
        onclick={scrollToBottom}
        title="Gulir ke pesan terbaru"
        aria-label="Gulir ke bawah"
      >
        <CaretDown size={20} weight="bold" />
      </button>
    {/if}

    <!-- Floating Chat Input Bar -->
    <ChatInputBar
      {replyingTo}
      {editingMessage}
      emojiPacks={mekaarStore.emojiPacks}
      oncancelreply={() => (replyingTo = null)}
      oncanceledit={() => (editingMessage = null)}
      onsaveedit={(msgId, newText) => {
        mekaarStore.editMessage(activeConv.id, msgId, newText);
        editingMessage = null;
      }}
      onsend={(text, att) => {
        mekaarStore.sendMessage(activeConv.id, text, replyingTo, att);
        replyingTo = null;
        setTimeout(scrollToBottom, 50);
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
          <p style="font-size: 0.85rem; color: var(--text-secondary); display: flex; align-items: center; gap: 5px;">
            <LockKey size={14} weight="bold" color="var(--brand-blue)" /> E2EE Chat • Aegis Protocol
          </p>
        </div>
      </div>

      <div class="options-actions-list">
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            isSearchOpen = true;
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <MagnifyingGlass size={18} weight="bold" color="var(--brand-blue)" />
            Cari dalam Obrolan
          </span>
        </button>

        {#if !activeConv.isGroup && peerContact}
          <button
            type="button"
            class="sheet-action-row"
            onclick={() => {
              showRoomOptions = false;
              showContactDetail = true;
            }}
          >
            <span style="display: flex; align-items: center; gap: 8px;">
              <UserCheck size={18} weight="bold" color="var(--brand-blue)" />
              Detail Kontak & Verifikasi Aegis
            </span>
          </button>
        {/if}
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            showPrivacySheet = true;
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <ShieldCheck size={18} weight="bold" color="var(--brand-blue)" />
            Privasi Obrolan Ekstrem (2 Arah)
          </span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            showDisappearingModal = true;
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <Timer size={18} weight="bold" color="var(--brand-blue)" />
            Atur Timer Pesan Lenyap Otomatis ({activeConv.disappearingSeconds > 0 ? `${Math.round(activeConv.disappearingSeconds / 60)}m` : 'Nonaktif'})
          </span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            mekaarStore.startCall(activeConv, false);
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <Phone size={18} weight="bold" color="var(--brand-blue)" />
            Mulai Panggilan Suara E2EE
          </span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            mekaarStore.startCall(activeConv, true);
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <VideoCamera size={18} weight="bold" color="var(--brand-blue)" />
            Mulai Panggilan Video E2EE
          </span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            showMediaVault = true;
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <FolderSimpleLock size={18} weight="bold" color="var(--brand-blue)" />
            Brankas Media & Berkas Terenkripsi
          </span>
        </button>
        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            showRoomOptions = false;
            mekaarStore.forensicDialogOpen = true;
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <Key size={18} weight="bold" color="var(--brand-blue)" />
            Ekspor Verifikasi Kriptografi Forensik
          </span>
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
          <span style="display: flex; align-items: center; gap: 8px;">
            <Trash size={18} weight="bold" color="#E11D48" />
            Hapus Riwayat Obrolan
          </span>
        </button>
      </div>
    </div>
  </BottomSheet>

  <!-- Contact Detail & Action Hub Sheet -->
  <ContactDetailSheet
    open={showContactDetail}
    contact={peerContact}
    conversationId={activeConv.id}
    onclose={() => (showContactDetail = false)}
  />

  <!-- Chat Room Privacy Sheet (2-Way Extreme Protection) -->
  <ChatRoomPrivacySheet
    open={showPrivacySheet}
    conversation={activeConv}
    onclose={() => (showPrivacySheet = false)}
  />

  <!-- Disappearing Messages Timer Dialog -->
  <Dialog
    open={showDisappearingModal}
    title="Timer Pesan Lenyap Otomatis"
    onclose={() => (showDisappearingModal = false)}
  >
    <p style="font-size: 0.88rem; color: var(--text-secondary); margin-bottom: 14px;">
      Pesan baru yang dikirim setelah timer diatur akan otomatis terhapus secara permanen dari perangkat pengirim dan penerima setelah durasi berlalu.
    </p>

    <div style="display: flex; flex-direction: column; gap: 8px;">
      {#each disappearingDurations as item}
        <button
          type="button"
          class="sheet-action-row {activeConv.disappearingSeconds === item.sec ? 'selected-timer' : ''}"
          onclick={() => {
            mekaarStore.setDisappearingTimer(activeConv.id, item.sec);
            showDisappearingModal = false;
          }}
        >
          <span>{item.label}</span>
          {#if activeConv.disappearingSeconds === item.sec}
            <span style="color: var(--brand-blue); font-weight: 700; display: inline-flex; align-items: center; gap: 4px;">
              <Check size={16} weight="bold" /> Aktif
            </span>
          {/if}
        </button>
      {/each}
    </div>
  </Dialog>

  <!-- Tarik / Hapus Pesan Confirmation Dialog -->
  <Dialog
    open={showRevokeDialog}
    title="Hapus / Tarik Pesan"
    onclose={() => (showRevokeDialog = false)}
  >
    {#if revokeTarget}
      <p style="font-size: 0.88rem; color: var(--text-secondary); margin-bottom: 16px;">
        Pilih bagaimana Anda ingin menghapus pesan:
        <br />
        <em style="color: var(--text-primary); font-weight: 600;">"{revokeTarget.text || 'Lampiran Berkas'}"</em>
      </p>

      <div style="display: flex; flex-direction: column; gap: 10px;">
        {#if revokeTarget.isSelf}
          <button
            type="button"
            class="sheet-action-row danger-action"
            onclick={() => {
              if (revokeTarget) mekaarStore.revokeMessage(activeConv.id, revokeTarget.id, true);
              showRevokeDialog = false;
              revokeTarget = null;
            }}
          >
            <span style="display: flex; align-items: center; gap: 8px;">
              <Prohibit size={18} weight="bold" color="#E11D48" />
              Tarik untuk Semua Orang (Revoke)
            </span>
          </button>
        {/if}

        <button
          type="button"
          class="sheet-action-row"
          onclick={() => {
            if (revokeTarget) mekaarStore.revokeMessage(activeConv.id, revokeTarget.id, false);
            showRevokeDialog = false;
            revokeTarget = null;
          }}
        >
          <span style="display: flex; align-items: center; gap: 8px;">
            <Trash size={18} weight="bold" color="var(--text-primary)" />
            Hapus untuk Saya Saja
          </span>
        </button>

        <button
          type="button"
          class="sheet-action-row"
          style="background: #F1F5F9; color: var(--text-secondary); text-align: center;"
          onclick={() => {
            showRevokeDialog = false;
            revokeTarget = null;
          }}
        >
          Batal
        </button>
      </div>
    {/if}
  </Dialog>

  <!-- Media Lightbox Modal Fullscreen -->
  <MediaLightboxModal
    open={lightboxState.open}
    src={lightboxState.src}
    name={lightboxState.name}
    sizeLabel={lightboxState.sizeLabel}
    isStego={lightboxState.isStego}
    onclose={() => (lightboxState.open = false)}
    onopenstego={(src, name) => {
      lightboxState.open = false;
      selectedStegoSrc = src;
      selectedStegoName = name;
      showStegoDecoder = true;
    }}
  />

  <!-- Steganografi LSB Decoder Dialog -->
  <StegoDecoderDialog
    open={showStegoDecoder}
    imageSrc={selectedStegoSrc}
    carrierName={selectedStegoName}
    onclose={() => (showStegoDecoder = false)}
  />

  <!-- Brankas Media & Berkas Terenkripsi -->
  <SecureMediaVaultDialog
    open={showMediaVault}
    onclose={() => (showMediaVault = false)}
    onOpenStegoDecoder={(src, name) => {
      selectedStegoSrc = src;
      selectedStegoName = name;
      showStegoDecoder = true;
    }}
  />
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

  /* In-Chat Search Bar */
  .in-chat-search-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 16px 8px 16px;
    background-color: var(--bg-primary);
    border-bottom: 1px solid var(--border-subtle, #E2E8F0);
    z-index: 25;
    animation: slideDown 0.15s ease;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .search-input-field {
    flex: 1;
    height: 38px;
    border-radius: var(--radius-pill);
    background-color: #FFFFFF;
    border: 1px solid var(--border-light, #CBD5E1);
    display: flex;
    align-items: center;
    padding: 0 12px;
    gap: 8px;
  }

  .in-chat-search-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    font-size: 0.85rem;
    color: var(--text-primary);
  }

  .search-match-counter {
    font-size: 0.76rem;
    font-weight: 700;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .search-nav-arrows {
    display: flex;
    gap: 2px;
  }

  .search-nav-btn {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: #FFFFFF;
    border: 1px solid var(--border-subtle, #E2E8F0);
    color: var(--text-primary);
    font-size: 0.7rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .search-nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .close-search-btn {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 0.95rem;
    cursor: pointer;
    padding: 4px;
  }

  /* Pinned Message Bar */
  .pinned-message-bar {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 16px;
    background-color: #F0FDF4;
    border-bottom: 1px solid #BBF7D0;
    cursor: pointer;
    z-index: 20;
    transition: background-color 0.15s ease;
  }

  .pinned-message-bar:hover {
    background-color: #DCFCE7;
  }

  .pinned-icon {
    font-size: 1.1rem;
    flex-shrink: 0;
  }

  .pinned-text-wrap {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .pinned-label {
    font-size: 0.72rem;
    font-weight: 700;
    color: #15803D;
  }

  .pinned-preview {
    font-size: 0.82rem;
    color: #166534;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .unpin-btn {
    background: none;
    border: none;
    color: #15803D;
    font-size: 0.85rem;
    cursor: pointer;
    padding: 4px;
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
    scroll-behavior: smooth;
  }

  /* Scroll To Bottom FAB */
  .scroll-bottom-fab {
    position: absolute;
    right: 20px;
    bottom: 84px;
    width: 42px;
    height: 42px;
    border-radius: 50%;
    background-color: #FFFFFF;
    border: 1px solid var(--border-light, #CBD5E1);
    color: var(--brand-blue, #0284C7);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 14px rgba(21, 38, 65, 0.16);
    cursor: pointer;
    z-index: 20;
    transition: transform 0.15s ease, background-color 0.15s ease;
    animation: popFab 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .scroll-bottom-fab:hover {
    transform: scale(1.08);
    background-color: #F8FAFC;
  }

  .scroll-bottom-fab:active {
    transform: scale(0.94);
  }

  @keyframes popFab {
    from {
      opacity: 0;
      transform: scale(0.7) translateY(8px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
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
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .sheet-action-row:active {
    background: #F1F5F9;
  }

  .sheet-action-row.selected-timer {
    border-color: var(--brand-blue, #0284C7);
    background-color: #EFF6FF;
  }

  .danger-action {
    color: var(--sos-coral);
    border-color: #FECDD3;
    background: #FFF1F2;
  }

  /* In-Chat Active Call Banner */
  .in-chat-call-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: linear-gradient(90deg, #064E3B, #065F46);
    color: #FFFFFF;
    border-bottom: 1px solid rgba(16, 185, 129, 0.4);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    cursor: pointer;
    z-index: 100;
  }

  .call-banner-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .call-banner-pulse {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background-color: #34D399;
    box-shadow: 0 0 10px #34D399;
    animation: bannerPulse 1.5s infinite ease-in-out;
  }

  .call-banner-title {
    font-size: 0.82rem;
    font-weight: 700;
  }

  .call-banner-timer {
    font-size: 0.78rem;
    font-family: monospace;
    background: rgba(0, 0, 0, 0.3);
    padding: 2px 6px;
    border-radius: 6px;
    color: #A7F3D0;
    font-weight: 700;
  }

  .call-banner-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .call-banner-btn {
    padding: 4px 10px;
    border-radius: var(--radius-pill, 999px);
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
    border: none;
    transition: all 0.15s ease;
  }

  .btn-maximize-call {
    background: rgba(255, 255, 255, 0.2);
    color: #FFFFFF;
  }

  .btn-maximize-call:hover {
    background: rgba(255, 255, 255, 0.35);
  }

  .btn-end-call-banner {
    background: #EF4444;
    color: #FFFFFF;
  }

  .btn-end-call-banner:hover {
    background: #DC2626;
  }

  @keyframes bannerPulse {
    0%, 100% { transform: scale(1); opacity: 1; }
    50% { transform: scale(1.3); opacity: 0.5; }
  }
</style>
