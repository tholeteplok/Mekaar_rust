<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';

  let radarFilter = $state<'semua' | 'kontak'>('semua');
  let sortMode = $state<'az' | 'recent'>('az');
  let searchQuery = $state('');
  let isSearchActive = $state(false);
  let showQrDialog = $state(false);
  let showAddContactDialog = $state(false);
  let newContactUsername = $state('');
  let newContactDisplayName = $state('');

  const sortedContacts = $derived(
    [...mekaarStore.contacts]
      .filter((c) => {
        if (!searchQuery) return true;
        return (
          c.displayName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          c.username.toLowerCase().includes(searchQuery.toLowerCase())
        );
      })
      .sort((a, b) => {
        if (sortMode === 'az') {
          return a.displayName.localeCompare(b.displayName);
        }
        return 0;
      })
  );

  function handleStartChat(contact: typeof mekaarStore.contacts[0]) {
    // Check if conversation exists, or create one
    let conv = mekaarStore.conversations.find((c) => c.title === contact.displayName);
    if (!conv) {
      const newId = `conv-${Date.now()}`;
      mekaarStore.conversations = [
        {
          id: newId,
          title: contact.displayName,
          subtitle: '🔒 Sesi E2EE',
          lastTimestamp: 'Sekarang',
          avatarColor: contact.avatarColor || '#EFF6FF',
          unreadCount: 0,
          isPinned: false,
          isGroup: false,
          isOnline: contact.isOnline,
          isGuardian: contact.isGuardian,
          disappearingSeconds: 0
        },
        ...mekaarStore.conversations
      ];
      mekaarStore.openConversation(newId);
    } else {
      mekaarStore.openConversation(conv.id);
    }
  }

  function handleAddContact() {
    if (!newContactUsername.trim()) return;
    const username = newContactUsername.startsWith('@') ? newContactUsername.trim() : `@${newContactUsername.trim()}`;
    const displayName = newContactDisplayName.trim() || username.replace('@', '');
    mekaarStore.contacts = [
      {
        id: `usr-${Date.now()}`,
        username,
        displayName,
        avatarColor: '#EFF6FF',
        fingerprint: 'ED25519: VERIFIED-AEGIS',
        isOnline: true,
        isGuardian: false,
        isBlocked: false
      },
      ...mekaarStore.contacts
    ];
    newContactUsername = '';
    newContactDisplayName = '';
    showAddContactDialog = false;
  }
</script>

<AppBar title="Kontak">
  {#snippet actions()}
    <!-- QR Code Scan / Share Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (showQrDialog = true)}
      title="Kode QR"
      aria-label="Kode QR"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="3" y="3" width="7" height="7"></rect>
        <rect x="14" y="3" width="7" height="7"></rect>
        <rect x="14" y="14" width="7" height="7"></rect>
        <rect x="3" y="14" width="7" height="7"></rect>
      </svg>
    </button>

    <!-- Search Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (isSearchActive = !isSearchActive)}
      title="Cari Kontak"
      aria-label="Cari Kontak"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
    </button>
  {/snippet}
</AppBar>

<div class="contacts-page-container">
  {#if isSearchActive}
    <div class="search-bar-wrap">
      <input
        type="text"
        class="search-input"
        placeholder="Cari kontak berdasarkan nama atau @username..."
        bind:value={searchQuery}
      />
    </div>
  {/if}

  <!-- Section 1: Teman Sekitar (Proximity Radar) -->
  <div class="section-radar">
    <div class="radar-header-row">
      <div class="radar-title-wrap">
        <span class="radar-icon">🌀</span>
        <span class="radar-title">Teman Sekitar</span>
      </div>
      <div class="radar-segment">
        <button
          type="button"
          class="radar-seg-btn {radarFilter === 'semua' ? 'active' : ''}"
          onclick={() => (radarFilter = 'semua')}
        >
          Semua
        </button>
        <button
          type="button"
          class="radar-seg-btn {radarFilter === 'kontak' ? 'active' : ''}"
          onclick={() => (radarFilter = 'kontak')}
        >
          Kontak
        </button>
      </div>
      <button
        type="button"
        class="radar-refresh-btn"
        title="Pindai Ulang Radar"
        aria-label="Pindai Ulang"
      >
        🔄
      </button>
    </div>

    <!-- Empty State Teman Sekitar -->
    <div class="radar-empty-box">
      <div class="radar-avatar-placeholder">
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#9FB0C9" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="7" r="4"></circle>
        </svg>
      </div>
      <p class="radar-empty-title">Belum ada teman di sekitar</p>
      <p class="radar-empty-desc">Jarak diperbarui otomatis saat aktif di radius &lt; 10 km</p>
    </div>
  </div>

  <!-- Section 2: Kontak Card with Corner Ticks -->
  <div class="contacts-card-wrap">
    <Card withTicks={true} padding="16px">
      <!-- Card Toolbar Header -->
      <div class="card-toolbar-row">
        <div class="card-toolbar-left">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="var(--brand-blue)">
            <path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"/>
          </svg>
          <span class="toolbar-title">Kontak</span>
          <span class="count-pill">{mekaarStore.contacts.length}</span>
        </div>

        <div class="card-toolbar-center">
          <div class="sort-segment">
            <button
              type="button"
              class="sort-btn {sortMode === 'az' ? 'active' : ''}"
              onclick={() => (sortMode = 'az')}
            >
              A-Z
            </button>
            <button
              type="button"
              class="sort-btn {sortMode === 'recent' ? 'active' : ''}"
              onclick={() => (sortMode = 'recent')}
            >
              Baru Ditam...
            </button>
          </div>
        </div>

        <button
          type="button"
          class="toolbar-more-btn"
          onclick={() => (showAddContactDialog = true)}
          title="Tambah Kontak"
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="1"></circle>
            <circle cx="19" cy="12" r="1"></circle>
            <circle cx="5" cy="12" r="1"></circle>
          </svg>
        </button>
      </div>

      <div class="divider-line"></div>

      <!-- Contacts List in Card -->
      <div class="contacts-items-list">
        {#each sortedContacts as contact (contact.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="contact-item-row"
            onclick={() => handleStartChat(contact)}
          >
            <SketchAvatar
              name={contact.displayName}
              size={52}
              online={contact.isOnline}
              isGuardian={contact.isGuardian}
            />
            <div class="contact-item-info">
              <span class="contact-item-name">{contact.displayName}</span>
              <span class="contact-item-handle">{contact.username}</span>
            </div>
            <button
              type="button"
              class="contact-info-btn"
              onclick={(e) => {
                e.stopPropagation();
                alert(`Info Kontak: ${contact.displayName}\nUsername: ${contact.username}\nFingerprint: ${contact.fingerprint}`);
              }}
              aria-label="Info Kontak"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--text-secondary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" y1="16" x2="12" y2="12"></line>
                <line x1="12" y1="8" x2="12.01" y2="8"></line>
              </svg>
            </button>
          </div>
        {:else}
          <div style="padding: 24px; text-align: center; color: var(--text-secondary); font-size: 0.9rem;">
            Belum ada kontak terdaftar.
          </div>
        {/each}
      </div>
    </Card>
  </div>
</div>

<!-- Modal Tambah Kontak -->
<Dialog
  open={showAddContactDialog}
  title="Tambah Kontak Baru"
  onclose={() => (showAddContactDialog = false)}
>
  <div style="display: flex; flex-direction: column; gap: 14px;">
    <label style="font-size: 0.85rem; font-weight: 600;">
      Username Teman:
      <input
        type="text"
        class="sketch-input"
        placeholder="@teplok"
        bind:value={newContactUsername}
        style="margin-top: 4px;"
      />
    </label>
    <label style="font-size: 0.85rem; font-weight: 600;">
      Nama Tampilan (Opsional):
      <input
        type="text"
        class="sketch-input"
        placeholder="Nama Panggilan"
        bind:value={newContactDisplayName}
        style="margin-top: 4px;"
      />
    </label>
    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
      <button type="button" class="btn-secondary" onclick={() => (showAddContactDialog = false)}>
        Batal
      </button>
      <button type="button" class="btn-primary" onclick={handleAddContact}>
        Simpan Kontak
      </button>
    </div>
  </div>
</Dialog>

<!-- Modal Kode QR -->
<Dialog
  open={showQrDialog}
  title="Kode QR Identitas Saya"
  onclose={() => (showQrDialog = false)}
>
  <div style="text-align: center; padding: 12px 0;">
    <div style="width: 180px; height: 180px; margin: 0 auto 16px auto; background: #FFFFFF; border: 1px solid var(--border-subtle); border-radius: 16px; display: flex; align-items: center; justify-content: center;">
      <!-- QR Placeholder SVG -->
      <svg width="140" height="140" viewBox="0 0 24 24" fill="var(--text-primary)">
        <path d="M2 2h8v8H2V2zm2 2v4h4V4H4zm8-2h8v8h-8V2zm2 2v4h4V4h-4zM2 14h8v8H2v-8zm2 2v4h4v-4H4zm11 0h2v2h-2v-2zm3 0h2v2h-2v-2zm-3 3h2v2h-2v-2zm3 0h2v2h-2v-2zm0-5h2v2h-2v-2zm-5 0h2v2h-2v-2zm2 2h2v2h-2v-2z"/>
      </svg>
    </div>
    <p style="font-weight: 700; font-size: 1.05rem; color: var(--text-primary);">
      {mekaarStore.currentUser.displayName}
    </p>
    <p style="font-size: 0.85rem; color: var(--brand-blue); margin-top: 2px;">
      {mekaarStore.currentUser.username}
    </p>
    <p style="font-size: 0.78rem; color: var(--text-secondary); margin-top: 8px;">
      Pindai untuk verifikasi kunci E2EE secara langsung tanpa server relay.
    </p>
  </div>
</Dialog>

<style>
  .contacts-page-container {
    padding: 0 16px 20px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .icon-action-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }

  .icon-action-btn:active {
    background-color: var(--brand-blue-subtle);
  }

  .search-bar-wrap {
    margin-bottom: 2px;
  }

  .search-input {
    width: 100%;
    padding: 10px 16px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background-color: #FFFFFF;
    font-size: 0.9rem;
    color: var(--text-primary);
    box-sizing: border-box;
    outline: none;
  }

  .search-input:focus {
    border-color: var(--brand-blue);
  }

  /* Section 1: Teman Sekitar */
  .section-radar {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .radar-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .radar-title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .radar-icon {
    font-size: 1.1rem;
  }

  .radar-title {
    font-size: 0.98rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .radar-segment {
    display: flex;
    background-color: #F1F5F9;
    padding: 3px;
    border-radius: var(--radius-pill);
    gap: 2px;
  }

  .radar-seg-btn {
    padding: 5px 14px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    font-size: 0.84rem;
    font-weight: 600;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .radar-seg-btn.active {
    background-color: #2DD4BF;
    color: #FFFFFF;
    box-shadow: 0 1px 4px rgba(45, 212, 191, 0.3);
  }

  .radar-refresh-btn {
    background: transparent;
    border: none;
    font-size: 1rem;
    cursor: pointer;
    padding: 4px;
  }

  .radar-empty-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px 16px;
    text-align: center;
  }

  .radar-avatar-placeholder {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    background-color: #F1F5F9;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
  }

  .radar-empty-title {
    font-size: 0.98rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .radar-empty-desc {
    font-size: 0.82rem;
    color: var(--text-secondary);
    margin-top: 4px;
  }

  /* Section 2: Contacts Card */
  .contacts-card-wrap {
    margin-top: 2px;
  }

  .card-toolbar-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .card-toolbar-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .toolbar-title {
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .count-pill {
    background-color: #EFF6FF;
    color: var(--brand-blue);
    font-size: 0.76rem;
    font-weight: 700;
    padding: 1px 7px;
    border-radius: 999px;
  }

  .sort-segment {
    display: flex;
    background-color: #F1F5F9;
    padding: 2px;
    border-radius: var(--radius-pill);
    gap: 2px;
  }

  .sort-btn {
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .sort-btn.active {
    background-color: var(--brand-blue);
    color: #FFFFFF;
  }

  .toolbar-more-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .divider-line {
    height: 1px;
    background-color: var(--border-light);
    margin: 14px -16px;
  }

  .contacts-items-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .contact-item-row {
    display: flex;
    align-items: center;
    gap: 14px;
    cursor: pointer;
    border-radius: 12px;
    transition: background-color 0.15s ease;
  }

  .contact-item-row:active {
    opacity: 0.85;
  }

  .contact-item-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .contact-item-name {
    font-size: 1.05rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .contact-item-handle {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .contact-info-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
