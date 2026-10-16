<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { VaultMediaItemModel } from '../types';
  import Dialog from './Dialog.svelte';

  let {
    open = false,
    onclose,
    onOpenStegoDecoder
  }: {
    open: boolean;
    onclose: () => void;
    onOpenStegoDecoder?: (imgSrc: string, name: string) => void;
  } = $props();

  let filterTab = $state<'all' | 'image' | 'stego' | 'document' | 'voice'>('all');
  let searchQuery = $state('');
  let toastMsg = $state('');
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  function showToast(msg: string) {
    toastMsg = msg;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toastMsg = '';
    }, 2500);
  }

  const filteredItems = $derived(
    mekaarStore.vaultMediaItems.filter((item) => {
      if (filterTab === 'image' && item.kind !== 'image') return false;
      if (filterTab === 'stego' && !item.isStego) return false;
      if (filterTab === 'document' && item.kind !== 'document') return false;
      if (filterTab === 'voice' && item.kind !== 'voice') return false;

      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return (
        item.name.toLowerCase().includes(q) ||
        item.conversationTitle.toLowerCase().includes(q)
      );
    })
  );

  function handleDownload(item: VaultMediaItemModel) {
    showToast(`Mengekspor ${item.name} ke berkas lokal...`);
  }

  function handleShred(item: VaultMediaItemModel) {
    mekaarStore.deleteVaultMediaItem(item.id);
    showToast(`Berkas ${item.name} berhasil dihapus permanen (shredded).`);
  }

  function handleExtractStego(item: VaultMediaItemModel) {
    if (onOpenStegoDecoder) {
      onOpenStegoDecoder(item.urlOrPreview, item.name);
    }
  }
</script>

<Dialog
  {open}
  title="Brankas Media & Berkas Terenkripsi"
  {onclose}
  maxWidth="560px"
>
  <div class="vault-dialog-content">
    <p class="dialog-sub">Penyimpanan lokal terlindungi enkripsi SQLCipher</p>

    <!-- Toast Feedback -->
    {#if toastMsg}
      <div class="vault-toast-pill">
        ✓ {toastMsg}
      </div>
    {/if}

    <!-- Search & Filter Bar -->
    <div class="filter-controls-wrap">
      <input
        type="text"
        class="vault-search-input"
        placeholder="Cari nama berkas atau nama percakapan..."
        bind:value={searchQuery}
      />

      <div class="category-tabs-bar">
        <button
          type="button"
          class="cat-tab-btn {filterTab === 'all' ? 'active' : ''}"
          onclick={() => (filterTab = 'all')}
        >
          Semua ({mekaarStore.vaultMediaItems.length})
        </button>
        <button
          type="button"
          class="cat-tab-btn {filterTab === 'image' ? 'active' : ''}"
          onclick={() => (filterTab = 'image')}
        >
          🖼️ Foto
        </button>
        <button
          type="button"
          class="cat-tab-btn {filterTab === 'stego' ? 'active' : ''}"
          onclick={() => (filterTab = 'stego')}
        >
          🕵️ Stego Photo
        </button>
        <button
          type="button"
          class="cat-tab-btn {filterTab === 'document' ? 'active' : ''}"
          onclick={() => (filterTab = 'document')}
        >
          📄 Dokumen
        </button>
        <button
          type="button"
          class="cat-tab-btn {filterTab === 'voice' ? 'active' : ''}"
          onclick={() => (filterTab = 'voice')}
        >
          🎙️ Suara
        </button>
      </div>
    </div>

    <!-- Scrollable Media Items List -->
    <div class="vault-items-scroll">
      {#if filteredItems.length === 0}
        <div class="empty-vault-view">
          <span class="empty-vault-icon">📂</span>
          <h4>Tidak Ada Berkas</h4>
          <p>Belum ada media terenkripsi dalam kategori ini.</p>
        </div>
      {:else}
        <div class="vault-items-list">
          {#each filteredItems as item (item.id)}
            <div class="media-card">
              <!-- Icon / Thumbnail -->
              <div class="media-preview-icon">
                {#if item.isStego}
                  🕵️
                {:else if item.kind === 'image'}
                  🖼️
                {:else if item.kind === 'voice'}
                  🎙️
                {:else}
                  📄
                {/if}
              </div>

              <!-- Info -->
              <div class="media-info-col">
                <div class="media-name-row">
                  <span class="media-name">{item.name}</span>
                  {#if item.isStego}
                    <span class="stego-pill-tag">Stego Photo</span>
                  {/if}
                </div>
                <span class="media-conv-origin">Obrolan: <b>{item.conversationTitle}</b></span>
                <span class="media-meta-line">{item.sizeLabel} • {item.timestamp}</span>
                <code class="media-checksum-tag">{item.checksum}</code>
              </div>

              <!-- Action Buttons -->
              <div class="media-actions-col">
                {#if item.isStego && onOpenStegoDecoder}
                  <button
                    type="button"
                    class="btn-action-decode"
                    title="Buka Pesan di Balik Foto (Stego Photo)"
                    onclick={() => handleExtractStego(item)}
                  >
                    Buka Pesan
                  </button>
                {/if}
                <button
                  type="button"
                  class="btn-action-download"
                  title="Buka / Ekspor Berkas"
                  onclick={() => handleDownload(item)}
                >
                  Ekspor
                </button>
                <button
                  type="button"
                  class="btn-action-shred"
                  title="Hapus Permanen (Shred)"
                  onclick={() => handleShred(item)}
                >
                  🗑️
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Security Notice -->
    <div class="vault-security-footer">
      🔒 Semua berkas disimpan terenkripsi menggunakan SQLCipher di ruang penyimpanan lokal perangkat.
    </div>
  </div>
</Dialog>

<style>
  .vault-dialog-content {
    display: flex;
    flex-direction: column;
    max-height: 75vh;
  }

  .dialog-sub {
    font-size: 0.76rem;
    color: var(--text-secondary);
    margin: -6px 0 12px 0;
  }

  .vault-toast-pill {
    background-color: var(--brand-teal, #0D9488);
    color: #FFFFFF;
    font-size: 0.78rem;
    font-weight: 700;
    font-family: var(--font-active);
    padding: 6px 14px;
    border-radius: var(--radius-pill);
    margin-bottom: 10px;
    align-self: center;
  }

  .filter-controls-wrap {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 12px;
  }

  .vault-search-input {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    padding: 8px 14px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    background-color: var(--bg-card);
    font-size: 0.84rem;
    font-family: var(--font-active);
    color: var(--text-primary);
    outline: none;
    box-sizing: border-box;
  }

  .vault-search-input:focus {
    border-color: var(--brand-blue);
  }

  .category-tabs-bar {
    display: flex;
    background-color: var(--bg-primary);
    padding: 4px;
    border-radius: var(--radius-pill);
    gap: 4px;
    overflow-x: auto;
  }

  .cat-tab-btn {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    white-space: nowrap;
    padding: 5px 12px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    font-size: 0.74rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .cat-tab-btn.active {
    background-color: var(--bg-card);
    color: var(--brand-blue);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  }

  /* Items Scroll */
  .vault-items-scroll {
    overflow-y: auto;
    max-height: 420px;
    padding-right: 4px;
  }

  .empty-vault-view {
    padding: 44px 16px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .empty-vault-icon {
    font-size: 2.5rem;
    margin-bottom: 8px;
  }

  .empty-vault-view h4 {
    font-size: 0.95rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary);
    margin: 0 0 4px 0;
  }

  .empty-vault-view p {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin: 0;
  }

  .vault-items-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .media-card {
    background-color: var(--bg-card);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-md);
    padding: 12px;
    display: flex;
    align-items: center;
    gap: 12px;
    transition: all 0.15s ease;
  }

  .media-card:hover {
    border-color: var(--brand-blue);
  }

  .media-preview-icon {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-md);
    background-color: var(--brand-blue-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.4rem;
    flex-shrink: 0;
  }

  .media-info-col {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .media-name-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .media-name {
    font-size: 0.86rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .stego-pill-tag {
    background-color: var(--brand-teal-light, rgba(13, 148, 136, 0.12));
    color: var(--brand-teal, #0D9488);
    font-size: 0.65rem;
    font-weight: 800;
    padding: 1px 6px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .media-conv-origin {
    font-size: 0.74rem;
    color: var(--text-secondary);
  }

  .media-meta-line {
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .media-checksum-tag {
    font-size: 0.65rem;
    color: var(--brand-blue);
    font-family: var(--font-mono);
  }

  .media-actions-col {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .btn-action-decode {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-teal, #0D9488);
    color: #FFFFFF;
    font-size: 0.74rem;
    font-weight: 700;
    font-family: var(--font-active);
    border: none;
    cursor: pointer;
  }

  .btn-action-download {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--brand-blue);
    font-size: 0.74rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
  }

  .btn-action-download:hover {
    background-color: var(--brand-blue-subtle);
  }

  .btn-action-shred {
    min-height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 4px 8px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #DC2626;
    font-size: 0.85rem;
    cursor: pointer;
  }

  .btn-action-shred:hover {
    background-color: rgba(239, 68, 68, 0.08);
  }

  .vault-security-footer {
    font-size: 0.72rem;
    color: var(--text-secondary);
    text-align: center;
    background-color: var(--bg-primary);
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    margin-top: 12px;
  }
</style>
