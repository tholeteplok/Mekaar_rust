<script lang="ts">
  import type { MessageModel, EmojiPackModel } from '../types';
  import AttachMediaDialog from './AttachMediaDialog.svelte';

  interface Props {
    replyingTo?: MessageModel | null;
    emojiPacks?: EmojiPackModel[];
    oncancelreply?: () => void;
    onsend: (text: string, attachment?: MessageModel['attachment']) => void;
  }

  let {
    replyingTo = null,
    emojiPacks = [],
    oncancelreply,
    onsend
  }: Props = $props();

  let text = $state('');
  let isRecording = $state(false);
  let showAttachDialog = $state(false);

  function submitText() {
    if (!text.trim()) return;
    onsend(text.trim());
    text = '';
  }

  function handleAttachClick() {
    showAttachDialog = true;
  }

  function toggleVoiceRecord() {
    if (isRecording) {
      isRecording = false;
      onsend('', {
        id: `att-${Date.now()}`,
        kind: 'voice',
        name: 'Pesan Suara (00:08)',
        sizeLabel: '0:08 • Opus E2EE',
        urlOrPreview: 'voice'
      });
    } else {
      isRecording = true;
    }
  }
</script>

<div class="chat-input-dock-wrap">
  {#if replyingTo}
    <div class="reply-floating-bar">
      <div class="reply-meta">
        <span class="reply-label">Membalas {replyingTo.senderName}:</span>
        <span class="reply-text-preview">{replyingTo.text}</span>
      </div>
      <button type="button" class="close-reply-btn" onclick={oncancelreply}>✕</button>
    </div>
  {/if}

  <div class="dock-row">
    <!-- 1. Circular Attachment Button -->
    <button
      type="button"
      class="circle-action-btn"
      onclick={handleAttachClick}
      title="Lampirkan Dokumen / Gambar"
      aria-label="Lampiran"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path>
      </svg>
    </button>

    <!-- 2. Pill Input Container -->
    <div class="input-pill-field">
      <button
        type="button"
        class="emoji-trigger-btn"
        onclick={() => (text += ' 😊 ')}
        title="Emoji"
        aria-label="Emoji"
      >
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#60A5FA" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
          <line x1="9" y1="9" x2="9.01" y2="9"></line>
          <line x1="15" y1="9" x2="15.01" y2="9"></line>
        </svg>
      </button>

      <input
        type="text"
        class="text-field"
        placeholder={isRecording ? '🎙️ Merekam suara terenkripsi...' : 'Ketik pesan...'}
        bind:value={text}
        onkeydown={(e) => e.key === 'Enter' && submitText()}
      />

      {#if text.trim()}
        <button
          type="button"
          class="send-arrow-btn"
          onclick={submitText}
          aria-label="Kirim Pesan"
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
          </svg>
        </button>
      {/if}
    </div>

    <!-- 3. Circular Mic Button -->
    <button
      type="button"
      class="circle-action-btn {isRecording ? 'recording' : ''}"
      onclick={toggleVoiceRecord}
      title="Pesan Suara"
      aria-label="Pesan Suara"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke={isRecording ? '#E11D48' : '#152641'} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path>
        <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
        <line x1="12" y1="19" x2="12" y2="23"></line>
        <line x1="8" y1="23" x2="16" y2="23"></line>
      </svg>
    </button>
  </div>
</div>

<AttachMediaDialog
  open={showAttachDialog}
  onclose={() => (showAttachDialog = false)}
  onSelectAttachment={(att) => {
    showAttachDialog = false;
    onsend('', att);
  }}
/>


<style>
  .chat-input-dock-wrap {
    padding: 10px 16px calc(14px + env(safe-area-inset-bottom, 0px)) 16px;
    background: transparent;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-sizing: border-box;
  }

  .reply-floating-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 14px;
    background: #FFFFFF;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    box-shadow: 0 2px 8px rgba(21, 38, 65, 0.06);
  }

  .reply-meta {
    display: flex;
    flex-direction: column;
  }

  .reply-label {
    font-size: 0.76rem;
    font-weight: 700;
    color: var(--brand-blue);
  }

  .reply-text-preview {
    font-size: 0.82rem;
    color: var(--text-secondary);
  }

  .close-reply-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 0.9rem;
    padding: 4px;
  }

  .dock-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .circle-action-btn {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: #FFFFFF;
    border: 1px solid var(--border-light);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(21, 38, 65, 0.06);
    flex-shrink: 0;
    transition: transform 0.12s ease;
  }

  .circle-action-btn:active {
    transform: scale(0.92);
  }

  .circle-action-btn.recording {
    background-color: #FEE2E2;
    border-color: #FCA5A5;
    animation: mic-rec-pulse 1s infinite alternate;
  }

  .input-pill-field {
    flex: 1;
    height: 48px;
    border-radius: var(--radius-pill);
    background-color: #F1F5F9;
    display: flex;
    align-items: center;
    padding: 0 12px 0 14px;
    gap: 10px;
    box-sizing: border-box;
  }

  .emoji-trigger-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .text-field {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    font-family: var(--font-sans);
    font-size: 0.98rem;
    color: var(--text-primary);
  }

  .text-field::placeholder {
    color: #9FB0C9;
  }

  .send-arrow-btn {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background-color: var(--brand-blue);
    color: #FFFFFF;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: transform 0.12s ease;
  }

  .send-arrow-btn:active {
    transform: scale(0.9);
  }

  @keyframes mic-rec-pulse {
    0% { transform: scale(1); }
    100% { transform: scale(1.08); }
  }
</style>
