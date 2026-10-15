<script lang="ts">
  import type { MessageModel, EmojiPackModel } from '../types';
  import AttachMediaDialog from './AttachMediaDialog.svelte';
  import EmojiPickerPopover from './EmojiPickerPopover.svelte';
  import {
    Paperclip,
    Smiley,
    Microphone,
    PaperPlaneRight,
    Check,
    X,
    PencilSimple
  } from 'phosphor-svelte';

  interface Props {
    replyingTo?: MessageModel | null;
    editingMessage?: MessageModel | null;
    emojiPacks?: EmojiPackModel[];
    oncancelreply?: () => void;
    oncanceledit?: () => void;
    onsend: (text: string, attachment?: MessageModel['attachment']) => void;
    onsaveedit?: (msgId: string, newText: string) => void;
  }

  let {
    replyingTo = null,
    editingMessage = null,
    emojiPacks = [],
    oncancelreply,
    oncanceledit,
    onsend,
    onsaveedit
  }: Props = $props();

  let text = $state('');
  let isRecording = $state(false);
  let showAttachDialog = $state(false);
  let showEmojiPicker = $state(false);

  // Sync text when editingMessage changes
  $effect(() => {
    if (editingMessage) {
      text = editingMessage.text;
    }
  });

  function submitText() {
    if (!text.trim()) return;

    if (editingMessage) {
      onsaveedit?.(editingMessage.id, text.trim());
      text = '';
      oncanceledit?.();
      showEmojiPicker = false;
      return;
    }

    onsend(text.trim());
    text = '';
    showEmojiPicker = false;
  }

  function handleAttachClick() {
    showAttachDialog = true;
    showEmojiPicker = false;
  }

  function toggleVoiceRecord() {
    showEmojiPicker = false;
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

  function handleSelectEmoji(emoji: string) {
    text += emoji;
  }
</script>

<div class="chat-input-dock-wrap">
  <!-- Emoji Popover anchored above input bar -->
  <div class="emoji-popover-anchor">
    <EmojiPickerPopover
      open={showEmojiPicker}
      onselect={handleSelectEmoji}
      onclose={() => (showEmojiPicker = false)}
    />
  </div>

  <!-- Replying Banner -->
  {#if replyingTo && !editingMessage}
    <div class="reply-floating-bar">
      <div class="reply-meta">
        <span class="reply-label">Membalas {replyingTo.senderName}:</span>
        <span class="reply-text-preview">{replyingTo.text}</span>
      </div>
      <button
        type="button"
        class="close-reply-btn"
        onclick={oncancelreply}
        aria-label="Batalkan Balasan"
        style="display: flex; align-items: center; justify-content: center;"
      >
        <X size={14} weight="bold" />
      </button>
    </div>
  {/if}

  <!-- Editing Banner -->
  {#if editingMessage}
    <div class="edit-floating-bar">
      <div class="edit-meta" style="display: flex; align-items: center; gap: 6px;">
        <PencilSimple size={14} weight="bold" color="var(--brand-blue, #0284C7)" />
        <span class="edit-label">Mengedit Pesan:</span>
        <span class="edit-text-preview">{editingMessage.text}</span>
      </div>
      <button
        type="button"
        class="close-edit-btn"
        onclick={() => {
          text = '';
          oncanceledit?.();
        }}
        aria-label="Batalkan Edit"
        style="display: flex; align-items: center; justify-content: center;"
      >
        <X size={14} weight="bold" />
      </button>
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
      style="display: flex; align-items: center; justify-content: center;"
    >
      <Paperclip size={22} weight="bold" color="var(--brand-blue, #0284C7)" />
    </button>

    <!-- 2. Pill Input Container -->
    <div class="input-pill-field">
      <button
        type="button"
        class="emoji-trigger-btn {showEmojiPicker ? 'active-emoji' : ''}"
        onclick={() => (showEmojiPicker = !showEmojiPicker)}
        title="Buka Papan Emoji Lengkap"
        aria-label="Emoji"
        style="display: flex; align-items: center; justify-content: center;"
      >
        <Smiley size={22} weight="bold" color={showEmojiPicker ? 'var(--brand-blue, #0284C7)' : '#60A5FA'} />
      </button>

      <input
        type="text"
        class="text-field"
        placeholder={isRecording ? 'Merekam suara terenkripsi...' : editingMessage ? 'Edit teks pesan...' : 'Ketik pesan...'}
        bind:value={text}
        onkeydown={(e) => e.key === 'Enter' && submitText()}
      />

      {#if text.trim()}
        {#if editingMessage}
          <!-- Checkmark Save Button for Edit Mode -->
          <button
            type="button"
            class="send-arrow-btn edit-mode-btn"
            onclick={submitText}
            aria-label="Simpan Perubahan"
            title="Simpan Perubahan"
            style="display: flex; align-items: center; justify-content: center;"
          >
            <Check size={18} weight="bold" color="#FFFFFF" />
          </button>
        {:else}
          <!-- Regular Send Button -->
          <button
            type="button"
            class="send-arrow-btn"
            onclick={submitText}
            aria-label="Kirim Pesan"
            style="display: flex; align-items: center; justify-content: center;"
          >
            <PaperPlaneRight size={18} weight="fill" color="#FFFFFF" />
          </button>
        {/if}
      {/if}
    </div>

    <!-- 3. Circular Mic Button -->
    <button
      type="button"
      class="circle-action-btn {isRecording ? 'recording' : ''}"
      onclick={toggleVoiceRecord}
      title="Pesan Suara"
      aria-label="Pesan Suara"
      style="display: flex; align-items: center; justify-content: center;"
    >
      <Microphone size={22} weight="bold" color={isRecording ? '#E11D48' : '#152641'} />
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
    position: relative;
    padding: 10px 16px calc(14px + env(safe-area-inset-bottom, 0px)) 16px;
    background: transparent;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-sizing: border-box;
  }

  .emoji-popover-anchor {
    position: absolute;
    bottom: calc(100% + 4px);
    left: 16px;
    z-index: 100;
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
    animation: slideUp 0.15s ease;
  }

  .edit-floating-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 14px;
    background: #EFF6FF;
    border-radius: var(--radius-pill);
    border: 1px solid #BFDBFE;
    box-shadow: 0 2px 8px rgba(2, 132, 199, 0.08);
    animation: slideUp 0.15s ease;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .reply-meta, .edit-meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
    flex: 1;
  }

  .reply-label {
    font-size: 0.76rem;
    font-weight: 700;
    color: var(--brand-blue);
  }

  .edit-label {
    font-size: 0.76rem;
    font-weight: 700;
    color: var(--brand-blue);
  }

  .reply-text-preview, .edit-text-preview {
    font-size: 0.82rem;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .close-reply-btn, .close-edit-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 0.9rem;
    padding: 4px;
    margin-left: 8px;
  }

  .dock-row {
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;
    z-index: 110;
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
    transition: transform 0.15s ease;
  }

  .emoji-trigger-btn:hover {
    transform: scale(1.15);
  }

  .emoji-trigger-btn.active-emoji {
    transform: scale(1.2);
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
    transition: transform 0.12s ease, background-color 0.15s ease;
  }

  .send-arrow-btn:active {
    transform: scale(0.9);
  }

  .edit-mode-btn {
    background-color: #10B981;
  }

  .edit-mode-btn:hover {
    background-color: #059669;
  }

  @keyframes mic-rec-pulse {
    0% { transform: scale(1); }
    100% { transform: scale(1.08); }
  }
</style>
