<script lang="ts">
  import type { MessageModel, EmojiPackModel } from '../types';

  interface Props {
    replyingTo: MessageModel | null;
    emojiPacks: EmojiPackModel[];
    oncancelreply: () => void;
    onsend: (text: string, attachment?: MessageModel['attachment']) => void;
    onopenemojistore: () => void;
  }

  let {
    replyingTo,
    emojiPacks,
    oncancelreply,
    onsend,
    onopenemojistore
  }: Props = $props();

  let text = $state('');
  let showStickers = $state(false);
  let recordingVoice = $state(false);

  function submitText() {
    if (!text.trim()) return;
    onsend(text);
    text = '';
  }

  function sendQuickAttachment(kind: 'image' | 'document' | 'voice' | 'location') {
    const map = {
      image: { name: 'Foto_Lapangan_E2EE.jpg', sizeLabel: '420 KB • Terenkripsi' },
      document: { name: 'Laporan_Tim_Mekaar.pdf', sizeLabel: '1.1 MB • BLAKE3 Verified' },
      voice: { name: 'Pesan_Suara_00-14.ogg', sizeLabel: '0:14 • Opus E2EE' },
      location: { name: 'Titik Koordinat Langsung (-7.5568, 110.8318)', sizeLabel: 'Akurasi 5m' }
    };
    onsend('', {
      id: `att-${Date.now()}`,
      kind,
      name: map[kind].name,
      sizeLabel: map[kind].sizeLabel,
      urlOrPreview: kind
    });
  }
</script>

<div class="input-wrapper">
  {#if replyingTo}
    <div class="reply-banner sketch-card">
      <div>
        <span style="font-weight: 700; font-size: 0.8rem;">Membalas {replyingTo.senderName}:</span>
        <div style="font-size: 0.82rem; color: var(--text-secondary);">{replyingTo.text}</div>
      </div>
      <button type="button" class="clear-btn" onclick={oncancelreply}>✕</button>
    </div>
  {/if}

  {#if showStickers}
    <div class="sticker-tray sketch-card">
      <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;">
        <span style="font-weight: 700; font-size: 0.84rem;">Paket Stiker & Emoji Aktif</span>
        <button type="button" class="store-link" onclick={onopenemojistore}>+ Toko Emoji</button>
      </div>
      <div style="display: flex; flex-wrap: wrap; gap: 8px;">
        {#each emojiPacks.filter((p) => p.installed) as pack}
          {#each pack.stickers as sticker}
            <button
              type="button"
              class="sticker-item"
              onclick={() => {
                onsend(sticker);
                showStickers = false;
              }}
            >
              {sticker}
            </button>
          {/each}
        {/each}
      </div>
    </div>
  {/if}

  <div class="input-bar sketch-card">
    <button
      type="button"
      class="icon-btn"
      title="Stiker & Emoji"
      onclick={() => (showStickers = !showStickers)}
    >
      😊
    </button>
    <button
      type="button"
      class="icon-btn"
      title="Kirim Gambar Terenkripsi"
      onclick={() => sendQuickAttachment('image')}
    >
      🖼️
    </button>
    <button
      type="button"
      class="icon-btn"
      title="Kirim Dokumen"
      onclick={() => sendQuickAttachment('document')}
    >
      📎
    </button>
    <button
      type="button"
      class="icon-btn"
      title="Bagikan Titik Lokasi"
      onclick={() => sendQuickAttachment('location')}
    >
      📍
    </button>

    <input
      type="text"
      class="chat-field"
      placeholder={recordingVoice ? '🎙️ Merekam pesan suara terenkripsi...' : 'Tulis pesan terenkripsi E2EE...'}
      bind:value={text}
      onkeydown={(e) => e.key === 'Enter' && submitText()}
    />

    {#if text.trim()}
      <button type="button" class="send-btn" onclick={submitText}>Kirim ➤</button>
    {:else}
      <button
        type="button"
        class="icon-btn {recordingVoice ? 'rec-active' : ''}"
        title="Pesan Suara"
        onclick={() => {
          if (recordingVoice) {
            recordingVoice = false;
            sendQuickAttachment('voice');
          } else {
            recordingVoice = true;
          }
        }}
      >
        {recordingVoice ? '⏹️' : '🎙️'}
      </button>
    {/if}
  </div>
</div>

<style>
  .input-wrapper {
    padding: 10px 14px;
    background: var(--register-header);
    border-top: 2px solid var(--sketch-border-color);
  }
  .reply-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    margin-bottom: 8px;
    background: var(--bg-secondary);
  }
  .sticker-tray {
    padding: 10px 12px;
    margin-bottom: 8px;
  }
  .store-link {
    background: var(--accent-secondary);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 8px;
    padding: 3px 8px;
    font-size: 0.78rem;
    font-weight: 700;
    cursor: pointer;
  }
  .sticker-item {
    font-size: 1.4rem;
    background: var(--bg-secondary);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 10px;
    padding: 4px 8px;
    cursor: pointer;
  }
  .input-bar {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
  }
  .icon-btn {
    background: var(--bg-secondary);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 10px;
    width: 34px;
    height: 34px;
    cursor: pointer;
    font-size: 0.95rem;
    flex-shrink: 0;
  }
  .icon-btn.rec-active {
    background: #FEE2E2;
    border-color: #E11D48;
  }
  .chat-field {
    flex: 1;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 0.95rem;
    outline: none;
    padding: 4px 6px;
  }
  .send-btn {
    background: var(--accent-primary);
    color: #FFFFFF;
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 10px;
    padding: 6px 14px;
    font-weight: 700;
    cursor: pointer;
  }
  .clear-btn {
    background: none;
    border: none;
    font-weight: 700;
    cursor: pointer;
  }
</style>
