<script lang="ts">
  import type { MessageModel } from '../types';

  interface Props {
    message: MessageModel;
    onreply: (msg: MessageModel) => void;
    onreact: (msgId: string, emoji: string) => void;
  }

  let { message, onreply, onreact }: Props = $props();
  let showQuickReactions = $state(false);
  const quickEmojis = ['👍', '❤️', '🔥', '🛡️', '🫡', '✨'];
</script>

<div class="bubble-row {message.isSelf ? 'self' : 'peer'}">
  <div class="bubble-box sketch-card">
    {#if !message.isSelf}
      <div class="sender-label">{message.senderName}</div>
    {/if}

    {#if message.replyPreviewText}
      <div class="reply-quote">
        <span style="font-weight: 700;">↩ {message.replySenderName ?? 'Pesan'}:</span>
        <span>{message.replyPreviewText}</span>
      </div>
    {/if}

    {#if message.attachment}
      <div class="attachment-card">
        <span>
          {#if message.attachment.kind === 'image'}🖼️{:else if message.attachment.kind === 'voice'}🎙️{:else if message.attachment.kind === 'location'}📍{:else}📄{/if}
        </span>
        <div>
          <div style="font-weight: 700; font-size: 0.85rem;">{message.attachment.name}</div>
          <div style="font-size: 0.75rem; color: var(--text-secondary);">{message.attachment.sizeLabel}</div>
        </div>
      </div>
    {/if}

    {#if message.text}
      <div class="message-text">{message.text}</div>
    {/if}

    <div class="bubble-meta">
      {#if message.expiresAt}
        <span class="timer-pill" title="Pesan menghilang otomatis">⏱️ {message.expiresAt}</span>
      {/if}
      <span>{message.timestamp}</span>
      {#if message.isSelf}
        <span class="receipt">{message.status === 'read' ? '✓✓' : '✓'}</span>
      {/if}
      <button type="button" class="mini-action" onclick={() => onreply(message)} title="Balas">↩</button>
      <button
        type="button"
        class="mini-action"
        onclick={() => (showQuickReactions = !showQuickReactions)}
        title="Reaksi Emoji"
      >
        😊
      </button>
    </div>

    {#if showQuickReactions}
      <div class="reaction-picker sketch-card">
        {#each quickEmojis as emoji}
          <button
            type="button"
            class="emoji-btn"
            onclick={() => {
              onreact(message.id, emoji);
              showQuickReactions = false;
            }}
          >
            {emoji}
          </button>
        {/each}
      </div>
    {/if}

    {#if Object.keys(message.reactions).length > 0}
      <div class="reactions-bar">
        {#each Object.entries(message.reactions) as [emoji, users]}
          <button type="button" class="reaction-chip" onclick={() => onreact(message.id, emoji)}>
            {emoji} {users.length}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .bubble-row {
    display: flex;
    margin-bottom: 10px;
  }
  .bubble-row.self {
    justify-content: flex-end;
  }
  .bubble-row.peer {
    justify-content: flex-start;
  }
  .bubble-box {
    position: relative;
    max-width: min(78%, 520px);
    padding: 10px 14px;
    background: var(--bubble-peer-bg);
    color: var(--bubble-peer-text);
  }
  .bubble-row.self .bubble-box {
    background: var(--bubble-self-bg);
    color: var(--bubble-self-text);
  }
  .sender-label {
    font-size: 0.78rem;
    font-weight: 700;
    color: var(--accent-primary);
    margin-bottom: 4px;
  }
  .reply-quote {
    font-size: 0.8rem;
    padding: 6px 8px;
    margin-bottom: 6px;
    border-left: 3px solid var(--accent-primary);
    background: rgba(0, 0, 0, 0.05);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
  }
  .attachment-card {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    margin-bottom: 6px;
    background: rgba(255, 255, 255, 0.65);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 10px;
  }
  .message-text {
    font-size: 0.94rem;
    white-space: pre-wrap;
    word-break: break-word;
  }
  .bubble-meta {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 6px;
    margin-top: 4px;
    font-size: 0.72rem;
    opacity: 0.85;
  }
  .timer-pill {
    background: rgba(0, 0, 0, 0.08);
    padding: 1px 6px;
    border-radius: 999px;
    font-weight: 600;
  }
  .receipt {
    font-weight: 800;
    color: var(--accent-primary);
  }
  .mini-action {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.8rem;
    padding: 0 2px;
  }
  .reaction-picker {
    display: flex;
    gap: 4px;
    padding: 4px 8px;
    margin-top: 6px;
  }
  .emoji-btn {
    background: none;
    border: none;
    font-size: 1rem;
    cursor: pointer;
  }
  .reactions-bar {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 6px;
  }
  .reaction-chip {
    background: var(--bg-surface);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 999px;
    padding: 1px 8px;
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
  }
</style>
