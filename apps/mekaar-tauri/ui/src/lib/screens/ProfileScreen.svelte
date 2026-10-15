<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import Dialog from '../components/Dialog.svelte';

  let showEditFieldModal = $state(false);
  let editFieldTitle = $state('');
  let editFieldValue = $state('');
  let editFieldKey = $state<'name' | 'username' | 'bio' | 'email'>('name');

  let showChangePinModal = $state(false);
  let newPinInput = $state('');

  function openEditField(key: 'name' | 'username' | 'bio' | 'email', title: string, currentVal: string) {
    editFieldKey = key;
    editFieldTitle = title;
    editFieldValue = currentVal;
    showEditFieldModal = true;
  }

  function saveField() {
    if (editFieldKey === 'name') {
      mekaarStore.currentUser.displayName = editFieldValue.trim() || mekaarStore.currentUser.displayName;
    } else if (editFieldKey === 'username') {
      const u = editFieldValue.trim();
      mekaarStore.currentUser.username = u.startsWith('@') ? u : `@${u}`;
    } else if (editFieldKey === 'bio') {
      mekaarStore.currentUser.bio = editFieldValue.trim();
    }
    showEditFieldModal = false;
  }

  function handleChangePin() {
    if (newPinInput.length === 6) {
      mekaarStore.pinCode = newPinInput;
      alert('Kode PIN berhasil diperbarui!');
      showChangePinModal = false;
      newPinInput = '';
    } else {
      alert('PIN harus tepat 6 digit angka.');
    }
  }

  function handleLogout() {
    if (confirm('Apakah Anda yakin ingin mengunci akun dan keluar?')) {
      mekaarStore.pinVerified = false;
      mekaarStore.vaultUnlocked = false;
    }
  }
</script>

<AppBar title="Profil" />

<div class="profile-page-container">
  <!-- Large Crayon Avatar & Photo Link -->
  <div class="profile-hero">
    <SketchAvatar
      name={mekaarStore.currentUser.displayName || 'Mika'}
      size={100}
      online={true}
    />
    <button
      type="button"
      class="change-photo-btn"
      onclick={() => alert('Fitur ubah foto: avatar sketsa krayon MEKAAR aktif.')}
    >
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path>
        <circle cx="12" cy="13" r="4"></circle>
      </svg>
      <span>Ubah Foto</span>
    </button>
  </div>

  <!-- User Attributes Card with Corner Ticks -->
  <Card withTicks={false} padding="16px">
    <div class="profile-rows-list">
      <!-- Nama -->
      <div class="attr-row">
        <span class="attr-label">Nama</span>
        <span class="attr-value">{mekaarStore.currentUser.displayName}</span>
        <button
          type="button"
          class="attr-edit-btn"
          onclick={() => openEditField('name', 'Nama Tampilan', mekaarStore.currentUser.displayName)}
          aria-label="Edit Nama"
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--text-secondary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
          </svg>
        </button>
      </div>

      <!-- Username -->
      <div class="attr-row">
        <span class="attr-label">Username</span>
        <span class="attr-value">{mekaarStore.currentUser.username}</span>
        <button
          type="button"
          class="attr-edit-btn"
          onclick={() => openEditField('username', 'Username', mekaarStore.currentUser.username)}
          aria-label="Edit Username"
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--text-secondary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
          </svg>
        </button>
      </div>

      <!-- Bio -->
      <div class="attr-row">
        <span class="attr-label">Bio</span>
        <span class="attr-value bio-val">{mekaarStore.currentUser.bio || 'Aktif di Mekaar Mesh Network'}</span>
        <button
          type="button"
          class="attr-edit-btn"
          onclick={() => openEditField('bio', 'Bio', mekaarStore.currentUser.bio || 'Aktif di Mekaar Mesh Network')}
          aria-label="Edit Bio"
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--text-secondary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
          </svg>
        </button>
      </div>

      <!-- Email / Alamat Jaringan Privat -->
      <div class="attr-row">
        <span class="attr-label">Alamat Node</span>
        <span class="attr-value" style="font-family: var(--font-mono); font-size: 0.85rem;">{mekaarStore.currentUser.username.replace('@', '')}@mesh.internal</span>
        <span class="attr-lock-icon" title="Identitas mesh terenkripsi E2EE">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--brand-teal)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
          </svg>
        </span>
      </div>
    </div>
  </Card>

  <!-- Section Keamanan Akun -->
  <div class="security-section">
    <span class="section-title">KEAMANAN AKUN</span>
    <Card withTicks={false} padding="16px">
      <div class="pin-security-row">
        <span class="pin-label">PIN Kunci</span>
        <div class="pin-badge-wrap">
          <span class="pin-active-badge">PIN Aktif</span>
        </div>
        <button
          type="button"
          class="change-pin-btn"
          onclick={() => (showChangePinModal = true)}
        >
          Ubah PIN
        </button>
      </div>
    </Card>
  </div>

  <!-- Logout Button -->
  <div class="logout-wrap">
    <button
      type="button"
      class="logout-btn"
      onclick={handleLogout}
    >
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
        <polyline points="16 17 21 12 16 7"></polyline>
        <line x1="21" y1="12" x2="9" y2="12"></line>
      </svg>
      <span>Keluar dari Akun</span>
    </button>
  </div>
</div>

<!-- Modal Edit Atribut -->
<Dialog
  open={showEditFieldModal}
  title="Edit {editFieldTitle}"
  onclose={() => (showEditFieldModal = false)}
>
  <div style="display: flex; flex-direction: column; gap: 14px;">
    <input
      type="text"
      class="sketch-input"
      bind:value={editFieldValue}
      onkeydown={(e) => e.key === 'Enter' && saveField()}
    />
    <div style="display: flex; justify-content: flex-end; gap: 8px;">
      <button type="button" class="btn-secondary" onclick={() => (showEditFieldModal = false)}>
        Batal
      </button>
      <button type="button" class="btn-primary" onclick={saveField}>
        Simpan
      </button>
    </div>
  </div>
</Dialog>

<!-- Modal Ubah PIN -->
<Dialog
  open={showChangePinModal}
  title="Ubah PIN Kunci 6 Digit"
  onclose={() => (showChangePinModal = false)}
>
  <div style="display: flex; flex-direction: column; gap: 14px;">
    <p style="font-size: 0.88rem; color: var(--text-secondary);">
      Masukkan 6 digit angka PIN baru Anda:
    </p>
    <input
      type="password"
      maxlength="6"
      class="sketch-input"
      style="text-align: center; letter-spacing: 0.3em; font-size: 1.3rem; font-weight: 800;"
      placeholder="••••••"
      bind:value={newPinInput}
    />
    <div style="display: flex; justify-content: flex-end; gap: 8px;">
      <button type="button" class="btn-secondary" onclick={() => (showChangePinModal = false)}>
        Batal
      </button>
      <button type="button" class="btn-primary" onclick={handleChangePin}>
        Perbarui PIN
      </button>
    </div>
  </div>
</Dialog>

<style>
  .profile-page-container {
    padding: 0 16px 24px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .profile-hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 12px 0 8px 0;
  }

  .change-photo-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    margin-top: 12px;
    padding: 4px 8px;
  }

  .change-photo-btn:active {
    opacity: 0.8;
  }

  .profile-rows-list {
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  .attr-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .attr-label {
    width: 90px;
    font-size: 0.95rem;
    color: var(--text-secondary);
  }

  .attr-value {
    flex: 1;
    font-size: 1.05rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .bio-val {
    font-weight: 500;
    font-size: 0.98rem;
  }

  .attr-edit-btn, .attr-lock-icon {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Section Keamanan Akun */
  .security-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 4px;
  }

  .section-title {
    font-size: 0.82rem;
    font-weight: 700;
    color: var(--text-secondary);
    letter-spacing: 0.5px;
    margin-left: 4px;
  }

  .pin-security-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .pin-label {
    font-size: 0.98rem;
    color: var(--text-secondary);
  }

  .pin-active-badge {
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    background-color: var(--safe-teal-bg);
    border: 1px solid var(--safe-teal-border);
    color: var(--safe-teal-ink);
    font-size: 0.8rem;
    font-weight: 700;
  }

  .change-pin-btn {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
  }

  .change-pin-btn:active {
    opacity: 0.8;
  }

  /* Logout */
  .logout-wrap {
    display: flex;
    justify-content: center;
    margin-top: 14px;
  }

  .logout-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: transparent;
    border: none;
    color: var(--sos-coral);
    font-size: 1rem;
    font-weight: 700;
    cursor: pointer;
    padding: 10px 20px;
    border-radius: var(--radius-pill);
  }

  .logout-btn:active {
    background-color: #FEE2E2;
  }
</style>
