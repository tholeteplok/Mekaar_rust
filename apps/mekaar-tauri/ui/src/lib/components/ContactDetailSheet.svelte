<script lang="ts">
  import { mekaarStore } from '../stores';
  import type { ContactModel, Conversation } from '../types';
  import BottomSheet from './BottomSheet.svelte';
  import Dialog from './Dialog.svelte';
  import SketchAvatar from './SketchAvatar.svelte';
  import ChatRoomPrivacySheet from './ChatRoomPrivacySheet.svelte';
  import { WebRtcManager } from '../services/webrtc';

  interface Props {
    open: boolean;
    contact: ContactModel | null;
    conversationId?: string | null;
    onclose: () => void;
    onOpenChat?: (convId: string) => void;
  }

  let { open, contact, conversationId, onclose, onOpenChat }: Props = $props();

  let showE2eeDetails = $state(false);
  let showSasDialog = $state(false);
  let showPrivacySheet = $state(false);
  let showClearConfirmDialog = $state(false);
  let showBlockConfirmDialog = $state(false);
  let toastMessage = $state('');
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  function showToast(msg: string) {
    toastMessage = msg;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toastMessage = '';
    }, 2800);
  }

  // Cari percakapan terkait
  const matchedConv = $derived.by(() => {
    if (!contact) return null;
    if (conversationId) {
      const found = mekaarStore.conversations.find((c) => c.id === conversationId);
      if (found) return found;
    }
    return (
      mekaarStore.conversations.find(
        (c) => c.title === contact.displayName || c.id === `conv-${contact.id}`
      ) ?? null
    );
  });

  const isVaultHidden = $derived(!!matchedConv?.isVaultHidden);
  const isMuted = $derived(!!matchedConv?.isMuted);

  // Dapatkan atau buat ID ruang obrolan
  function getOrCreateConvId(): string {
    if (matchedConv) return matchedConv.id;
    if (!contact) return '';

    const newId = `conv-${Date.now()}`;
    const newConv: Conversation = {
      id: newId,
      title: contact.displayName,
      subtitle: '🔒 Sesi E2EE Terenkripsi',
      avatarColor: contact.avatarColor || '#EFF6FF',
      unreadCount: 0,
      lastTimestamp: 'Sekarang',
      isOnline: contact.isOnline,
      isTyping: false,
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
    };
    mekaarStore.conversations = [newConv, ...mekaarStore.conversations];
    return newId;
  }

  function handleJumpToChat() {
    if (!contact) return;
    const convId = getOrCreateConvId();
    onclose();
    if (onOpenChat) {
      onOpenChat(convId);
    } else {
      mekaarStore.openConversation(convId);
    }
  }

  function handleStartCall(isVideo: boolean) {
    if (!contact) return;
    const convId = getOrCreateConvId();
    const conv = mekaarStore.conversations.find((c) => c.id === convId);
    if (conv) {
      onclose();
      mekaarStore.startCall(conv, isVideo);
    }
  }

  function handleToggleVault() {
    if (!contact) return;
    const convId = getOrCreateConvId();
    mekaarStore.toggleVaultHidden(convId);
    showToast(
      isVaultHidden
        ? 'Obrolan dikeluarkan dari Private Vault'
        : 'Obrolan disembunyikan ke dalam Private Vault'
    );
  }

  function handleToggleGuardian() {
    if (!contact) return;
    const nextStatus = mekaarStore.toggleGuardianContact(contact.id);
    showToast(
      nextStatus
        ? `${contact.displayName} diangkat sebagai Wali Siaga (Guardian)`
        : `${contact.displayName} dicabut dari Wali Siaga`
    );
  }

  function handleToggleMute() {
    if (!contact) return;
    const convId = getOrCreateConvId();
    const muted = mekaarStore.toggleMuteConversation(convId);
    showToast(
      muted ? 'Notifikasi obrolan dibisukan' : 'Notifikasi obrolan diaktifkan'
    );
  }

  function handleCopyText(text: string, label: string) {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(text);
      showToast(`${label} berhasil disalin ke papan klip`);
    }
  }

  function handleConfirmBlock() {
    if (!contact) return;
    mekaarStore.toggleContactBlock(contact.id);
    showBlockConfirmDialog = false;
    showToast(
      contact.isBlocked
        ? `${contact.displayName} telah diblokir`
        : `Blokir ${contact.displayName} telah dibuka`
    );
  }

  function handleConfirmClearChat() {
    if (matchedConv) {
      mekaarStore.clearConversationMessages(matchedConv.id);
      showClearConfirmDialog = false;
      showToast('Riwayat obrolan telah dibersihkan');
    }
  }

  // SAS (Short Authentication String) untuk verifikasi MITM
  const sasCode = $derived.by(() => {
    if (!contact) return { code: '8492 1039 4812', emojis: '🛡️ 🔐 ⚡ 🛰️' };
    return WebRtcManager.generateSas(`mekaar-verify-${contact.id}`);
  });
</script>

<BottomSheet open={open} title="Detail Kontak & Keamanan" onclose={onclose}>
  {#if contact}
    <div class="contact-sheet-content">
      <!-- 1. Header Hub Profil -->
      <div class="profile-header-card">
        <div class="avatar-presence-wrapper">
          <SketchAvatar
            name={contact.displayName}
            size={68}
            online={contact.isOnline}
            isGuardian={contact.isGuardian}
          />
        </div>

        <div class="profile-title-col">
          <h2 class="profile-display-name">{contact.displayName}</h2>
          <span class="profile-username-tag">{contact.username}</span>

          <!-- Indikator Presensi -->
          <div class="presence-badge-row">
            <span class="presence-dot {contact.isOnline ? 'dot-online' : 'dot-offline'}"></span>
            <span class="presence-text">
              {contact.isOnline ? 'Daring (Online)' : 'Terakhir dilihat baru-baru ini'}
            </span>
          </div>

          <!-- Status Badges -->
          <div class="status-tags-row">
            {#if contact.isGuardian}
              <span class="badge-pill badge-guardian">🛡️ Guardian Siaga</span>
            {/if}
            {#if isVaultHidden && mekaarStore.vaultUnlocked}
              <span class="badge-pill badge-vault">🔒 Private Vault</span>
            {/if}
            {#if contact.isBlocked}
              <span class="badge-pill badge-blocked">⛔ Diblokir</span>
            {/if}
          </div>
        </div>
      </div>

      <!-- Toast Feedback Popup -->
      {#if toastMessage}
        <div class="sheet-toast-banner">
          <span>✨ {toastMessage}</span>
        </div>
      {/if}

      <!-- 2. Quick Action Hub (4 Tombol Sentral Frosted Glass) -->
      <div class="action-hub-grid">
        <button
          type="button"
          class="action-hub-btn"
          onclick={handleJumpToChat}
          title="Buka Obrolan"
        >
          <span class="hub-icon">💬</span>
          <span class="hub-label">Obrolan</span>
        </button>

        <button
          type="button"
          class="action-hub-btn"
          onclick={() => handleStartCall(false)}
          title="Panggilan Suara E2EE"
        >
          <span class="hub-icon">📞</span>
          <span class="hub-label">Suara</span>
        </button>

        <button
          type="button"
          class="action-hub-btn"
          onclick={() => handleStartCall(true)}
          title="Panggilan Video E2EE"
        >
          <span class="hub-icon">🎥</span>
          <span class="hub-label">Video</span>
        </button>

        <button
          type="button"
          class="action-hub-btn {isVaultHidden ? 'hub-active' : ''}"
          onclick={handleToggleVault}
          title={isVaultHidden ? 'Keluarkan dari Vault' : 'Sembunyikan ke Vault'}
        >
          <span class="hub-icon">{isVaultHidden ? '👁️' : '🔒'}</span>
          <span class="hub-label">{isVaultHidden ? 'Tampilkan' : 'Vault'}</span>
        </button>
      </div>

      <!-- 3. Bio & Catatan Taktis Lapangan -->
      <div class="info-card-box">
        <div class="card-box-header">
          <span class="section-micro-title">BIO / CATATAN KEAMANAN</span>
          {#if contact.notes}
            <button
              type="button"
              class="copy-micro-btn"
              onclick={() => handleCopyText(contact.notes, 'Catatan kontak')}
            >
              📋 Salin
            </button>
          {/if}
        </div>
        <p class="notes-text-content">
          {contact.notes || 'Kontak terverifikasi jaringan Mekaar Aegis Shield.'}
        </p>
      </div>

      <!-- 4. Pengaturan Obrolan & Relasi Keamanan -->
      <div class="settings-list-card">
        <span class="section-micro-title" style="padding: 12px 14px 6px 14px;">
          PENGATURAN OBROLAN & RELASI
        </span>

        <!-- Toggle Wali Siaga (Guardian) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-toggle-row" onclick={handleToggleGuardian}>
          <div class="setting-icon-col icon-teal">
            🛡️
          </div>
          <div class="setting-text-col">
            <span class="setting-title">Wali Siaga (Guardian)</span>
            <span class="setting-desc">Dapat memantau lokasi darurat & menerima alert SOS</span>
          </div>
          <div class="custom-switch {contact.isGuardian ? 'switch-active' : ''}">
            <span class="switch-thumb"></span>
          </div>
        </div>

        <div class="card-separator"></div>

        <!-- Toggle Bisukan Notifikasi (Mute) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-toggle-row" onclick={handleToggleMute}>
          <div class="setting-icon-col icon-amber">
            🔔
          </div>
          <div class="setting-text-col">
            <span class="setting-title">Bisukan Notifikasi</span>
            <span class="setting-desc">Senyapkan suara pesan & dering panggilan dari obrolan ini</span>
          </div>
          <div class="custom-switch {isMuted ? 'switch-active' : ''}">
            <span class="switch-thumb"></span>
          </div>
        </div>

        <div class="card-separator"></div>

        <!-- Privasi Obrolan Ekstrem (2 Arah) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="setting-toggle-row" onclick={() => (showPrivacySheet = true)}>
          <div class="setting-icon-col icon-teal">
            🛡️
          </div>
          <div class="setting-text-col">
            <span class="setting-title">Pengaturan Privasi Obrolan (2 Arah)</span>
            <span class="setting-desc">Proteksi layar, anti-copy, burn on exit, dan wipe 2 arah</span>
          </div>
          <div class="arrow-circle">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--text-muted)" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </div>
        </div>

        <div class="card-separator"></div>

        <!-- Sidik Jari Kunci Keamanan E2EE -->
        <div class="setting-info-row">
          <div class="setting-icon-col icon-blue">
            🔐
          </div>
          <div class="setting-text-col">
            <div style="display: flex; align-items: center; justify-content: space-between;">
              <span class="setting-title">Sidik Jari Kunci Publik E2EE</span>
              <button
                type="button"
                class="reveal-eye-btn"
                onclick={() => (showE2eeDetails = !showE2eeDetails)}
              >
                {showE2eeDetails ? 'Sembunyikan' : 'Lihat'}
              </button>
            </div>
            {#if showE2eeDetails}
              <div class="fingerprint-box">
                <code class="fingerprint-mono">{contact.fingerprint}</code>
                <button
                  type="button"
                  class="copy-fingerprint-btn"
                  onclick={() => handleCopyText(contact.fingerprint, 'Sidik jari E2EE')}
                  title="Salin Sidik Jari Kunci"
                >
                  Salin
                </button>
              </div>
            {:else}
              <span class="setting-desc">Ketuk 'Lihat' untuk memeriksa sidik jari kunci asimetris</span>
            {/if}

            <button
              type="button"
              class="verify-sas-link-btn"
              onclick={() => (showSasDialog = true)}
            >
              🛡️ Bandingkan Kode SAS (Aegis Shield)
            </button>
          </div>
        </div>
      </div>

      <!-- 5. Danger Zone Card -->
      <div class="danger-zone-card">
        {#if matchedConv}
          <button
            type="button"
            class="danger-btn danger-clear"
            onclick={() => (showClearConfirmDialog = true)}
          >
            🗑️ Bersihkan Riwayat Obrolan
          </button>
        {/if}

        <button
          type="button"
          class="danger-btn {contact.isBlocked ? 'danger-unblock' : 'danger-block'}"
          onclick={() => (showBlockConfirmDialog = true)}
        >
          {contact.isBlocked ? '✓ Buka Blokir Pengguna' : '⛔ Blokir Kontak Ini'}
        </button>
      </div>
    </div>
  {/if}
</BottomSheet>

<!-- Dialog Verifikasi Kriptografi SAS (Aegis Shield) -->
<Dialog
  open={showSasDialog}
  title="🛡️ Verifikasi Aegis Shield (SAS)"
  onclose={() => (showSasDialog = false)}
>
  {#if contact}
    <div class="sas-verify-container">
      <div class="sas-icon-badge">
        <span style="font-size: 2.2rem;">🛡️</span>
      </div>
      <h3 style="font-size: 1.05rem; font-weight: 700; margin-bottom: 6px;">
        Verifikasi Jalur Komunikasi Bebas Sadap
      </h3>
      <p style="font-size: 0.84rem; color: var(--text-secondary); line-height: 1.45; margin-bottom: 16px;">
        Bandingkan kode autentikasi pendek (*Short Authentication String*) di bawah dengan perangkat <strong>{contact.displayName}</strong> secara tatap muka atau telepon suara untuk memastikan tidak ada serangan <em>Man-In-The-Middle</em> (MITM).
      </p>

      <div class="sas-emojis-banner">
        <span class="sas-emojis-text">{sasCode.emojis}</span>
      </div>

      <div class="sas-code-box">
        <span class="sas-code-numbers">{sasCode.code}</span>
      </div>

      <p style="font-size: 0.8rem; color: var(--text-muted); margin-top: 14px;">
        Kunci Protokol: Ed25519 (Tanda Tangan Kriptografis) & X25519 (Pertukaran Kunci ECDH Sesi).
      </p>

      <div style="display: flex; justify-content: flex-end; margin-top: 18px;">
        <button
          type="button"
          class="btn-primary"
          onclick={() => {
            showSasDialog = false;
            showToast('Identitas kriptografis kontak terverifikasi aman.');
          }}
        >
          ✓ Kunci Terverifikasi Cocok
        </button>
      </div>
    </div>
  {/if}
</Dialog>

<!-- Dialog Konfirmasi Blokir -->
<Dialog
  open={showBlockConfirmDialog}
  title={contact?.isBlocked ? 'Buka Blokir Kontak?' : `Blokir ${contact?.displayName}?`}
  onclose={() => (showBlockConfirmDialog = false)}
>
  <p style="font-size: 0.88rem; color: var(--text-secondary); margin-bottom: 18px; line-height: 1.45;">
    {#if contact?.isBlocked}
      Apakah Anda yakin ingin membuka blokir <strong>{contact.displayName}</strong>? Pengguna akan dapat mengirim pesan dan melakukan panggilan kembali.
    {:else}
      Apakah Anda yakin ingin memblokir <strong>{contact?.displayName}</strong>? Kontak ini tidak akan dapat mengirim pesan, melakukan panggilan, atau memantau status GPS Anda.
    {/if}
  </p>
  <div style="display: flex; justify-content: flex-end; gap: 10px;">
    <button
      type="button"
      class="btn-secondary"
      onclick={() => (showBlockConfirmDialog = false)}
    >
      Batal
    </button>
    <button
      type="button"
      class="btn-primary"
      style={contact?.isBlocked ? 'background: var(--safe-teal);' : 'background: var(--sos-coral); color: #fff;'}
      onclick={handleConfirmBlock}
    >
      {contact?.isBlocked ? 'Buka Blokir' : 'Blokir Kontak'}
    </button>
  </div>
</Dialog>

<!-- Dialog Konfirmasi Hapus Obrolan -->
<Dialog
  open={showClearConfirmDialog}
  title="Bersihkan Riwayat Obrolan?"
  onclose={() => (showClearConfirmDialog = false)}
>
  <p style="font-size: 0.88rem; color: var(--text-secondary); margin-bottom: 18px; line-height: 1.45;">
    Semua pesan teks, lampiran berkas, dan catatan lokasi di ruang obrolan dengan <strong>{contact?.displayName}</strong> akan dihapus secara permanen dari basis data SQLCipher perangkat ini.
  </p>
  <div style="display: flex; justify-content: flex-end; gap: 10px;">
    <button
      type="button"
      class="btn-secondary"
      onclick={() => (showClearConfirmDialog = false)}
    >
      Batal
    </button>
    <button
      type="button"
      class="btn-primary"
      style="background: var(--sos-coral); color: #fff;"
      onclick={handleConfirmClearChat}
    >
      Hapus Riwayat
    </button>
  </div>
</Dialog>

<!-- Sheet Privasi Obrolan Ekstrem (2 Arah) -->
<ChatRoomPrivacySheet
  open={showPrivacySheet}
  conversation={matchedConv}
  onclose={() => (showPrivacySheet = false)}
  onopenSas={() => (showSasDialog = true)}
/>

<style>
  .contact-sheet-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 10px;
  }

  /* 1. Profile Header */
  .profile-header-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 8px 12px 14px 12px;
    background: var(--bg-surface);
    border: 1.5px solid var(--border-subtle);
    border-radius: var(--radius-card);
    box-shadow: var(--shadow-card);
  }

  .avatar-presence-wrapper {
    position: relative;
    margin-bottom: 10px;
  }

  .profile-title-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    width: 100%;
  }

  .profile-display-name {
    font-size: 1.25rem;
    font-weight: 800;
    color: var(--text-primary);
    margin: 0;
    line-height: 1.2;
  }

  .profile-username-tag {
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .presence-badge-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 4px;
  }

  .presence-dot {
    width: 8px;
    height: 8px;
    border-radius: 999px;
  }

  .dot-online {
    background: #10B981;
    box-shadow: 0 0 6px rgba(16, 185, 129, 0.5);
  }

  .dot-offline {
    background: var(--text-muted);
  }

  .presence-text {
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .status-tags-row {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 6px;
    margin-top: 8px;
  }

  .badge-pill {
    padding: 3px 10px;
    font-size: 0.72rem;
    font-weight: 700;
    border-radius: var(--radius-pill);
    letter-spacing: 0.3px;
  }

  .badge-guardian {
    background: var(--safe-teal-bg);
    color: var(--safe-teal-ink);
    border: 1px solid var(--safe-teal);
  }

  .badge-vault {
    background: var(--brand-blue-light);
    color: var(--brand-blue);
    border: 1px solid var(--brand-blue);
  }

  .badge-blocked {
    background: #FEE2E2;
    color: #DC2626;
    border: 1px solid #FCA5A5;
  }

  /* Sheet Toast Banner */
  .sheet-toast-banner {
    background: var(--brand-blue-light);
    color: var(--brand-blue);
    padding: 8px 14px;
    border-radius: var(--radius-button);
    font-size: 0.82rem;
    font-weight: 600;
    text-align: center;
    border: 1px solid var(--border-subtle);
    animation: fadeIn 0.2s ease-out;
  }

  /* 2. Action Hub Grid */
  .action-hub-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
  }

  .action-hub-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 12px 6px;
    background: var(--bg-surface);
    border: 1.5px solid var(--border-subtle);
    border-radius: var(--radius-button);
    cursor: pointer;
    transition: all 0.15s ease-in-out;
    box-shadow: var(--shadow-card);
  }

  .action-hub-btn:hover {
    background: var(--bg-secondary);
    border-color: var(--brand-blue);
    transform: translateY(-1px);
  }

  .action-hub-btn.hub-active {
    background: var(--brand-blue-light);
    border-color: var(--brand-blue);
  }

  .hub-icon {
    font-size: 1.3rem;
  }

  .hub-label {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  /* 3. Info Card Box */
  .info-card-box {
    background: var(--bg-surface);
    border: 1.5px solid var(--border-subtle);
    border-radius: var(--radius-card);
    padding: 12px 14px;
    box-shadow: var(--shadow-card);
  }

  .card-box-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 6px;
  }

  .section-micro-title {
    font-size: 0.7rem;
    font-weight: 800;
    letter-spacing: 0.6px;
    color: var(--text-muted);
  }

  .copy-micro-btn {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .copy-micro-btn:hover {
    background: var(--brand-blue-light);
  }

  .notes-text-content {
    font-size: 0.88rem;
    color: var(--text-primary);
    line-height: 1.45;
    margin: 0;
    word-break: break-word;
  }

  /* 4. Settings List Card */
  .settings-list-card {
    background: var(--bg-surface);
    border: 1.5px solid var(--border-subtle);
    border-radius: var(--radius-card);
    overflow: hidden;
    box-shadow: var(--shadow-card);
  }

  .setting-toggle-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .setting-toggle-row:hover {
    background: var(--bg-secondary);
  }

  .setting-info-row {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px 14px;
  }

  .setting-icon-col {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.15rem;
    flex-shrink: 0;
  }

  .icon-teal {
    background: var(--safe-teal-bg);
  }

  .icon-amber {
    background: var(--warn-amber-bg);
  }

  .icon-blue {
    background: var(--brand-blue-light);
  }

  .setting-text-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .setting-title {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .setting-desc {
    font-size: 0.78rem;
    color: var(--text-secondary);
    line-height: 1.35;
  }

  .card-separator {
    height: 1px;
    background: var(--border-subtle);
  }

  /* Custom Switch */
  .custom-switch {
    width: 44px;
    height: 24px;
    background: var(--border-light);
    border-radius: 999px;
    position: relative;
    transition: background 0.2s ease;
    flex-shrink: 0;
  }

  .custom-switch.switch-active {
    background: var(--safe-teal);
  }

  .switch-thumb {
    width: 18px;
    height: 18px;
    background: #fff;
    border-radius: 999px;
    position: absolute;
    top: 3px;
    left: 3px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s ease;
  }

  .custom-switch.switch-active .switch-thumb {
    transform: translateX(20px);
  }

  /* Fingerprint Box */
  .reveal-eye-btn {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
  }

  .fingerprint-box {
    margin-top: 6px;
    padding: 8px 10px;
    background: var(--bg-secondary);
    border-radius: var(--radius-button);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .fingerprint-mono {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-primary);
    word-break: break-all;
  }

  .copy-fingerprint-btn {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    padding: 4px 8px;
    font-size: 0.72rem;
    font-weight: 700;
    color: var(--text-primary);
    cursor: pointer;
  }

  .verify-sas-link-btn {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.78rem;
    font-weight: 700;
    cursor: pointer;
    text-align: left;
    padding: 0;
    margin-top: 8px;
  }

  .verify-sas-link-btn:hover {
    text-decoration: underline;
  }

  /* 5. Danger Zone */
  .danger-zone-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 4px;
  }

  .danger-btn {
    width: 100%;
    padding: 12px;
    border-radius: var(--radius-button);
    font-size: 0.85rem;
    font-weight: 700;
    cursor: pointer;
    text-align: center;
    border: 1.5px solid transparent;
    transition: all 0.15s ease;
  }

  .danger-clear {
    background: var(--bg-surface);
    color: var(--sos-coral);
    border-color: #FECDD3;
  }

  .danger-clear:hover {
    background: #FFF1F2;
  }

  .danger-block {
    background: #FFF1F2;
    color: #E11D48;
    border-color: #FDA4AF;
  }

  .danger-block:hover {
    background: #FEE2E2;
  }

  .danger-unblock {
    background: var(--safe-teal-bg);
    color: var(--safe-teal-ink);
    border-color: var(--safe-teal);
  }

  .danger-unblock:hover {
    background: #D1FAE5;
  }

  /* SAS Modal Inner */
  .sas-verify-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .sas-icon-badge {
    margin-bottom: 10px;
  }

  .sas-emojis-banner {
    background: var(--bg-secondary);
    padding: 10px 20px;
    border-radius: var(--radius-pill);
    margin-bottom: 12px;
  }

  .sas-emojis-text {
    font-size: 1.6rem;
    letter-spacing: 8px;
  }

  .sas-code-box {
    background: var(--brand-blue-light);
    border: 1.5px dashed var(--brand-blue);
    padding: 10px 18px;
    border-radius: var(--radius-button);
  }

  .sas-code-numbers {
    font-family: var(--font-mono);
    font-size: 1.25rem;
    font-weight: 800;
    color: var(--brand-blue);
    letter-spacing: 2px;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
