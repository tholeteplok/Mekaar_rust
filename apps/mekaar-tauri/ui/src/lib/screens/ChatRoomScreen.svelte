<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { chatStore } from '../stores';
  import { AppBar, Button } from '../components';

  interface Props {
    roomId: string;
    onback: () => void;
    onopensos: () => void;
  }

  let { roomId, onback, onopensos }: Props = $props();

  let messageText = $state('');
  let messagesContainer: HTMLDivElement | null = $state(null);

  const timers = [
    { label: 'Off', val: null },
    { label: '10d', val: 10 },
    { label: '30d', val: 30 },
    { label: '1m', val: 60 },
    { label: '5m', val: 300 },
  ];

  let currentTime = $state(Date.now());
  let tickerId: ReturnType<typeof setInterval> | null = null;

  onMount(async () => {
    await chatStore.selectRoom(roomId);
    scrollToBottom();

    tickerId = setInterval(() => {
      currentTime = Date.now();
    }, 1000);
  });

  onDestroy(() => {
    if (tickerId) clearInterval(tickerId);
  });

  async function scrollToBottom() {
    await tick();
    if (messagesContainer) {
      messagesContainer.scrollTop = messagesContainer.scrollHeight;
    }
  }

  async function handleSend() {
    if (!messageText.trim()) return;
    const text = messageText;
    messageText = '';
    const success = await chatStore.sendMessage(text);
    if (success) {
      scrollToBottom();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  function getRemainingSecs(timestamp: number, disappearingSecs?: number): number | null {
    if (!disappearingSecs) return null;
    const expiry = timestamp + disappearingSecs * 1000;
    return Math.max(0, Math.ceil((expiry - currentTime) / 1000));
  }

  function formatTime(ts: number) {
    const d = new Date(ts);
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
</script>

<div class="chat-room-screen">
  <!-- Top Appbar -->
  <AppBar
    title={chatStore.activeConversation?.peerName ?? 'Ruang Rahasia'}
    subtitle="Enkripsi Double Ratchet X25519 • Aktif"
    showBack
    {onback}
  >
    {#snippet actions()}
      <Button variant="danger" size="sm" onclick={onopensos}>
        🚨 SOS
      </Button>
    {/snippet}
  </AppBar>

  <!-- Disappearing Message Timer Selector Strip -->
  <div class="timer-selector-strip">
    <div class="timer-label">
      <span class="fire-icon">⏳</span>
      <span>Pesan Hilang:</span>
    </div>
    <div class="timer-chips">
      {#each timers as t}
        <button
          type="button"
          class="timer-chip"
          class:active={chatStore.disappearingTimerSecs === t.val}
          onclick={() => chatStore.setDisappearingTimer(t.val)}
        >
          {t.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- Messages Scroll Area -->
  <div class="messages-viewport" bind:this={messagesContainer}>
    <div class="messages-inner">
      <!-- Encryption Security Banner -->
      <div class="encryption-banner">
        🔒 Pesan di ruangan ini dilindungi enkripsi Double Ratchet & tersimpan dalam database SQLCipher lokal.
      </div>

      {#each chatStore.activeMessages as msg (msg.id)}
        {@const remaining = getRemainingSecs(msg.timestamp, msg.disappearingSecs)}
        {#if remaining === null || remaining > 0}
          <div class="message-row" class:is-me={msg.isMe}>
            <div class="message-bubble" class:is-me={msg.isMe} class:burning={remaining !== null && remaining <= 5}>
              <p class="message-text">{msg.content}</p>
              <div class="message-meta">
                {#if remaining !== null}
                  <span class="burn-badge" class:critical={remaining <= 5}>
                    🔥 {remaining}s
                  </span>
                {/if}
                <span class="time-text">{formatTime(msg.timestamp)}</span>
                {#if msg.isMe}
                  <span class="status-check">✓✓</span>
                {/if}
              </div>
            </div>
          </div>
        {:else}
          <div class="message-burned-notice">
            🔥 <em>Pesan rahasia telah musnah secara otomatis</em>
          </div>
        {/if}
      {/each}
    </div>
  </div>

  <!-- Bottom Input Bar -->
  <footer class="chat-input-bar">
    <div class="input-container">
      <input
        type="text"
        class="chat-text-input"
        placeholder="Ketik pesan terenkripsi..."
        bind:value={messageText}
        onkeydown={handleKeydown}
      />
      <Button
        variant="primary"
        size="md"
        disabled={!messageText.trim()}
        onclick={handleSend}
      >
        Kirim
      </Button>
    </div>
  </footer>
</div>

<style>
  .chat-room-screen {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: var(--surface-canvas);
  }

  .timer-selector-strip {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) var(--space-4);
    background-color: var(--surface-card);
    border-bottom: 1px solid var(--border-subtle);
    gap: var(--space-2);
    overflow-x: auto;
  }

  .timer-label {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: var(--font-caption-size);
    font-weight: 600;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .timer-chips {
    display: flex;
    gap: var(--space-1);
  }

  .timer-chip {
    padding: 3px 10px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background: transparent;
    color: var(--text-secondary);
    font-size: var(--font-caption-size);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--motion-fast);
  }

  .timer-chip:hover {
    border-color: var(--brand-blue);
    color: var(--text-primary);
  }

  .timer-chip.active {
    background-color: var(--brand-blue);
    border-color: var(--brand-blue);
    color: #FFFFFF;
  }

  .messages-viewport {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4);
  }

  .messages-inner {
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .encryption-banner {
    text-align: center;
    font-size: var(--font-caption-size);
    color: var(--text-secondary);
    background-color: var(--surface-subtle);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-2);
  }

  .message-row {
    display: flex;
    justify-content: flex-start;
  }

  .message-row.is-me {
    justify-content: flex-end;
  }

  .message-bubble {
    max-width: 75%;
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    background-color: var(--surface-card);
    border: 1px solid var(--border-subtle);
    color: var(--text-primary);
    box-shadow: var(--shadow-soft);
    display: flex;
    flex-direction: column;
    gap: 4px;
    word-break: break-word;
  }

  .message-bubble.is-me {
    background-color: var(--brand-blue);
    color: var(--text-on-blue);
    border-color: var(--brand-blue);
    border-bottom-right-radius: 4px;
  }

  .message-bubble:not(.is-me) {
    border-bottom-left-radius: 4px;
  }

  .message-text {
    font-size: var(--font-body-size);
    line-height: var(--font-body-line);
    margin: 0;
  }

  .message-meta {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 6px;
    font-size: var(--font-caption-size);
    opacity: 0.8;
  }

  .burn-badge {
    background-color: rgba(255, 93, 93, 0.25);
    color: inherit;
    padding: 1px 6px;
    border-radius: var(--radius-pill);
    font-weight: 700;
  }

  .burn-badge.critical {
    background-color: var(--sos-coral);
    color: #FFFFFF;
    animation: burnPulse 500ms infinite alternate;
  }

  .message-bubble.burning {
    border-color: var(--sos-coral);
    animation: burnFlicker 800ms infinite alternate;
  }

  .message-burned-notice {
    text-align: center;
    font-size: var(--font-caption-size);
    color: var(--sos-coral);
    padding: var(--space-1) var(--space-3);
    opacity: 0.8;
  }

  @keyframes burnFlicker {
    0% { box-shadow: 0 0 4px rgba(var(--sos-coral-rgb), 0.2); }
    100% { box-shadow: 0 0 12px rgba(var(--sos-coral-rgb), 0.5); }
  }

  @keyframes burnPulse {
    0% { transform: scale(1); }
    100% { transform: scale(1.08); }
  }

  .time-text {
    font-size: 10.5px;
  }

  .status-check {
    font-size: 11px;
    font-weight: 800;
  }

  .chat-input-bar {
    position: sticky;
    bottom: 0;
    background-color: var(--surface-card);
    border-top: 1px solid var(--border-subtle);
    padding: var(--space-3) var(--space-4);
    z-index: 20;
  }

  .input-container {
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .chat-text-input {
    flex: 1;
    height: 48px;
    padding: 0 var(--space-4);
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background-color: var(--surface-canvas);
    color: var(--text-primary);
    font-family: var(--font-family);
    font-size: var(--font-body-size);
    outline: none;
    transition: border-color var(--motion-fast);
  }

  .chat-text-input:focus {
    border-color: var(--border-focus);
  }
</style>
