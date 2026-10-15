<script lang="ts">
  import type { MessageModel } from '../types';

  interface Props {
    message: MessageModel;
    onreply?: (msg: MessageModel) => void;
    onreact?: (msgId: string, emoji: string) => void;
    onopenstego?: (imgSrc: string, name: string) => void;
  }

  let { message, onreply, onreact, onopenstego }: Props = $props();

  let isPlayingAudio = $state(false);

  function toggleAudio() {
    isPlayingAudio = !isPlayingAudio;
    if (isPlayingAudio) {
      setTimeout(() => {
        isPlayingAudio = false;
      }, 4000);
    }
  }
</script>

<div class="bubble-row {message.isSelf ? 'self' : 'peer'}">
  <div class="bubble-container {message.isSelf ? 'bubble-self' : 'bubble-peer'}">
    {#if !message.isSelf && message.senderName}
      <div class="sender-name">{message.senderName}</div>
    {/if}

    {#if message.replyPreviewText}
      <div class="reply-box {message.isSelf ? 'reply-self' : 'reply-peer'}">
        <span class="reply-author">{message.replySenderName ?? 'Pesan'}:</span>
        <span class="reply-text">{message.replyPreviewText}</span>
      </div>
    {/if}

    <!-- Attachment Rendering -->
    {#if message.attachment}
      {#if message.attachment.kind === 'image'}
        <div class="image-attachment-wrap">
          {#if message.attachment.urlOrPreview && message.attachment.urlOrPreview.startsWith('data:image')}
            <img
              src={message.attachment.urlOrPreview}
              alt={message.attachment.name}
              class="bubble-inline-img"
            />
          {:else}
            <div class="img-generic-preview">
              <span>🖼️ {message.attachment.name}</span>
            </div>
          {/if}

          <!-- Stego Badge & Extraction Action -->
          {#if message.attachment.isStego}
            <div class="stego-banner-overlay">
              <div class="stego-badge-label">
                <span>🕵️ Steganografi LSB</span>
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
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <rect x="6" y="4" width="4" height="16"></rect>
                <rect x="14" y="4" width="4" height="16"></rect>
              </svg>
            {:else}
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <polygon points="5 3 19 12 5 21 5 3"></polygon>
              </svg>
            {/if}
          </button>
          <div class="voice-waveform-col">
            <div class="waveform-bars-row">
              {#each [8, 14, 20, 10, 16, 24, 12, 18, 6, 14, 22, 16, 8, 12, 18] as h, i}
                <span
                  class="wave-bar {isPlayingAudio && i % 3 === 0 ? 'anim-wave' : ''}"
                  style="height: {h}px;"
                ></span>
              {/each}
            </div>
            <span class="voice-duration-label">
              {isPlayingAudio ? '00:03 / 00:15' : message.attachment.sizeLabel}
            </span>
          </div>
        </div>
      {:else}
        <!-- Document / Location Attachment Box -->
        <div class="attachment-box {message.isSelf ? 'att-self' : 'att-peer'}">
          <span class="att-icon">
            {#if message.attachment.kind === 'location'}📍{:else}📄{/if}
          </span>
          <div class="att-info">
            <span class="att-name">{message.attachment.name}</span>
            <span class="att-size">{message.attachment.sizeLabel}</span>
          </div>
        </div>
      {/if}
    {/if}

    {#if message.text}
      <div class="message-body">{message.text}</div>
    {/if}

    <!-- Bubble Meta: Lock, Timestamp, Status Checks -->
    <div class="bubble-footer {message.isSelf ? 'footer-self' : 'footer-peer'}">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" class="lock-icon">
        <path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z"/>
      </svg>
      <span class="msg-time">{message.timestamp}</span>
      {#if message.isSelf}
        <span class="read-receipt">{message.status === 'read' ? '✓✓' : '✓'}</span>
      {/if}
    </div>
  </div>
</div>

<style>
  .bubble-row {
    display: flex;
    margin-bottom: 8px;
    width: 100%;
  }

  .bubble-row.self {
    justify-content: flex-end;
  }

  .bubble-row.peer {
    justify-content: flex-start;
  }

  .bubble-container {
    position: relative;
    max-width: min(80%, 460px);
    padding: 10px 14px 6px 14px;
    box-shadow: 0 1px 4px rgba(21, 38, 65, 0.05);
    word-break: break-word;
    box-sizing: border-box;
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

  /* Image Attachment */
  .image-attachment-wrap {
    margin-bottom: 6px;
    border-radius: 12px;
    overflow: hidden;
    position: relative;
    background-color: #0F172A;
  }

  .bubble-inline-img {
    width: 100%;
    max-height: 220px;
    object-fit: cover;
    display: block;
    border-radius: 10px;
  }

  .img-generic-preview {
    padding: 30px 16px;
    text-align: center;
    color: #FFFFFF;
    font-size: 0.85rem;
    font-weight: 600;
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
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background-color: #FFFFFF;
    color: var(--brand-blue, #0284C7);
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    flex-shrink: 0;
  }

  .voice-waveform-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .waveform-bars-row {
    display: flex;
    align-items: center;
    gap: 2.5px;
    height: 24px;
  }

  .wave-bar {
    width: 2.5px;
    background-color: currentColor;
    opacity: 0.65;
    border-radius: 1px;
    transition: all 0.15s ease;
  }

  .anim-wave {
    opacity: 1;
    filter: brightness(1.3);
  }

  .voice-duration-label {
    font-size: 0.7rem;
    opacity: 0.8;
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

  .lock-icon {
    opacity: 0.75;
  }

  .msg-time {
    font-weight: 500;
  }

  .read-receipt {
    font-weight: 700;
    margin-left: 2px;
  }
</style>
