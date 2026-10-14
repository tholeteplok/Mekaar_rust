<script lang="ts">
  import {
    MekaarScaffold,
    AppBar,
    Button,
    Card,
    CardExpanded,
    Dialog,
    BottomSheet,
    HoldToConfirmButton,
  } from './lib/components';

  import {
    VaultScreen,
    ConversationListScreen,
    ChatRoomScreen,
    SosScreen,
  } from './lib/screens';

  import { vaultStore, chatStore } from './lib/stores';
  import type { ScreenType, TriRegister } from './lib/types';

  // Navigation and view state
  let currentScreen = $state<ScreenType | 'gallery'>('vault');
  let currentTheme = $state<'light' | 'dark'>('light');
  let activeRoomId = $state<string>('room_guard_1');

  // Component gallery demo state
  let showGalleryDialog = $state(false);
  let showGallerySheet = $state(false);
  let galleryAccordionOpen = $state(false);

  // Derive register from current screen
  const currentRegister = $derived<TriRegister>(
    currentScreen === 'sos' ? 'sos' : currentScreen === 'chat' ? 'room' : 'core'
  );

  function toggleTheme() {
    currentTheme = currentTheme === 'light' ? 'dark' : 'light';
  }

  function handleUnlocked() {
    currentScreen = 'conversations';
  }

  function handleSelectRoom(roomId: string) {
    activeRoomId = roomId;
    currentScreen = 'chat';
  }

  function handleOpenSos() {
    currentScreen = 'sos';
  }

  function handleLock() {
    vaultStore.lock();
    currentScreen = 'vault';
  }

  function handleBackToConversations() {
    currentScreen = 'conversations';
  }
</script>

<MekaarScaffold theme={currentTheme} register={currentRegister}>
  <!-- Top Global Quick Bar (Screen & Theme Switcher) — Only visible when unlocked -->
  {#if currentScreen !== 'vault'}
    <div class="quick-nav-bar">
      <div class="nav-pills">
        <button
          type="button"
          class="nav-pill"
          class:active={currentScreen === 'conversations'}
          onclick={() => currentScreen = 'conversations'}
        >
          💬 Obrolan
        </button>
        <button
          type="button"
          class="nav-pill"
          class:active={currentScreen === 'chat'}
          onclick={() => { activeRoomId = 'room_guard_1'; currentScreen = 'chat'; }}
        >
          ✉️ Chat E2EE
        </button>
        <button
          type="button"
          class="nav-pill pill-sos"
          class:active={currentScreen === 'sos'}
          onclick={() => currentScreen = 'sos'}
        >
          🚨 SOS
        </button>
        <button
          type="button"
          class="nav-pill"
          class:active={currentScreen === 'gallery'}
          onclick={() => currentScreen = 'gallery'}
        >
          🎨 Gallery
        </button>
        <button
          type="button"
          class="nav-pill"
          onclick={handleLock}
        >
          🔒 Kunci
        </button>
      </div>

      <div class="nav-actions">
        <button
          type="button"
          class="theme-toggle-btn"
          aria-label="Ganti Tema"
          onclick={toggleTheme}
        >
          {currentTheme === 'light' ? '🌙 Dark' : '☀️ Light'}
        </button>
      </div>
    </div>
  {/if}

  <!-- Screen Routing Views -->
  {#if currentScreen === 'vault'}
    <VaultScreen onunlocked={handleUnlocked} />
  {:else if currentScreen === 'conversations'}
    <ConversationListScreen
      onselectroom={handleSelectRoom}
      onopensos={handleOpenSos}
      onlock={handleLock}
    />
  {:else if currentScreen === 'chat'}
    <ChatRoomScreen
      roomId={activeRoomId}
      onback={handleBackToConversations}
      onopensos={handleOpenSos}
    />
  {:else if currentScreen === 'sos'}
    <SosScreen onback={handleBackToConversations} />
  {:else if currentScreen === 'gallery'}
    <!-- Gallery Screen for Centralized Components Testing -->
    <div class="gallery-wrapper">
      <AppBar
        title="Pustaka Komponen Tersentralisasi"
        subtitle="Zero-Hardcoded • Single Source of Truth"
        showBack
        onback={() => currentScreen = 'conversations'}
      />

      <div class="gallery-body">
        <section class="gallery-section">
          <h2 class="section-title">Kartu & Akordeon Forensik</h2>
          <div class="card-grid">
            <Card variant="default">
              <h3>Surface Card Default</h3>
              <p>Warna adaptif light/dark dengan bayangan lembut.</p>
            </Card>
            <Card variant="elevated" interactive>
              <h3>Elevated Interactive</h3>
              <p>Sentuh atau arahkan kursor untuk efek elevasi halus.</p>
            </Card>
          </div>

          <CardExpanded
            title="Bukti Forensik Kriptografis"
            subtitle="SHA-256 Checksum • Anti-Tamper"
            badge="Terverifikasi"
            badgeVariant="green"
            bind:expanded={galleryAccordionOpen}
          >
            <p>Hash: <code>e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855</code></p>
          </CardExpanded>
        </section>

        <section class="gallery-section">
          <h2 class="section-title">Tombol & Modal</h2>
          <div class="btn-group">
            <Button variant="primary">Primary</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="danger">Danger</Button>
            <Button variant="secondary" onclick={() => showGalleryDialog = true}>Buka Dialog</Button>
            <Button variant="secondary" onclick={() => showGallerySheet = true}>Buka Sheet</Button>
          </div>
        </section>

        <section class="gallery-section">
          <h2 class="section-title">Pemicu Panik 76px</h2>
          <HoldToConfirmButton
            durationMs={2000}
            label="SOS"
            sublabel="Tahan 2 Detik"
            onconfirm={() => alert('SOS Dikonfirmasi!')}
          />
        </section>
      </div>

      <Dialog
        bind:open={showGalleryDialog}
        title="Modal Tersentralisasi"
        description="Dialog modal dengan backdrop blur otomatis."
      >
        <p>Konten modal tersentralisasi dan bebas dari style inline hardcoded.</p>
        {#snippet actions()}
          <Button variant="primary" onclick={() => showGalleryDialog = false}>Tutup</Button>
        {/snippet}
      </Dialog>

      <BottomSheet
        bind:open={showGallerySheet}
        title="Bottom Sheet Aksi"
      >
        <p>Bottom sheet responsif untuk aksi cepat pada layar mobile & desktop.</p>
      </BottomSheet>
    </div>
  {/if}
</MekaarScaffold>

<style>
  .quick-nav-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) var(--space-4);
    background-color: var(--surface-card);
    border-bottom: 1px solid var(--border-subtle);
    gap: var(--space-2);
    overflow-x: auto;
    position: sticky;
    top: 0;
    z-index: 60;
  }

  .nav-pills {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .nav-pill {
    padding: 4px 12px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background: transparent;
    color: var(--text-secondary);
    font-size: var(--font-caption-size);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--motion-fast);
    white-space: nowrap;
  }

  .nav-pill:hover {
    border-color: var(--brand-blue);
    color: var(--text-primary);
  }

  .nav-pill.active {
    background-color: var(--brand-blue);
    border-color: var(--brand-blue);
    color: #FFFFFF;
  }

  .nav-pill.pill-sos.active {
    background-color: var(--sos-coral);
    border-color: var(--sos-coral);
    color: #FFFFFF;
  }

  .nav-actions {
    display: flex;
    align-items: center;
  }

  .theme-toggle-btn {
    padding: 4px 10px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-subtle);
    background-color: var(--surface-subtle);
    color: var(--text-primary);
    font-size: var(--font-caption-size);
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .gallery-wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .gallery-body {
    max-width: 800px;
    width: 100%;
    margin: 0 auto;
    padding: var(--space-6) var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .gallery-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .section-title {
    font-size: var(--font-h2-size);
    font-weight: 700;
    color: var(--text-primary);
  }

  .card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-4);
  }

  .btn-group {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
  }
</style>
