<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';
  import MyQrDialog from '../components/MyQrDialog.svelte';
  import ContactQrScanDialog from '../components/ContactQrScanDialog.svelte';
  import ChatRequestsDialog from '../components/ChatRequestsDialog.svelte';

  let radarFilter = $state<'semua' | 'kontak'>('semua');
  let sortMode = $state<'az' | 'recent'>('az');
  let searchQuery = $state('');
  let isSearchActive = $state(false);

  // Dialog states
  let showQrDialog = $state(false);
  let showScanDialog = $state(false);
  let showRequestsDialog = $state(false);
  let showAddContactDialog = $state(false);
  let showContactInfoDialog = $state(false);
  let selectedContact = $state<typeof mekaarStore.contacts[0] | null>(null);

  // Vault states
  let vaultToast = $state('');
  let vaultToastTimer: ReturnType<typeof setTimeout> | null = null;
  let vaultDebounceTimer: ReturnType<typeof setTimeout> | null = null;

  let newContactUsername = $state('');
  let newContactDisplayName = $state('');

  function triggerVaultToast(msg: string) {
    vaultToast = msg;
    if (vaultToastTimer) clearTimeout(vaultToastTimer);
    vaultToastTimer = setTimeout(() => {
      vaultToast = '';
    }, 3500);
  }

  function handleSearchInput(val: string) {
    searchQuery = val;
    if (vaultDebounceTimer) clearTimeout(vaultDebounceTimer);

    // Cek kode rahasia vault saat pengguna mengetik (paritas contact_list_screen.dart)
    if (val.trim().length >= 4) {
      vaultDebounceTimer = setTimeout(() => {
        const isMatch = mekaarStore.verifyVaultPasscode(val);
        if (isMatch) {
          searchQuery = '';
          isSearchActive = false;
          triggerVaultToast('Koleksi Kontak & Obrolan Tersembunyi Dibuka');
        }
      }, 400);
    }
  }

  const sortedContacts = $derived(
    [...mekaarStore.contacts]
      .filter((c) => {
        // Jika vault terkunci, sembunyikan kontak yang ada di conversation isVaultHidden
        const conv = mekaarStore.conversations.find(
          (r) => r.title === c.displayName || r.id === `conv-${c.id}`
        );
        if (!mekaarStore.vaultUnlocked && conv?.isVaultHidden) {
          return false;
        }

        if (!searchQuery) return true;
        const q = searchQuery.toLowerCase();
        return (
          c.displayName.toLowerCase().includes(q) ||
          c.username.toLowerCase().includes(q)
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
    let conv = mekaarStore.conversations.find((c) => c.title === contact.displayName || c.id === `conv-${contact.id}`);
    if (!conv) {
      const newId = `conv-${Date.now()}`;
      mekaarStore.conversations = [
        {
          id: newId,
          title: contact.displayName,
          subtitle: '🔒 Sesi E2EE',
          avatarColor: contact.avatarColor || '#EFF6FF',
          unreadCount: 0,
          lastTimestamp: 'Sekarang',
          isOnline: contact.isOnline,
          isPinned: false,
          isGroup: false,
          isMuted: false,
          isGuardian: contact.isGuardian,
          disappearingSeconds: 0,
          chatTheme: 'classic',
          participants: [
            { userId: mekaarStore.currentUser.id, displayName: mekaarStore.currentUser.displayName, role: 'admin', muted: false },
            { userId: contact.id, displayName: contact.displayName, role: 'member', muted: false }
          ]
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
    const newContact = {
      id: `usr-${Date.now()}`,
      username,
      displayName,
      avatarColor: '#EFF6FF',
      fingerprint: 'ED25519: VERIFIED-AEGIS',
      isOnline: true,
      isGuardian: false,
      isBlocked: false,
      notes: 'Ditambahkan secara manual'
    };
    mekaarStore.contacts = [newContact, ...mekaarStore.contacts];
    newContactUsername = '';
    newContactDisplayName = '';
    showAddContactDialog = false;
  }

  function openContactInfo(contact: typeof mekaarStore.contacts[0]) {
    selectedContact = contact;
    showContactInfoDialog = true;
  }

  function isContactVaultHidden(contact: typeof mekaarStore.contacts[0]): boolean {
    const conv = mekaarStore.conversations.find(
      (r) => r.title === contact.displayName || r.id === `conv-${contact.id}`
    );
    return !!conv?.isVaultHidden;
  }

  function toggleVaultForSelected() {
    if (!selectedContact) return;
    const conv = mekaarStore.conversations.find(
      (r) => r.title === selectedContact!.displayName || r.id === `conv-${selectedContact!.id}`
    );
    if (conv) {
      mekaarStore.toggleVaultHidden(conv.id);
    }
  }

  function toggleBlockForSelected() {
    if (!selectedContact) return;
    mekaarStore.toggleContactBlock(selectedContact.id);
  }
</script>

<AppBar title="Kontak">
  {#snippet actions()}
    <!-- Tombol Notifikasi Permintaan Chat Masuk -->
    {#if mekaarStore.pendingChatRequestsCount > 0}
      <button
        type="button"
        class="req-badge-action-btn"
        onclick={() => (showRequestsDialog = true)}
        title="Permintaan Chat Masuk"
        aria-label="Permintaan Chat Masuk"
      >
        <span class="bell-icon">🔔</span>
        <span class="req-count-pill">{mekaarStore.pendingChatRequestsCount} Permintaan</span>
      </button>
    {/if}

    <!-- QR Code Scan / Share Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (showQrDialog = true)}
      title="Kode QR & Pindai"
      aria-label="Kode QR"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue, #0284C7)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--brand-blue, #0284C7)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
    </button>
  {/snippet}
</AppBar>

<div class="contacts-page-container">
  <!-- Toast Notification -->
  {#if vaultToast}
    <div class="vault-toast-banner">
      <span>🛡️ {vaultToast}</span>
    </div>
  {/if}

  {#if isSearchActive}
    <div class="search-bar-wrap">
      <input
        type="text"
        class="search-input"
        placeholder="Cari nama, @username, atau ketik kode sandi brankas..."
        value={searchQuery}
        oninput={(e) => handleSearchInput((e.target as HTMLInputElement).value)}
      />
    </div>
  {/if}

  <!-- Banner Private Vault Aktif (jika vault sedang terbuka) -->
  {#if mekaarStore.vaultUnlocked}
    <div class="vault-status-banner">
      <div class="vault-status-left">
        <span class="vault-status-icon">👁️</span>
        <span class="vault-status-text">Private Vault Terbuka</span>
      </div>
      <button
        type="button"
        class="btn-vault-lock"
        onclick={() => {
          mekaarStore.vaultUnlocked = false;
          triggerVaultToast('Private Vault Dikunci Kembali');
        }}
      >
        🔒 Kunci
      </button>
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
          <svg width="18" height="18" viewBox="0 0 24 24" fill="var(--brand-blue, #0284C7)">
            <path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"/>
          </svg>
          <span class="toolbar-title">Kontak</span>
          <span class="count-pill">{sortedContacts.length}</span>
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
              Baru Ditambahkan
            </button>
          </div>
        </div>

        <button
          type="button"
          class="toolbar-more-btn"
          onclick={() => (showAddContactDialog = true)}
          title="Tambah Kontak"
          aria-label="Tambah Kontak"
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
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
              <div class="contact-name-badges-row">
                <span class="contact-item-name">{contact.displayName}</span>
                {#if isContactVaultHidden(contact) && mekaarStore.vaultUnlocked}
                  <span class="vault-tag-pill">🔒 Vault</span>
                {/if}
                {#if contact.isBlocked}
                  <span class="blocked-tag-pill">Diblokir</span>
                {/if}
              </div>
              <span class="contact-item-handle">{contact.username}</span>
            </div>
            <button
              type="button"
              class="contact-info-btn"
              onclick={(e) => {
                e.stopPropagation();
                openContactInfo(contact);
              }}
              title="Informasi Kontak"
              aria-label="Info Kontak"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--text-secondary, #64748B)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" y1="16" x2="12" y2="12"></line>
                <line x1="12" y1="8" x2="12.01" y2="8"></line>
              </svg>
            </button>
          </div>
        {:else}
          <div style="padding: 24px; text-align: center; color: var(--text-secondary, #64748B); font-size: 0.9rem;">
            {searchQuery ? 'Kontak tidak ditemukan.' : 'Belum ada kontak terdaftar.'}
          </div>
        {/each}
      </div>
    </Card>
  </div>
</div>

<!-- Modal Tambah Kontak Manual -->
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

<!-- Modal Info Kontak & Privasi -->
<Dialog
  open={showContactInfoDialog}
  title="Informasi Kontak & Keamanan"
  onclose={() => (showContactInfoDialog = false)}
>
  {#if selectedContact}
    <div class="contact-details-box">
      <div class="details-profile-header">
        <SketchAvatar
          name={selectedContact.displayName}
          size={56}
          online={selectedContact.isOnline}
          isGuardian={selectedContact.isGuardian}
        />
        <div class="details-name-column">
          <h4>{selectedContact.displayName}</h4>
          <p>{selectedContact.username}</p>
        </div>
      </div>

      <div class="details-field">
        <span class="field-title">FINGERPRINT KUNCI PUBLIK (ED25519)</span>
        <code class="fingerprint-code">{selectedContact.fingerprint}</code>
      </div>

      <div class="details-field">
        <span class="field-title">CATATAN KONTAK</span>
        <p class="field-value">{selectedContact.notes || 'Kontak terverifikasi Aegis'}</p>
      </div>

      <div class="details-actions-list">
        <button
          type="button"
          class="btn-detail-action"
          onclick={toggleVaultForSelected}
        >
          {isContactVaultHidden(selectedContact) ? '👁️ Tampilkan dari Vault' : '🔒 Sembunyikan ke Private Vault'}
        </button>

        <button
          type="button"
          class="btn-detail-action {selectedContact.isBlocked ? 'btn-unblock' : 'btn-block'}"
          onclick={toggleBlockForSelected}
        >
          {selectedContact.isBlocked ? '✓ Buka Blokir Kontak' : '⛔ Blokir Pengguna Ini'}
        </button>
      </div>
    </div>
  {/if}
</Dialog>

<!-- Dialog Kode QR Profil Telegram Style -->
<MyQrDialog
  open={showQrDialog}
  onclose={() => (showQrDialog = false)}
  onScanClick={() => (showScanDialog = true)}
/>

<!-- Dialog Pemindai QR dengan Kamera & Off-grid Fallback -->
<ContactQrScanDialog
  open={showScanDialog}
  onclose={() => (showScanDialog = false)}
  onConnectSuccess={(convId) => {
    mekaarStore.openConversation(convId);
  }}
/>

<!-- Dialog Permintaan Chat Masuk (Chat Requests) -->
<ChatRequestsDialog
  open={showRequestsDialog}
  onclose={() => (showRequestsDialog = false)}
/>

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
    background-color: var(--brand-blue-subtle, #EFF6FF);
  }

  .req-badge-action-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    background-color: rgba(249, 115, 22, 0.12);
    border: 1px solid rgba(249, 115, 22, 0.3);
    padding: 4px 10px;
    border-radius: var(--radius-pill, 999px);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .req-badge-action-btn:hover {
    background-color: rgba(249, 115, 22, 0.2);
  }

  .bell-icon {
    font-size: 0.85rem;
  }

  .req-count-pill {
    font-size: 0.74rem;
    font-weight: 800;
    color: #EA580C;
  }

  .vault-toast-banner {
    background-color: #0D9488;
    color: #FFFFFF;
    font-size: 0.82rem;
    font-weight: 700;
    padding: 9px 16px;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(13, 148, 136, 0.25);
    animation: fadeIn 0.2s ease-out;
  }

  .vault-status-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: rgba(13, 148, 136, 0.12);
    border: 1px solid rgba(13, 148, 136, 0.25);
    border-radius: 12px;
    padding: 8px 14px;
  }

  .vault-status-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .vault-status-icon {
    font-size: 1rem;
  }

  .vault-status-text {
    font-size: 0.82rem;
    font-weight: 800;
    color: #0D9488;
  }

  .btn-vault-lock {
    background-color: #0D9488;
    color: #FFFFFF;
    border: none;
    border-radius: var(--radius-pill, 999px);
    padding: 4px 12px;
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
  }

  .search-bar-wrap {
    margin-bottom: 2px;
  }

  .search-input {
    width: 100%;
    padding: 10px 16px;
    border-radius: var(--radius-pill, 999px);
    border: 1px solid var(--border-subtle, #CBD5E1);
    background-color: #FFFFFF;
    font-size: 0.88rem;
    color: var(--text-primary, #0F172A);
    box-sizing: border-box;
    outline: none;
  }

  .search-input:focus {
    border-color: var(--brand-blue, #0284C7);
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
    color: var(--text-primary, #0F172A);
  }

  .radar-segment {
    display: flex;
    background-color: #F1F5F9;
    padding: 3px;
    border-radius: var(--radius-pill, 999px);
    gap: 2px;
  }

  .radar-seg-btn {
    padding: 5px 14px;
    border-radius: var(--radius-pill, 999px);
    border: none;
    background: transparent;
    font-size: 0.84rem;
    font-weight: 600;
    color: var(--text-secondary, #64748B);
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
    color: var(--text-primary, #0F172A);
  }

  .radar-empty-desc {
    font-size: 0.82rem;
    color: var(--text-secondary, #64748B);
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
    color: var(--text-primary, #0F172A);
  }

  .count-pill {
    background-color: #EFF6FF;
    color: var(--brand-blue, #0284C7);
    font-size: 0.76rem;
    font-weight: 700;
    padding: 1px 7px;
    border-radius: 999px;
  }

  .sort-segment {
    display: flex;
    background-color: #F1F5F9;
    padding: 2px;
    border-radius: var(--radius-pill, 999px);
    gap: 2px;
  }

  .sort-btn {
    padding: 4px 12px;
    border-radius: var(--radius-pill, 999px);
    border: none;
    background: transparent;
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--text-secondary, #64748B);
    cursor: pointer;
  }

  .sort-btn.active {
    background-color: var(--brand-blue, #0284C7);
    color: #FFFFFF;
  }

  .toolbar-more-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #64748B);
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .divider-line {
    height: 1px;
    background-color: var(--border-light, #F1F5F9);
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

  .contact-name-badges-row {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .contact-item-name {
    font-size: 1.05rem;
    font-weight: 700;
    color: var(--text-primary, #0F172A);
  }

  .vault-tag-pill {
    background-color: rgba(13, 148, 136, 0.12);
    color: #0D9488;
    font-size: 0.68rem;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 4px;
  }

  .blocked-tag-pill {
    background-color: rgba(239, 68, 68, 0.12);
    color: #DC2626;
    font-size: 0.68rem;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 4px;
  }

  .contact-item-handle {
    font-size: 0.85rem;
    color: var(--text-secondary, #64748B);
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

  /* Contact Details Dialog Box */
  .contact-details-box {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 8px 0;
  }

  .details-profile-header {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .details-name-column h4 {
    font-size: 1.1rem;
    font-weight: 800;
    color: var(--text-primary, #0F172A);
    margin: 0;
  }

  .details-name-column p {
    font-size: 0.85rem;
    color: var(--brand-blue, #0284C7);
    margin: 2px 0 0 0;
  }

  .details-field {
    background-color: var(--bg-surface, #F8FAFC);
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: 10px;
    padding: 10px 12px;
  }

  .field-title {
    font-size: 0.7rem;
    font-weight: 800;
    color: var(--text-secondary, #64748B);
    display: block;
    margin-bottom: 4px;
    letter-spacing: 0.5px;
  }

  .fingerprint-code {
    font-size: 0.82rem;
    color: var(--brand-blue, #0284C7);
    font-family: monospace;
    font-weight: 700;
    word-break: break-all;
  }

  .field-value {
    font-size: 0.85rem;
    color: var(--text-primary, #0F172A);
    margin: 0;
  }

  .details-actions-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 8px;
  }

  .btn-detail-action {
    width: 100%;
    height: 42px;
    border-radius: var(--radius-pill, 999px);
    border: 1px solid var(--border-subtle, #CBD5E1);
    background: transparent;
    color: var(--text-primary, #0F172A);
    font-size: 0.85rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-detail-action:hover {
    background-color: var(--border-light, #F1F5F9);
  }

  .btn-detail-action.btn-block {
    color: #DC2626;
    border-color: rgba(239, 68, 68, 0.4);
  }

  .btn-detail-action.btn-block:hover {
    background-color: rgba(239, 68, 68, 0.08);
  }

  .btn-detail-action.btn-unblock {
    color: #059669;
    border-color: rgba(16, 185, 129, 0.4);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
