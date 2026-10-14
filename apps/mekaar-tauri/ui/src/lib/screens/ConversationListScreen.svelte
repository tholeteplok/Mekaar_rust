<script lang="ts">
  import { mekaarStore } from '../stores';
  import AppBar from '../components/AppBar.svelte';
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import Dialog from '../components/Dialog.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  let subTab = $state<'chats' | 'requests' | 'contacts'>('chats');
  let searchQuery = $state('');
  let showCreateGroup = $state(false);
  let showQrModal = $state(false);
  let qrMode = $state<'my_qr' | 'scan_qr'>('my_qr');
  let scannedPeerInput = $state('');

  let groupTitle = $state('');
  let selectedMembers = $state<string[]>([]);

  const filteredConversations = $derived(
    mekaarStore.conversations.filter(
      (c) =>
        c.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.subtitle.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  function toggleMember(id: string) {
    if (selectedMembers.includes(id)) {
      selectedMembers = selectedMembers.filter((m) => m !== id);
    } else {
      selectedMembers = [...selectedMembers, id];
    }
  }

  function handleCreateGroup() {
    if (!groupTitle.trim()) return;
    const newId = mekaarStore.createGroup(groupTitle.trim(), selectedMembers);
    groupTitle = '';
    selectedMembers = [];
    showCreateGroup = false;
    mekaarStore.openConversation(newId);
  }

  function handleAddContactByQr() {
    if (!scannedPeerInput.trim()) return;
    const clean = scannedPeerInput.trim();
    mekaarStore.contacts = [
      {
        id: `usr-qr-${Date.now()}`,
        username: clean.startsWith('@') ? clean : `@${clean.toLowerCase().replace(/\s+/g, '')}`,
        displayName: clean,
        avatarColor: '#DDD6FE',
        fingerprint: 'ED25519: QR-VERIFIED-8821',
        isOnline: true,
        isGuardian: false,
        isBlocked: false,
        notes: 'Ditambahkan melalui pemindaian QR E2EE'
      },
      ...mekaarStore.contacts
    ];
    scannedPeerInput = '';
    showQrModal = false;
    subTab = 'contacts';
  }
</script>

<AppBar
  title="Mekaar Chat"
  subtitle="P2P & Relay E2EE • {mekaarStore.serverStatus.wsEndpoint}"
>
  {#snippet actions()}
    <Button size="sm" variant="secondary" onclick={() => (showQrModal = true)}>📷 Kode QR</Button>
    <Button size="sm" variant="primary" onclick={() => (showCreateGroup = true)}>+ Grup Baru</Button>
  {/snippet}
</AppBar>

<div class="screen-container">
  <!-- Bilah Pencarian & Filter Sub-Tab (Chats / Permintaan / Kontak) -->
  <div style="display: flex; gap: 10px; margin-bottom: 12px; flex-wrap: wrap;">
    <input
      type="text"
      class="sketch-input"
      style="flex: 1; min-width: 200px;"
      placeholder="🔍 Cari percakapan, grup, atau kontak..."
      bind:value={searchQuery}
    />
    <div style="display: flex; gap: 6px;">
      <Button
        size="sm"
        variant={subTab === 'chats' ? 'primary' : 'secondary'}
        onclick={() => (subTab = 'chats')}
      >
        💬 Chat ({mekaarStore.conversations.length})
      </Button>
      <Button
        size="sm"
        variant={subTab === 'requests' ? 'primary' : 'secondary'}
        onclick={() => (subTab = 'requests')}
      >
        📩 Permintaan ({mekaarStore.chatRequests.filter((r) => r.status === 'pending').length})
      </Button>
      <Button
        size="sm"
        variant={subTab === 'contacts' ? 'primary' : 'secondary'}
        onclick={() => (subTab = 'contacts')}
      >
        👥 Kontak ({mekaarStore.contacts.length})
      </Button>
    </div>
  </div>

  {#if subTab === 'chats'}
    <div style="display: flex; flex-direction: column; gap: 10px;">
      {#each filteredConversations as conv (conv.id)}
        <Card interactive onclick={() => mekaarStore.openConversation(conv.id)}>
          <div style="display: flex; align-items: center; gap: 12px;">
            <SketchAvatar
              name={conv.title}
              color={conv.avatarColor}
              online={conv.isOnline}
              isGroup={conv.isGroup}
            />
            <div style="flex: 1; min-width: 0;">
              <div style="display: flex; align-items: center; justify-content: space-between;">
                <div style="display: flex; align-items: center; gap: 6px;">
                  {#if conv.isPinned}<span>📌</span>{/if}
                  <span style="font-weight: 700; font-size: 0.98rem;">{conv.title}</span>
                  {#if conv.disappearingSeconds > 0}
                    <span class="sketch-badge" style="background: #FEF3C7;">⏱️ Auto-Hapus</span>
                  {/if}
                </div>
                <span style="font-size: 0.76rem; color: var(--text-muted);">{conv.lastTimestamp}</span>
              </div>
              <div style="display: flex; align-items: center; justify-content: space-between; margin-top: 3px;">
                <p
                  style="font-size: 0.85rem; color: {conv.isTyping ? 'var(--accent-primary)' : 'var(--text-secondary)'}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;"
                >
                  {conv.isTyping ? '✍️ Sedang mengetik...' : conv.subtitle}
                </p>
                {#if conv.unreadCount > 0}
                  <span class="unread-pill">{conv.unreadCount}</span>
                {/if}
              </div>
            </div>
          </div>
        </Card>
      {/each}
    </div>
  {:else if subTab === 'requests'}
    <div style="display: flex; flex-direction: column; gap: 10px;">
      {#each mekaarStore.chatRequests as req (req.id)}
        <Card>
          <div style="display: flex; align-items: flex-start; gap: 12px;">
            <SketchAvatar name={req.fromDisplayName} color={req.avatarColor} />
            <div style="flex: 1;">
              <div style="display: flex; justify-content: space-between;">
                <div>
                  <strong>{req.fromDisplayName}</strong>
                  <span style="font-size: 0.8rem; color: var(--text-secondary); margin-left: 6px;">{req.fromUsername}</span>
                </div>
                <span style="font-size: 0.75rem; color: var(--text-muted);">{req.timestamp}</span>
              </div>
              <p style="font-size: 0.86rem; margin: 6px 0;">"{req.introMessage}"</p>
              <div style="font-size: 0.74rem; font-family: var(--font-mono); color: var(--text-secondary); margin-bottom: 10px;">
                🔑 {req.fingerprint}
              </div>
              {#if req.status === 'pending'}
                <div style="display: flex; gap: 8px;">
                  <Button size="sm" variant="primary" onclick={() => mekaarStore.respondChatRequest(req.id, true)}>
                    ✓ Terima & Buka Chat
                  </Button>
                  <Button size="sm" variant="danger" onclick={() => mekaarStore.respondChatRequest(req.id, false)}>
                    ✕ Tolak
                  </Button>
                </div>
              {:else}
                <span class="sketch-badge" style="background: {req.status === 'accepted' ? '#DCFCE7' : '#FEE2E2'};">
                  {req.status === 'accepted' ? 'Diterima' : 'Ditolak'}
                </span>
              {/if}
            </div>
          </div>
        </Card>
      {/each}
    </div>
  {:else}
    <div style="display: flex; flex-direction: column; gap: 10px;">
      {#each mekaarStore.contacts as contact (contact.id)}
        <Card>
          <div style="display: flex; align-items: center; justify-content: space-between; gap: 12px;">
            <div style="display: flex; align-items: center; gap: 12px;">
              <SketchAvatar name={contact.displayName} color={contact.avatarColor} online={contact.isOnline} />
              <div>
                <div style="display: flex; align-items: center; gap: 6px;">
                  <strong>{contact.displayName}</strong>
                  <span style="font-size: 0.8rem; color: var(--text-secondary);">{contact.username}</span>
                  {#if contact.isGuardian}
                    <span class="sketch-badge" style="background: #DBEAFE;">🛡️ Guardian</span>
                  {/if}
                </div>
                <div style="font-size: 0.8rem; color: var(--text-secondary);">{contact.notes}</div>
                <div style="font-size: 0.72rem; font-family: var(--font-mono); color: var(--text-muted);">
                  {contact.fingerprint}
                </div>
              </div>
            </div>
            <Button
              size="sm"
              variant="secondary"
              onclick={() => {
                const existing = mekaarStore.conversations.find((c) => c.title === contact.displayName);
                if (existing) {
                  mekaarStore.openConversation(existing.id);
                }
              }}
            >
              💬 Kirim Pesan
            </Button>
          </div>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<!-- Modal Buat Grup Baru (create_group_select_members_screen & create_group_details_screen parity) -->
<Dialog open={showCreateGroup} title="👥 Buat Grup Terenkripsi Baru" onclose={() => (showCreateGroup = false)}>
  <div style="display: flex; flex-direction: column; gap: 12px;">
    <label style="font-size: 0.85rem; font-weight: 700;">
      Nama Grup
      <input
        type="text"
        class="sketch-input"
        placeholder="Mis. Tim Siaga Surakarta..."
        bind:value={groupTitle}
        style="margin-top: 6px;"
      />
    </label>
    <div style="font-size: 0.85rem; font-weight: 700;">Pilih Anggota Grup:</div>
    <div style="display: flex; flex-direction: column; gap: 8px; max-height: 200px; overflow-y: auto;">
      {#each mekaarStore.contacts as c}
        <label class="sketch-card" style="padding: 8px 12px; display: flex; align-items: center; justify-content: space-between; cursor: pointer;">
          <span><strong>{c.displayName}</strong> ({c.username})</span>
          <input
            type="checkbox"
            checked={selectedMembers.includes(c.id)}
            onchange={() => toggleMember(c.id)}
          />
        </label>
      {/each}
    </div>
    <div style="display: flex; justify-content: flex-end; gap: 8px;">
      <Button variant="secondary" onclick={() => (showCreateGroup = false)}>Batal</Button>
      <Button variant="primary" onclick={handleCreateGroup}>Buat Grup ({selectedMembers.length + 1} Anggota)</Button>
    </div>
  </div>
</Dialog>

<!-- Modal QR Saya & Pindai QR Kontak (my_qr_screen.dart & contact_qr_scan_screen.dart parity) -->
<Dialog open={showQrModal} title="📷 Kode QR Identitas & Pemindai Kontak" onclose={() => (showQrModal = false)}>
  <div style="display: flex; flex-direction: column; gap: 12px;">
    <div style="display: flex; gap: 8px;">
      <Button
        size="sm"
        fullWidth
        variant={qrMode === 'my_qr' ? 'primary' : 'secondary'}
        onclick={() => (qrMode = 'my_qr')}
      >
        QR Saya
      </Button>
      <Button
        size="sm"
        fullWidth
        variant={qrMode === 'scan_qr' ? 'primary' : 'secondary'}
        onclick={() => (qrMode = 'scan_qr')}
      >
        Pindai QR Teman
      </Button>
    </div>

    {#if qrMode === 'my_qr'}
      <div class="sketch-card" style="padding: 18px; text-align: center; background: #FFFDF9;">
        <div class="qr-matrix-box">
          <div style="font-family: var(--font-mono); font-size: 0.72rem; line-height: 1.15; letter-spacing: 2px;">
            ▛▀▀▀▀▀▀▀▀▀▀▀▜<br />
            ▌ ██ ▄▀▄ ██ ▐<br />
            ▌ ▀▀ █▄█ ▀▀ ▐<br />
            ▌ ▄▀▄ ▀ ▄▀▄ ▐<br />
            ▌ ██ ▀▄▀ ██ ▐<br />
            ▙▄▄▄▄▄▄▄▄▄▄▄▟
          </div>
        </div>
        <div style="font-weight: 800; margin-top: 10px;">{mekaarStore.currentUser.displayName}</div>
        <div style="font-size: 0.82rem; color: var(--text-secondary);">{mekaarStore.currentUser.username}</div>
        <div style="font-size: 0.72rem; font-family: var(--font-mono); margin-top: 6px;">
          {mekaarStore.currentUser.publicKeyFingerprint}
        </div>
      </div>
    {:else}
      <div style="display: flex; flex-direction: column; gap: 10px;">
        <p style="font-size: 0.85rem; color: var(--text-secondary);">
          Tempel payload QR atau nama pengguna rekan untuk memverifikasi kunci Ed25519 secara langsung:
        </p>
        <input
          type="text"
          class="sketch-input"
          placeholder="Mis. @andikawijaya atau mekaar://qr/ed25519..."
          bind:value={scannedPeerInput}
        />
        <Button variant="primary" fullWidth onclick={handleAddContactByQr}>Verifikasi & Tambahkan Kontak</Button>
      </div>
    {/if}
  </div>
</Dialog>

<style>
  .screen-container {
    padding: 16px 18px;
    max-width: 880px;
    width: 100%;
    margin: 0 auto;
  }
  .unread-pill {
    background: var(--accent-primary);
    color: #FFFFFF;
    font-size: 0.74rem;
    font-weight: 800;
    padding: 1px 8px;
    border-radius: 999px;
    border: 1.5px solid var(--sketch-border-color);
  }
  .qr-matrix-box {
    display: inline-block;
    padding: 16px;
    background: #FFFFFF;
    border: 2.5px solid #1C1B18;
    border-radius: 14px;
    box-shadow: var(--sketch-shadow-sm);
  }
</style>
