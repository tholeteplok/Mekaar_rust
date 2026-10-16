<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';
  import MyQrDialog from '../components/MyQrDialog.svelte';
  import ContactQrScanDialog from '../components/ContactQrScanDialog.svelte';
  import ChatRequestsDialog from '../components/ChatRequestsDialog.svelte';
  import ContactDetailSheet from '../components/ContactDetailSheet.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import {
    Bell,
    QrCode,
    MagnifyingGlass,
    ShieldCheck,
    LockKey,
    LockSimple,
    Broadcast,
    ArrowsClockwise,
    Users,
    SortAscending,
    ClockCounterClockwise,
    List,
    SquaresFour,
    Plus,
    ChatCircleDots,
    Info,
    UserPlus
  } from 'phosphor-svelte';

  let radarFilter = $state<'semua' | 'kontak'>('semua');
  let sortMode = $state<'az' | 'recent'>('az');
  let isGridView = $state(false);
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
  let isSearchingContact = $state(false);
  let addContactError = $state('');
  let addContactSuccess = $state('');

  function openAddContactModal() {
    newContactUsername = '';
    newContactDisplayName = '';
    addContactError = '';
    addContactSuccess = '';
    isSearchingContact = false;
    showAddContactDialog = true;
  }

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
          isTyping: false,
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

  async function handleAddContact() {
    if (!newContactUsername.trim()) {
      addContactError = 'Silakan masukkan username pengguna.';
      return;
    }
    isSearchingContact = true;
    addContactError = '';
    addContactSuccess = '';

    try {
      const res = await mekaarStore.searchAndAddContact(newContactUsername);
      if (res.success) {
        addContactSuccess = res.message;
        setTimeout(() => {
          showAddContactDialog = false;
          newContactUsername = '';
          newContactDisplayName = '';
          addContactSuccess = '';
          isSearchingContact = false;
          if (res.conversationId) {
            mekaarStore.openConversation(res.conversationId);
          }
        }, 900);
      } else {
        addContactError = res.message;
        isSearchingContact = false;
      }
    } catch {
      addContactError = 'Gagal memproses penambahan kontak. Periksa koneksi jaringan.';
      isSearchingContact = false;
    }
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
        <span class="bell-icon" style="display: flex; align-items: center;">
          <Bell size={18} weight="bold" />
        </span>
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
      <QrCode size={22} weight="bold" color="var(--brand-blue)" />
    </button>

    <!-- Search Button -->
    <button
      type="button"
      class="icon-action-btn"
      onclick={() => (isSearchActive = !isSearchActive)}
      title="Cari Kontak"
      aria-label="Cari Kontak"
    >
      <MagnifyingGlass size={22} weight="bold" color="var(--brand-blue)" />
    </button>
  {/snippet}
</AppBar>

<div class="contacts-page-container">
  <!-- Toast Notification -->
  {#if vaultToast}
    <div class="vault-toast-banner">
      <span style="display: flex; align-items: center; justify-content: center; gap: 6px;">
        <ShieldCheck size={18} weight="bold" color="var(--brand-teal)" />
        <span>{vaultToast}</span>
      </span>
    </div>
  {/if}

  {#if isSearchActive}
    <div class="search-bar-wrap">
      <input
        type="text"
        class="search-input"
        placeholder="Cari kontak atau username..."
        value={searchQuery}
        oninput={(e) => handleSearchInput((e.target as HTMLInputElement).value)}
      />
    </div>
  {/if}

  <!-- Banner Private Vault Aktif (jika vault sedang terbuka) -->
  {#if mekaarStore.vaultUnlocked}
    <div class="vault-status-banner">
      <div class="vault-status-left">
        <span class="vault-status-icon" style="display: flex; align-items: center;">
          <LockKey size={18} weight="bold" color="var(--brand-teal)" />
        </span>
        <span class="vault-status-text">Private Vault Terbuka</span>
      </div>
      <button
        type="button"
        class="btn-vault-lock"
        style="display: flex; align-items: center; gap: 4px;"
        onclick={() => {
          mekaarStore.vaultUnlocked = false;
          triggerVaultToast('Private Vault Dikunci Kembali');
        }}
      >
        <LockSimple size={15} weight="bold" />
        <span>Kunci</span>
      </button>
    </div>
  {/if}

  <!-- Section 1: Teman Sekitar (Proximity Radar) -->
  <div class="section-radar">
    <div class="radar-header-row">
      <div class="radar-title-wrap">
        <span class="radar-icon" style="display: flex; align-items: center;">
          <Broadcast size={20} weight="bold" color="var(--brand-blue)" />
        </span>
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
        <ArrowsClockwise size={18} weight="bold" color="var(--brand-blue)" />
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

  <!-- Section 2: Kontak Card (Clean Flat Card with List/Grid Toggle) -->
  <div class="contacts-card-wrap">
    <Card withTicks={false} padding="16px">
      <!-- Card Toolbar Header -->
      <div class="card-toolbar-row">
        <div class="card-toolbar-left">
          <Users size={18} weight="bold" color="var(--brand-blue)" />
          <span class="toolbar-title">Kontak</span>
          <span class="count-pill">{sortedContacts.length}</span>
        </div>

        <div class="card-toolbar-center">
          <div class="sort-segment">
            <button
              type="button"
              class="sort-btn {sortMode === 'az' ? 'active' : ''}"
              onclick={() => (sortMode = 'az')}
              style="display: flex; align-items: center; gap: 4px;"
            >
              <SortAscending size={14} weight="bold" />
              <span>A-Z</span>
            </button>
            <button
              type="button"
              class="sort-btn {sortMode === 'recent' ? 'active' : ''}"
              onclick={() => (sortMode = 'recent')}
              style="display: flex; align-items: center; gap: 4px;"
            >
              <ClockCounterClockwise size={14} weight="bold" />
              <span>Baru Ditambahkan</span>
            </button>
          </div>
        </div>

        <div class="card-toolbar-right-actions">
          <!-- Toggle List vs Grid View (Paritas Mekaar_chat) -->
          <div class="view-mode-toggle">
            <button
              type="button"
              class="view-mode-btn {!isGridView ? 'active' : ''}"
              onclick={() => (isGridView = false)}
              title="Tampilan Daftar (List)"
              aria-label="Tampilan Daftar"
              style="display: flex; align-items: center; justify-content: center;"
            >
              <List size={18} weight="bold" />
            </button>
            <button
              type="button"
              class="view-mode-btn {isGridView ? 'active' : ''}"
              onclick={() => (isGridView = true)}
              title="Tampilan Kotak (Grid)"
              aria-label="Tampilan Kotak"
              style="display: flex; align-items: center; justify-content: center;"
            >
              <SquaresFour size={18} weight="bold" />
            </button>
          </div>

          <button
            type="button"
            class="toolbar-more-btn"
            onclick={openAddContactModal}
            title="Tambah Kontak"
            aria-label="Tambah Kontak"
            style="display: flex; align-items: center; justify-content: center;"
          >
            <Plus size={18} weight="bold" />
          </button>
        </div>
      </div>

      <div class="divider-line"></div>

      <!-- Contacts Rendering: List View vs Grid View -->
      {#if isGridView}
        <!-- GRID VIEW (2 Kolom Modern Cards) -->
        <div class="contacts-grid-layout">
          {#each sortedContacts as contact (contact.id)}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="contact-grid-card"
              onclick={() => handleStartChat(contact)}
            >
              <SketchAvatar
                name={contact.displayName}
                size={58}
                online={contact.isOnline}
                isGuardian={contact.isGuardian}
              />
              <span class="grid-contact-name">{contact.displayName}</span>
              <span class="grid-contact-handle">{contact.username}</span>

              <div class="grid-card-badges-row">
                {#if isContactVaultHidden(contact) && mekaarStore.vaultUnlocked}
                  <span class="vault-tag-pill" style="display: inline-flex; align-items: center; gap: 3px;">
                    <LockKey size={11} weight="bold" />
                    <span>Vault</span>
                  </span>
                {/if}
                {#if contact.isBlocked}
                  <span class="blocked-tag-pill">Diblokir</span>
                {/if}
              </div>

              <div class="grid-actions-row">
                <button
                  type="button"
                  class="grid-action-btn grid-chat-btn"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleStartChat(contact);
                  }}
                  title="Mulai Chat"
                  style="display: flex; align-items: center; justify-content: center; gap: 4px;"
                >
                  <ChatCircleDots size={15} weight="fill" />
                  <span>Chat</span>
                </button>
                <button
                  type="button"
                  class="grid-action-btn grid-info-btn"
                  onclick={(e) => {
                    e.stopPropagation();
                    openContactInfo(contact);
                  }}
                  title="Info Kontak"
                  style="display: flex; align-items: center; justify-content: center;"
                >
                  <Info size={16} weight="bold" />
                </button>
              </div>
            </div>
          {:else}
            <div style="grid-column: 1 / -1; width: 100%;">
              {#if searchQuery}
                <EmptyState
                  pose="confused"
                  title="Kontak Tidak Ditemukan"
                  message="Tidak ada kontak yang cocok dengan '{searchQuery}'."
                />
              {:else}
                <EmptyState
                  pose="hi"
                  title="Belum Ada Kontak"
                  message="Mulai tambahkan teman terpercaya atau pindai kode QR untuk memulai komunikasi terenkripsi."
                  actionLabel="Tambah Kontak"
                  actionIcon={UserPlus}
                  onaction={openAddContactModal}
                />
              {/if}
            </div>
          {/each}
        </div>
      {:else}
        <!-- LIST VIEW (Signal Style Clean Flat Inset List) -->
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
                size={48}
                online={contact.isOnline}
                isGuardian={contact.isGuardian}
              />
              <div class="contact-item-info">
                <div class="contact-name-badges-row">
                  <span class="contact-item-name">{contact.displayName}</span>
                  {#if isContactVaultHidden(contact) && mekaarStore.vaultUnlocked}
                    <span class="vault-tag-pill" style="display: inline-flex; align-items: center; gap: 3px;">
                      <LockKey size={11} weight="bold" />
                      <span>Vault</span>
                    </span>
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
                style="display: flex; align-items: center; justify-content: center;"
              >
                <Info size={20} weight="bold" color="var(--text-secondary, #64748B)" />
              </button>
            </div>
          {:else}
            <div style="width: 100%;">
              {#if searchQuery}
                <EmptyState
                  pose="confused"
                  title="Kontak Tidak Ditemukan"
                  message="Tidak ada kontak yang cocok dengan '{searchQuery}'."
                />
              {:else}
                <EmptyState
                  pose="hi"
                  title="Belum Ada Kontak"
                  message="Mulai tambahkan teman terpercaya atau pindai kode QR untuk memulai komunikasi terenkripsi."
                  actionLabel="Tambah Kontak"
                  actionIcon={UserPlus}
                  onaction={openAddContactModal}
                />
              {/if}
            </div>
          {/each}
        </div>
      {/if}
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
    {#if addContactError}
      <div style="padding: 10px 14px; background: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.3); border-radius: 8px; color: #ef4444; font-size: 0.85rem; line-height: 1.4;">
        {addContactError}
      </div>
    {/if}
    {#if addContactSuccess}
      <div style="padding: 10px 14px; background: rgba(34, 197, 94, 0.1); border: 1px solid rgba(34, 197, 94, 0.3); border-radius: 8px; color: #22c55e; font-size: 0.85rem; line-height: 1.4;">
        {addContactSuccess}
      </div>
    {/if}
    <label style="font-size: 0.85rem; font-weight: 600;">
      Username Teman:
      <input
        type="text"
        class="sketch-input"
        placeholder="@teplok"
        bind:value={newContactUsername}
        disabled={isSearchingContact}
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
        disabled={isSearchingContact}
        style="margin-top: 4px;"
      />
    </label>
    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
      <button type="button" class="btn-secondary" onclick={() => (showAddContactDialog = false)} disabled={isSearchingContact}>
        Batal
      </button>
      <button type="button" class="btn-primary" onclick={handleAddContact} disabled={isSearchingContact}>
        {isSearchingContact ? 'Mencari ke Server...' : 'Simpan Kontak'}
      </button>
    </div>
  </div>
</Dialog>

<!-- Bottom Sheet Detail Kontak Terpusat & Action Hub -->
<ContactDetailSheet
  open={showContactInfoDialog}
  contact={selectedContact}
  onclose={() => (showContactInfoDialog = false)}
  onOpenChat={(convId) => {
    mekaarStore.openConversation(convId);
  }}
/>

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
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: 0 16px 24px 16px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .icon-action-btn {
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    border-radius: 50%;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.15s ease, transform 0.1s ease;
  }

  .icon-action-btn:active {
    transform: scale(0.92);
    background-color: var(--brand-blue-subtle, #EFF6FF);
  }

  .req-badge-action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background-color: rgba(249, 115, 22, 0.12);
    border: 1px solid rgba(249, 115, 22, 0.3);
    min-height: var(--touch-target-min, 40px);
    padding: 6px 12px;
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
    background-color: var(--brand-teal);
    color: #FFFFFF;
    font-size: 0.82rem;
    font-weight: 700;
    padding: 9px 16px;
    border-radius: var(--radius-md, 12px);
    box-shadow: 0 4px 12px rgba(13, 148, 136, 0.25);
    animation: fadeIn 0.2s ease-out;
  }

  .vault-status-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: rgba(13, 148, 136, 0.12);
    border: 1px solid rgba(13, 148, 136, 0.25);
    border-radius: var(--radius-md, 12px);
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
    color: var(--brand-teal);
  }

  .btn-vault-lock {
    background-color: var(--brand-teal);
    color: #FFFFFF;
    border: none;
    border-radius: var(--radius-pill);
    min-height: 32px;
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
    gap: 10px;
    flex-wrap: wrap;
  }

  .card-toolbar-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .toolbar-title {
    font-size: 0.95rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary, #0F172A);
  }

  .count-pill {
    background-color: var(--brand-blue-subtle, #EFF6FF);
    color: var(--brand-blue, #0284C7);
    font-size: 0.76rem;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 999px;
  }

  .sort-segment {
    display: flex;
    background-color: var(--bg-surface, #F1F5F9);
    padding: 3px;
    border-radius: var(--radius-pill, 999px);
    gap: 2px;
  }

  .sort-btn {
    padding: 6px 14px;
    min-height: var(--touch-target-min, 40px);
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    font-size: 0.8rem;
    font-family: var(--font-active);
    font-weight: 600;
    color: var(--text-secondary);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .sort-btn.active {
    background-color: var(--brand-blue);
    color: #FFFFFF;
    box-shadow: 0 1px 4px rgba(2, 132, 199, 0.2);
  }

  .card-toolbar-right-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .view-mode-toggle {
    display: flex;
    align-items: center;
    background: var(--bg-surface, #F1F5F9);
    padding: 2px;
    border-radius: var(--radius-sm, 8px);
    gap: 2px;
  }

  .view-mode-btn {
    border: none;
    background: transparent;
    padding: 0;
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    border-radius: var(--radius-sm, 6px);
    font-size: 0.85rem;
    color: var(--text-secondary);
    cursor: pointer;
    line-height: 1;
    transition: all 0.15s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .view-mode-btn.active {
    background: var(--surface-card, #FFFFFF);
    color: var(--brand-blue);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  }

  @media (max-width: 600px) {
    .card-toolbar-row {
      flex-direction: column;
      align-items: stretch;
      gap: 10px;
    }

    .card-toolbar-left {
      justify-content: space-between;
    }

    .card-toolbar-center {
      width: 100%;
    }

    .sort-segment {
      width: 100%;
      display: flex;
    }

    .sort-btn {
      flex: 1;
      justify-content: center;
    }

    .card-toolbar-right-actions {
      justify-content: flex-end;
    }
  }

  .toolbar-more-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.15s ease, transform 0.1s ease;
  }

  .toolbar-more-btn:hover {
    background-color: var(--bg-secondary);
  }

  .toolbar-more-btn:active {
    transform: scale(0.92);
  }

  .grid-contact-name {
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary, #0F172A);
    text-align: center;
    margin-top: 4px;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .grid-contact-handle {
    font-size: 0.78rem;
    color: var(--text-secondary, #64748B);
    text-align: center;
    margin-top: 1px;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .grid-card-badges-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    margin-top: 4px;
    min-height: 18px;
  }

  .grid-actions-row {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    margin-top: 8px;
  }

  .grid-action-btn {
    border: none;
    cursor: pointer;
    min-height: var(--touch-target-min, 40px);
    padding: 8px 12px;
    border-radius: var(--radius-pill);
    font-size: 0.82rem;
    font-weight: 600;
    transition: opacity 0.15s ease;
  }

  .grid-action-btn:active {
    opacity: 0.8;
  }

  .grid-chat-btn {
    flex: 1;
    background: var(--brand-blue);
    color: #FFFFFF;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
  }

  .grid-info-btn {
    background: #F1F5F9;
    color: var(--text-secondary);
    padding: 8px 12px;
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
    color: var(--text-primary);
  }

  .vault-tag-pill {
    background-color: rgba(13, 148, 136, 0.12);
    color: var(--brand-teal);
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
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .contact-info-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.15s ease, transform 0.1s ease;
  }

  .contact-info-btn:hover {
    background-color: var(--bg-secondary);
  }

  .contact-info-btn:active {
    transform: scale(0.92);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
