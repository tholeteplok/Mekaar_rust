<script lang="ts">
  import type { MessageModel } from '../types';

  interface Props {
    message: MessageModel;
    onreply?: (msg: MessageModel) => void;
    onreact?: (msgId: string, emoji: string) => void;
  }

  let { message, onreply, onreact }: Props = $props();
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

    {#if message.attachment}
      <div class="attachment-box {message.isSelf ? 'att-self' : 'att-peer'}">
        <span class="att-icon">
          {#if message.attachment.kind === 'image'}🖼️{:else if message.attachment.kind === 'voice'}🎙️{:else if message.attachment.kind === 'location'}📍{:else}📄{/if}
        </span>
        <div class="att-info">
          <span class="att-name">{message.attachment.name}</span>
          <span class="att-size">{message.attachment.sizeLabel}</span>
        </div>
      </div>
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
    max-width: min(78%, 460px);
    padding: 10px 14px 6px 14px;
    box-shadow: 0 1px 4px rgba(21, 38, 65, 0.05);
    word-break: break-word;
    box-sizing: border-box;
  }

  /* Outgoing: Vibrant Blue #136CFC, white text */
  .bubble-self {
    background-color: var(--brand-blue);
    color: #FFFFFF;
    border-radius: 18px 18px 4px 18px;
  }

  /* Incoming: Clean White, subtle border, dark navy text */
  .bubble-peer {
    background-color: #FFFFFF;
    color: var(--text-primary);
    border: 1px solid var(--border-light);
    border-radius: 18px 18px 18px 4px;
  }

  .sender-name {
    font-size: 0.78rem;
    font-weight: 700;
    color: var(--brand-blue);
    margin-bottom: 3px;
  }

  .message-body {
    font-size: 0.96rem;
    line-height: 1.4;
    white-space: pre-wrap;
  }

  .reply-box {
    padding: 6px 10px;
    border-radius: 8px;
    margin-bottom: 6px;
    font-size: 0.8rem;
    display: flex;
    flex-direction: column;
  }

  .reply-self {
    background-color: rgba(255, 255, 255, 0.18);
    border-left: 3px solid #FFFFFF;
  }

  .reply-peer {
    background-color: #F1F5F9;
    border-left: 3px solid var(--brand-blue);
  }

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
    border: 1px solid var(--border-subtle);
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
    font-size: 0.75rem;
    opacity: 0.8;
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
    color: var(--text-secondary);
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
