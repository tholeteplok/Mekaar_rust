<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { MessageModel } from '../types';
  import LiveLocationCard from './LiveLocationCard.svelte';
  import {
    Smiley,
    ArrowBendUpLeft,
    DotsThreeVertical,
    Copy,
    Prohibit,
    PencilSimple,
    PushPin,
    Trash,
    MagnifyingGlassPlus,
    Image,
    EyeSlash,
    Play,
    Pause,
    FileText,
    Timer,
    LockKey,
    Clock,
    Check,
    Checks
  } from 'phosphor-svelte';

  interface Props {
    message: MessageModel;
    searchQuery?: string;
    isTargetJump?: boolean;
    onreply?: (msg: MessageModel) => void;
    onreact?: (msgId: string, emoji: string) => void;
    onopenstego?: (imgSrc: string, name: string) => void;
    onopenlightbox?: (imgSrc: string, name: string, sizeLabel?: string, isStego?: boolean) => void;
    onedit?: (msg: MessageModel) => void;
    onrevoke?: (msg: MessageModel) => void;
    onpin?: (msg: MessageModel) => void;
    onopenmap?: (lat?: number, lng?: number) => void;
    onrequestguardian?: (name: string, lat?: number, lng?: number) => void;
    onstoplive?: (msgId: string) => void;
    isForwardingProtected?: boolean;
  }

  let {
    message,
    searchQuery = '',
    isTargetJump = false,
    onreply,
    onreact,
    onopenstego,
    onopenlightbox,
    onedit,
    onrevoke,
    onpin,
    onopenmap,
    onrequestguardian,
    onstoplive,
    isForwardingProtected = false
  }: Props = $props();

  // Voice player state
  let isPlayingAudio = $state(false);
  let audioProgress = $state(0); // 0 to 100%
  let playbackSpeed = $state<1 | 1.5 | 2>(1);
  let audioTimer: any = null;

  // Popover menus state
  let showReactionMenu = $state(false);
  let showMoreMenu = $state(false);
  const quickEmojis = ['👍', '❤️', '😂', '😮', '😢', '🙏'];

  function toggleAudio() {
    isPlayingAudio = !isPlayingAudio;
    if (isPlayingAudio) {
      if (audioProgress >= 100) audioProgress = 0;
      clearInterval(audioTimer);
      audioTimer = setInterval(() => {
        audioProgress += (2.5 * playbackSpeed);
        if (audioProgress >= 100) {
          audioProgress = 100;
          isPlayingAudio = false;
          clearInterval(audioTimer);
        }
      }, 200);
    } else {
      clearInterval(audioTimer);
    }
  }

  function cycleSpeed() {
    if (playbackSpeed === 1) playbackSpeed = 1.5;
    else if (playbackSpeed === 1.5) playbackSpeed = 2;
    else playbackSpeed = 1;
  }

  function handleSeek(e: MouseEvent) {
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const pct = Math.max(0, Math.min(100, (clickX / rect.width) * 100));
    audioProgress = pct;
  }

  function handleAddReaction(emoji: string) {
    showReactionMenu = false;
    onreact?.(message.id, emoji);
  }

  function formatAudioDuration(progressPct: number, totalSeconds = 8) {
    const currentSec = Math.floor((progressPct / 100) * totalSeconds);
    const mCur = Math.floor(currentSec / 60);
    const sCur = currentSec % 60;
    const mTot = Math.floor(totalSeconds / 60);
    const sTot = totalSeconds % 60;
    return `${mCur}:${sCur < 10 ? '0' : ''}${sCur} / ${mTot}:${sTot < 10 ? '0' : ''}${sTot}`;
  }

  onDestroy(() => {
    if (audioTimer) clearInterval(audioTimer);
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  id="bubble-{message.id}"
  class="bubble-row {message.isSelf ? 'self' : 'peer'}"
  onmouseleave={() => {
    showReactionMenu = false;
    showMoreMenu = false;
  }}
>
  <div class="bubble-stack">
    <!-- Bubble Container -->
    <div class="bubble-container {message.isSelf ? 'bubble-self' : 'bubble-peer'} {isTargetJump ? 'target-jump-pulse' : ''} {isForwardingProtected ? 'protected-content' : ''}">
      <!-- Quick Reaction & Actions Float Trigger (visible on hover) -->
      {#if !message.isRevoked}
        <div class="bubble-hover-actions {message.isSelf ? 'actions-self' : 'actions-peer'}">
          <!-- Emoji Reaction Trigger -->
          <button
            type="button"
            class="hover-action-btn"
            onclick={() => {
              showMoreMenu = false;
              showReactionMenu = !showReactionMenu;
            }}
            title="Beri Reaksi Emoji"
            aria-label="Reaksi"
          >
            <Smiley size={14} weight="bold" />
          </button>

          <!-- Reply Button -->
          {#if onreply}
            <button
              type="button"
              class="hover-action-btn"
              onclick={() => onreply!(message)}
              title="Balas Pesan"
              aria-label="Balas"
            >
              <ArrowBendUpLeft size={14} weight="bold" />
            </button>
          {/if}

          <!-- More Options Menu Trigger -->
          <button
            type="button"
            class="hover-action-btn"
            onclick={() => {
              showReactionMenu = false;
              showMoreMenu = !showMoreMenu;
            }}
            title="Opsi Pesan Lanjutan"
            aria-label="Opsi Pesan"
          >
            <DotsThreeVertical size={14} weight="bold" />
          </button>
        </div>
      {/if}

      <!-- Quick Reaction Popup Toolbar -->
      {#if showReactionMenu}
        <div class="quick-reactions-bar {message.isSelf ? 'bar-self' : 'bar-peer'}">
          {#each quickEmojis as emoji}
            <button
              type="button"
              class="quick-emoji-btn"
              onclick={() => handleAddReaction(emoji)}
            >
              {emoji}
            </button>
          {/each}
        </div>
      {/if}

      <!-- More Options Dropdown Menu -->
      {#if showMoreMenu}
        <div class="more-options-menu {message.isSelf ? 'menu-self' : 'menu-peer'}">
          {#if onreply}
            <button
              type="button"
              class="menu-item-btn"
              onclick={() => {
                showMoreMenu = false;
                onreply!(message);
              }}
            >
              <ArrowBendUpLeft size={16} weight="bold" /> Balas Pesan
            </button>
          {/if}

          {#if !isForwardingProtected && message.text}
            <button
              type="button"
              class="menu-item-btn"
              onclick={() => {
                showMoreMenu = false;
                if (typeof navigator !== 'undefined' && navigator.clipboard) {
                  navigator.clipboard.writeText(message.text);
                }
              }}
            >
              <Copy size={16} weight="bold" /> Salin Teks
            </button>
          {:else if isForwardingProtected}
            <div class="menu-item-btn item-disabled" title="Penyalinan dilarang oleh proteksi privasi obrolan">
              <Prohibit size={16} weight="bold" /> Salin Dilarang
            </div>
          {/if}

          {#if message.isSelf && !message.isRevoked && onedit}
            <button
              type="button"
              class="menu-item-btn"
              onclick={() => {
                showMoreMenu = false;
                onedit!(message);
              }}
            >
              <PencilSimple size={16} weight="bold" /> Edit Pesan
            </button>
          {/if}

          {#if onpin}
            <button
              type="button"
              class="menu-item-btn"
              onclick={() => {
                showMoreMenu = false;
                onpin!(message);
              }}
            >
              <PushPin size={16} weight="bold" /> Sematkan Pesan
            </button>
          {/if}

          {#if !message.isRevoked && onrevoke}
            <button
              type="button"
              class="menu-item-btn danger-item"
              onclick={() => {
                showMoreMenu = false;
                onrevoke!(message);
              }}
            >
              <Trash size={16} weight="bold" /> {message.isSelf ? 'Tarik Pesan' : 'Hapus untuk Saya'}
            </button>
          {/if}
        </div>
      {/if}

      {#if !message.isSelf && message.senderName}
        <div class="sender-name">{message.senderName}</div>
      {/if}

      {#if message.replyPreviewText && !message.isRevoked}
        <div class="reply-box {message.isSelf ? 'reply-self' : 'reply-peer'}">
          <span class="reply-author">{message.replySenderName ?? 'Pesan'}:</span>
          <span class="reply-text">{message.replyPreviewText}</span>
        </div>
      {/if}

      <!-- REVOKED MESSAGE PRESENTATION -->
      {#if message.isRevoked}
        <div class="revoked-message-body {message.isSelf ? 'revoked-self' : 'revoked-peer'}">
          <span class="revoked-icon" style="display: flex; align-items: center;">
            <Prohibit size={16} weight="bold" color="var(--text-secondary)" />
          </span>
          <span class="revoked-text">Pesan ini telah ditarik</span>
        </div>
      {:else}
        <!-- Attachment Rendering -->
        {#if message.attachment}
          {#if message.attachment.kind === 'image'}
            <div class="image-attachment-wrap">
              {#if message.attachment.urlOrPreview && message.attachment.urlOrPreview.startsWith('data:image')}
                <button
                  type="button"
                  class="img-lightbox-trigger-btn"
                  onclick={() => onopenlightbox?.(
                    message.attachment!.urlOrPreview,
                    message.attachment!.name,
                    message.attachment!.sizeLabel,
                    message.attachment!.isStego
                  )}
                  title="Klik untuk pratinjau layar penuh & zoom"
                >
                  <img
                    src={message.attachment.urlOrPreview}
                    alt={message.attachment.name}
                    class="bubble-inline-img"
                  />
                  <div class="img-zoom-hint-overlay">
                    <MagnifyingGlassPlus size={24} weight="bold" color="#FFFFFF" />
                  </div>
                </button>
              {:else}
                <button
                  type="button"
                  class="img-generic-preview"
                  onclick={() => onopenlightbox?.(
                    message.attachment!.urlOrPreview,
                    message.attachment!.name,
                    message.attachment!.sizeLabel,
                    message.attachment!.isStego
                  )}
                >
                  <span style="display: flex; align-items: center; gap: 6px;">
                    <Image size={18} weight="bold" /> {message.attachment.name}
                  </span>
                </button>
              {/if}

              <!-- Stego Badge & Extraction Action -->
              {#if message.attachment.isStego}
                <div class="stego-banner-overlay">
                  <div class="stego-badge-label">
                    <span style="display: flex; align-items: center; gap: 4px;">
                      <EyeSlash size={14} weight="bold" /> Steganografi LSB
                    </span>
                  </div>
                  {#if onopenstego}
                    <button
                      type="button"
                      class="btn-extract-stego-pill"
                      onclick={() => onopenstego!(message.attachment!.urlOrPreview, message.attachment!.name)}
                    >
                      Ekstrak Pesan
                    </button>
                  {/if}
                </div>
              {/if}
              <div class="img-meta-row">
                <span class="img-size-sub">{message.attachment.sizeLabel}</span>
              </div>
            </div>
          {:else if message.attachment.kind === 'voice'}
            <!-- Voice Note Player Pill -->
            <div class="voice-bubble-pill {message.isSelf ? 'voice-self' : 'voice-peer'}">
              <button
                type="button"
                class="voice-play-circle"
                onclick={toggleAudio}
                aria-label={isPlayingAudio ? 'Jeda Audio' : 'Putar Audio'}
              >
                {#if isPlayingAudio}
                  <Pause size={14} weight="fill" />
                {:else}
                  <Play size={14} weight="fill" />
                {/if}
              </button>

              <div class="voice-waveform-col">
                <!-- Interactive Timeline Scrubber -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div
                  class="timeline-track"
                  onclick={handleSeek}
                  onkeydown={(e) => {
                    if (e.key === 'ArrowRight') audioProgress = Math.min(100, audioProgress + 5);
                    else if (e.key === 'ArrowLeft') audioProgress = Math.max(0, audioProgress - 5);
                  }}
                  role="slider"
                  tabindex="0"
                  aria-valuenow={Math.round(audioProgress)}
                  aria-valuemin="0"
                  aria-valuemax="100"
                >
                  <div class="waveform-bars-row">
                    {#each [8, 14, 20, 10, 16, 24, 12, 18, 6, 14, 22, 16, 8, 12, 18, 10, 14] as h, i}
                      {@const isPassed = (i / 17) * 100 <= audioProgress}
                      <span
                        class="wave-bar {isPassed ? 'wave-bar-passed' : 'wave-bar-future'} {isPlayingAudio && i % 3 === 0 ? 'anim-wave' : ''}"
                        style="height: {h}px;"
                      ></span>
                    {/each}
                  </div>
                  <!-- Progress Line Indicator -->
                  <div class="progress-fill-line" style="width: {audioProgress}%;"></div>
                </div>

                <!-- Audio Timer & Speed Toggle -->
                <div class="voice-bottom-row">
                  <span class="voice-duration-label">
                    {isPlayingAudio || audioProgress > 0
                      ? formatAudioDuration(audioProgress)
                      : message.attachment.sizeLabel}
                  </span>
                  <button
                    type="button"
                    class="speed-toggle-pill {message.isSelf ? 'speed-self' : 'speed-peer'}"
                    onclick={cycleSpeed}
                    title="Ubah kecepatan audio"
                  >
                    {playbackSpeed}x
                  </button>
                </div>
              </div>
            </div>
          {:else if message.attachment.kind === 'location'}
            <LiveLocationCard
              attachment={message.attachment}
              isSelf={message.isSelf}
              onopenmap={(lat, lng) => onopenmap?.(lat, lng)}
              onrequestguardian={(name, lat, lng) => onrequestguardian?.(name, lat, lng)}
              onstoplive={() => onstoplive?.(message.id)}
            />
          {:else}
            <!-- Document Attachment Box -->
            <div class="attachment-box {message.isSelf ? 'att-self' : 'att-peer'}">
              <span class="att-icon" style="display: flex; align-items: center;">
                <FileText size={22} weight="bold" color="var(--brand-blue)" />
              </span>
              <div class="att-info">
                <span class="att-name">{message.attachment.name}</span>
                <span class="att-size">{message.attachment.sizeLabel}</span>
              </div>
            </div>
          {/if}
        {/if}

        <!-- Text Body with Search Highlight Support -->
        {#if message.text}
          <div class="message-body">
            {#if searchQuery && searchQuery.trim()}
              {@const q = searchQuery.trim().toLowerCase()}
              {@const lower = message.text.toLowerCase()}
              {#if lower.includes(q)}
                {@const parts = message.text.split(new RegExp(`(${searchQuery.trim().replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi'))}
                {#each parts as part}
                  {#if part.toLowerCase() === q}
                    <mark class="search-match-highlight">{part}</mark>
                  {:else}
                    {part}
                  {/if}
                {/each}
              {:else}
                {message.text}
              {/if}
            {:else}
              {message.text}
            {/if}
          </div>
        {/if}
      {/if}

      <!-- Bubble Meta: Disappearing Timer, Lock, Timestamp, Edited label, Delivery Status -->
      <div class="bubble-footer {message.isSelf ? 'footer-self' : 'footer-peer'}">
        {#if message.disappearingSeconds || message.expiresAt}
          <span class="disappearing-badge" title="Pesan Lenyap Otomatis" style="display: inline-flex; align-items: center; gap: 3px;">
            <Timer size={11} weight="bold" /> {message.expiresAt || `${message.disappearingSeconds}s`}
          </span>
        {/if}

        <span style="display: inline-flex; align-items: center;">
          <LockKey size={11} weight="bold" color="currentColor" />
        </span>

        {#if message.isEdited}
          <span class="edited-badge" title="Pesan telah diedit">(diedit)</span>
        {/if}

        <span class="msg-time">{message.timestamp}</span>

        {#if message.isSelf && !message.isRevoked}
          <!-- Telegram / Signal 4-State Delivery Receipts -->
          {#if message.status === 'sending'}
            <span class="status-receipt sending" title="Mengirim..." style="display: inline-flex; align-items: center;">
              <Clock size={12} weight="bold" />
            </span>
          {:else if message.status === 'sent'}
            <span class="status-receipt sent" title="Terkirim ke Server" style="display: inline-flex; align-items: center;">
              <Check size={13} weight="bold" />
            </span>
          {:else if message.status === 'delivered'}
            <span class="status-receipt delivered" title="Terkirim ke Perangkat" style="display: inline-flex; align-items: center;">
              <Checks size={14} weight="bold" />
            </span>
          {:else if message.status === 'read'}
            <span class="status-receipt read" title="Dibaca" style="display: inline-flex; align-items: center;">
              <Checks size={14} weight="bold" color="var(--brand-blue)" />
            </span>
          {:else}
            <span class="status-receipt delivered" style="display: inline-flex; align-items: center;">
              <Checks size={14} weight="bold" />
            </span>
          {/if}
        {/if}
      </div>
    </div>

    <!-- Reaction Badges Row (Below Bubble) -->
    {#if !message.isRevoked && message.reactions && Object.keys(message.reactions).length > 0}
      <div class="reactions-badge-row {message.isSelf ? 'row-self' : 'row-peer'}">
        {#each Object.entries(message.reactions) as [emoji, users]}
          {#if users.length > 0}
            <button
              type="button"
              class="reaction-badge-pill {users.includes('self') || users.includes('usr-teplok-01') ? 'reacted-by-me' : ''}"
              onclick={() => handleAddReaction(emoji)}
              title="{users.join(', ')}"
            >
              <span class="reaction-emoji">{emoji}</span>
              <span class="reaction-count">{users.length}</span>
            </button>
          {/if}
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .bubble-row {
    display: flex;
    margin-bottom: 8px;
    width: 100%;
    position: relative;
  }

  .bubble-row.self {
    justify-content: flex-end;
  }

  .bubble-row.peer {
    justify-content: flex-start;
  }

  .bubble-stack {
    display: flex;
    flex-direction: column;
    max-width: min(80%, 460px);
    position: relative;
  }

  .bubble-container {
    position: relative;
    padding: 10px 14px 6px 14px;
    box-shadow: 0 1px 4px rgba(21, 38, 65, 0.05);
    word-break: break-word;
    box-sizing: border-box;
    transition: box-shadow 0.3s ease, border-color 0.3s ease;
  }

  /* Target Jump Pulse Animation */
  .target-jump-pulse {
    animation: jumpGlow 1.8s ease;
  }

  @keyframes jumpGlow {
    0% {
      box-shadow: 0 0 0 3px #38BDF8, 0 4px 16px rgba(56, 189, 248, 0.4);
      transform: scale(1.02);
    }
    50% {
      box-shadow: 0 0 0 4px #0284C7, 0 6px 20px rgba(2, 132, 199, 0.3);
      transform: scale(1.03);
    }
    100% {
      box-shadow: 0 1px 4px rgba(21, 38, 65, 0.05);
      transform: scale(1);
    }
  }

  /* Outgoing: Vibrant Blue */
  .bubble-self {
    background-color: var(--brand-blue, #0284C7);
    color: #FFFFFF;
    border-radius: 18px 18px 4px 18px;
  }

  /* Incoming: Clean White */
  .bubble-peer {
    background-color: #FFFFFF;
    color: var(--text-primary, #0F172A);
    border: 1px solid var(--border-light, #E2E8F0);
    border-radius: 18px 18px 18px 4px;
  }

  /* Hover Actions */
  .bubble-hover-actions {
    position: absolute;
    top: -12px;
    display: flex;
    gap: 4px;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.15s ease;
    z-index: 5;
  }

  .bubble-row:hover .bubble-hover-actions {
    opacity: 1;
    pointer-events: auto;
  }

  .actions-self {
    right: 8px;
  }

  .actions-peer {
    left: 8px;
  }

  .hover-action-btn {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background-color: #FFFFFF;
    border: 1px solid var(--border-subtle, #CBD5E1);
    color: var(--text-secondary, #64748B);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .hover-action-btn:hover {
    color: var(--brand-blue, #0284C7);
    transform: scale(1.1);
  }

  /* Quick Reactions Bar Popover */
  .quick-reactions-bar {
    position: absolute;
    top: -42px;
    background-color: #FFFFFF;
    border: 1px solid var(--border-subtle, #CBD5E1);
    border-radius: 999px;
    padding: 3px 6px;
    display: flex;
    align-items: center;
    gap: 4px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    z-index: 10;
    animation: popIn 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .bar-self {
    right: 0;
  }

  .bar-peer {
    left: 0;
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: scale(0.85) translateY(4px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  .quick-emoji-btn {
    background: none;
    border: none;
    font-size: 1.15rem;
    padding: 3px 5px;
    cursor: pointer;
    border-radius: 50%;
    transition: transform 0.15s ease;
    line-height: 1;
  }

  .quick-emoji-btn:hover {
    transform: scale(1.3);
  }

  /* More Options Dropdown Menu */
  .more-options-menu {
    position: absolute;
    top: -10px;
    background-color: #FFFFFF;
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: 12px;
    box-shadow: 0 6px 20px rgba(15, 23, 42, 0.18);
    display: flex;
    flex-direction: column;
    padding: 4px;
    z-index: 25;
    min-width: 160px;
    animation: popIn 0.15s ease;
  }

  .menu-self {
    right: 0;
  }

  .menu-peer {
    left: 0;
  }

  .menu-item-btn {
    background: none;
    border: none;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 12px;
    font-size: 0.82rem;
    font-weight: 600;
    color: var(--text-primary, #0F172A);
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    transition: background-color 0.12s ease;
  }

  .menu-item-btn:hover {
    background-color: #F1F5F9;
  }

  .menu-item-btn.danger-item {
    color: #E11D48;
  }

  .menu-item-btn.danger-item:hover {
    background-color: #FFF1F2;
  }

  .sender-name {
    font-size: 0.78rem;
    font-weight: 700;
    color: var(--brand-blue, #0284C7);
    margin-bottom: 3px;
  }

  .reply-box {
    display: flex;
    flex-direction: column;
    padding: 4px 8px;
    border-radius: 6px;
    margin-bottom: 6px;
    font-size: 0.78rem;
  }

  .reply-self {
    background-color: rgba(255, 255, 255, 0.18);
    border-left: 3px solid #FFFFFF;
  }

  .reply-peer {
    background-color: #F1F5F9;
    border-left: 3px solid var(--brand-blue, #0284C7);
  }

  .reply-author {
    font-weight: 700;
  }

  .reply-text {
    opacity: 0.85;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Revoked Message Presentation */
  .revoked-message-body {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 0;
    font-style: italic;
    font-size: 0.88rem;
  }

  .revoked-self {
    color: rgba(255, 255, 255, 0.8);
  }

  .revoked-peer {
    color: var(--text-secondary, #64748B);
  }

  .revoked-icon {
    font-style: normal;
    font-size: 0.95rem;
  }

  /* Search Highlight */
  .search-match-highlight {
    background-color: #FEF08A;
    color: #854D0E;
    padding: 1px 3px;
    border-radius: 3px;
    font-weight: 700;
  }

  /* Image Attachment */
  .image-attachment-wrap {
    margin-bottom: 6px;
    border-radius: 12px;
    overflow: hidden;
    position: relative;
    background-color: #0F172A;
  }

  .img-lightbox-trigger-btn {
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    cursor: pointer;
    display: block;
    width: 100%;
    position: relative;
  }

  .bubble-inline-img {
    width: 100%;
    max-height: 220px;
    object-fit: cover;
    display: block;
    border-radius: 10px;
    transition: filter 0.15s ease;
  }

  .img-zoom-hint-overlay {
    position: absolute;
    inset: 0;
    background-color: rgba(15, 23, 42, 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .img-lightbox-trigger-btn:hover .img-zoom-hint-overlay {
    opacity: 1;
  }

  .img-generic-preview {
    padding: 30px 16px;
    text-align: center;
    color: #FFFFFF;
    font-size: 0.85rem;
    font-weight: 600;
    background: none;
    border: none;
    width: 100%;
    cursor: pointer;
  }

  .stego-banner-overlay {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 10px;
    background-color: rgba(15, 23, 42, 0.85);
    backdrop-filter: blur(4px);
    border-radius: 0 0 10px 10px;
  }

  .stego-badge-label {
    display: flex;
    align-items: center;
    gap: 4px;
    color: #38BDF8;
    font-size: 0.72rem;
    font-weight: 800;
  }

  .btn-extract-stego-pill {
    padding: 3px 10px;
    background-color: #0D9488;
    color: #FFFFFF;
    font-size: 0.7rem;
    font-weight: 700;
    border-radius: var(--radius-pill, 999px);
    border: none;
    cursor: pointer;
  }

  .img-meta-row {
    padding: 4px 6px;
  }

  .img-size-sub {
    font-size: 0.7rem;
    opacity: 0.8;
  }

  /* Voice Note Pill */
  .voice-bubble-pill {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: 12px;
    margin-bottom: 6px;
  }

  .voice-self {
    background-color: rgba(255, 255, 255, 0.16);
  }

  .voice-peer {
    background-color: #F8FAFC;
    border: 1px solid var(--border-subtle, #E2E8F0);
  }

  .voice-play-circle {
    width: 34px;
    height: 34px;
    border-radius: 50%;
    background-color: #FFFFFF;
    color: var(--brand-blue, #0284C7);
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    flex-shrink: 0;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
  }

  .voice-waveform-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .timeline-track {
    position: relative;
    cursor: pointer;
    padding: 4px 0;
  }

  .waveform-bars-row {
    display: flex;
    align-items: center;
    gap: 2.5px;
    height: 24px;
  }

  .wave-bar {
    width: 2.5px;
    border-radius: 1px;
    transition: all 0.15s ease;
  }

  .wave-bar-passed {
    background-color: currentColor;
    opacity: 1;
  }

  .wave-bar-future {
    background-color: currentColor;
    opacity: 0.45;
  }

  .anim-wave {
    filter: brightness(1.4);
    transform: scaleY(1.15);
  }

  .progress-fill-line {
    height: 2px;
    background-color: currentColor;
    border-radius: 1px;
    margin-top: 2px;
    opacity: 0.7;
    transition: width 0.1s linear;
  }

  .voice-bottom-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .voice-duration-label {
    font-size: 0.7rem;
    opacity: 0.85;
    font-weight: 500;
  }

  .speed-toggle-pill {
    padding: 1px 6px;
    font-size: 0.68rem;
    font-weight: 700;
    border-radius: 999px;
    cursor: pointer;
    border: none;
    transition: background-color 0.15s ease;
  }

  .speed-self {
    background-color: rgba(255, 255, 255, 0.25);
    color: #FFFFFF;
  }

  .speed-peer {
    background-color: #E2E8F0;
    color: #334155;
  }

  /* Document / Generic Box */
  .attachment-box {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 10px;
    margin-bottom: 6px;
  }

  .att-self {
    background-color: rgba(255, 255, 255, 0.16);
  }

  .att-peer {
    background-color: #F8FAFC;
    border: 1px solid var(--border-subtle, #CBD5E1);
  }

  .att-icon {
    font-size: 1.5rem;
  }

  .att-info {
    display: flex;
    flex-direction: column;
  }

  .att-name {
    font-size: 0.85rem;
    font-weight: 700;
  }

  .att-size {
    font-size: 0.72rem;
    opacity: 0.8;
  }

  .message-body {
    font-size: 0.94rem;
    line-height: 1.45;
  }

  .bubble-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 4px;
    margin-top: 4px;
    font-size: 0.72rem;
    user-select: none;
  }

  .footer-self {
    color: rgba(255, 255, 255, 0.85);
  }

  .footer-peer {
    color: var(--text-secondary, #64748B);
  }

  .disappearing-badge {
    font-size: 0.68rem;
    opacity: 0.85;
    margin-right: 2px;
  }

  .edited-badge {
    font-size: 0.68rem;
    opacity: 0.8;
    font-style: italic;
  }

  .msg-time {
    font-weight: 500;
  }

  /* Status Receipts */
  .status-receipt {
    font-weight: 700;
    margin-left: 2px;
  }

  .status-receipt.sending {
    font-size: 0.65rem;
    opacity: 0.7;
  }

  .status-receipt.sent {
    opacity: 0.8;
  }

  .status-receipt.delivered {
    opacity: 0.85;
  }

  .status-receipt.read {
    color: #38BDF8;
    text-shadow: 0 0 3px rgba(56, 189, 248, 0.4);
  }

  /* Reactions Badges Row */
  .reactions-badge-row {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 2px;
  }

  .row-self {
    justify-content: flex-end;
  }

  .row-peer {
    justify-content: flex-start;
  }

  .reaction-badge-pill {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 7px;
    border-radius: 999px;
    background-color: #FFFFFF;
    border: 1px solid var(--border-subtle, #E2E8F0);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
    cursor: pointer;
    font-size: 0.75rem;
    color: var(--text-primary, #0F172A);
    transition: transform 0.15s ease, background-color 0.15s ease;
  }

  .reaction-badge-pill:hover {
    transform: scale(1.08);
  }

  .reaction-badge-pill.reacted-by-me {
    background-color: rgba(2, 132, 199, 0.1);
    border-color: var(--brand-blue, #0284C7);
    color: var(--brand-blue, #0284C7);
  }

  .reaction-emoji {
    font-size: 0.85rem;
    line-height: 1;
  }

  .reaction-count {
    font-weight: 700;
    font-size: 0.72rem;
  }

  .protected-content {
    user-select: none !important;
    -webkit-user-select: none !important;
  }

  .item-disabled {
    opacity: 0.55;
    cursor: not-allowed !important;
    color: var(--text-muted);
  }
</style>
