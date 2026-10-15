<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { MainTabType } from '../types';
  import { mekaarStore } from '../stores';
  import SosButton from './SosButton.svelte';
  import {
    ChatCircleDots,
    AddressBook,
    User,
    GearSix,
    Plus,
    ShieldCheck
  } from 'phosphor-svelte';

  interface Props {
    children?: Snippet;
    onNewChat?: () => void;
  }

  let { children, onNewChat }: Props = $props();

  const navTabs: { id: MainTabType; label: string; icon: string; badge?: () => number }[] = [
    {
      id: 'chat',
      label: 'Pesan',
      icon: 'chat',
      badge: () =>
        mekaarStore.conversations.reduce((acc, c) => acc + c.unreadCount, 0) +
        mekaarStore.chatRequests.filter((r) => r.status === 'pending').length
    },
    { id: 'contacts', label: 'Kontak', icon: 'contacts' },
    { id: 'profile', label: 'Profil', icon: 'profile' },
    { id: 'settings', label: 'Setelan', icon: 'settings' }
  ];

  function handleTabClick(tabId: MainTabType) {
    if (tabId !== 'chat') {
      mekaarStore.selectedConversationId = null;
    }
    mekaarStore.setTab(tabId);
  }

  function handleSosTrigger() {
    mekaarStore.setTab('sos');
  }

  function handleNewChatClick() {
    if (onNewChat) {
      onNewChat();
    } else {
      // Default: switch to contacts or open prompt
      mekaarStore.setTab('contacts');
    }
  }
</script>

<div
  class="scaffold-root"
  data-register={mekaarStore.activeRegister}
  data-theme={mekaarStore.themeMode}
>
  {#if mekaarStore.privacyBlurOnUnfocus && !mekaarStore.windowFocused}
    <div class="privacy-shield-overlay">
      <div class="clean-card" style="padding: 24px; text-align: center; max-width: 340px; display: flex; flex-direction: column; align-items: center;">
        <div style="margin-bottom: 12px; display: flex; justify-content: center;">
          <ShieldCheck size={48} weight="duotone" color="var(--brand-teal, #0D9488)" />
        </div>
        <h3 style="font-weight: 700; margin-bottom: 6px; color: var(--text-primary);">Layar Dilindungi</h3>
        <p style="font-size: 0.88rem; color: var(--text-secondary); line-height: 1.4;">
          Pratinjau pesan disembunyikan secara otomatis saat aplikasi tidak aktif.
        </p>
      </div>
    </div>
  {/if}

  <!-- Main Scrollable Content Area -->
  <main class="scaffold-body">
    {#if children}
      {@render children()}
    {/if}
  </main>

  <!-- Floating Action Buttons Row (SOS on Left, Blue FAB on Right) -->
  {#if mekaarStore.activeTab === 'chat' && !mekaarStore.selectedConversationId}
    <div class="floating-fab-row">
      <div class="fab-left">
        <SosButton size={72} onTrigger={handleSosTrigger} />
      </div>
      <div class="fab-right">
        <button
          type="button"
          class="fab-circle-blue"
          onclick={handleNewChatClick}
          aria-label="Mulai Pesan Baru"
        >
          <Plus size={24} weight="bold" />
        </button>
      </div>
    </div>
  {/if}

  <!-- Floating Frosted-Glass 4-Tab Pill Dock -->
  <nav class="floating-nav-dock" aria-label="Navigasi Utama Mekaar">
    {#each navTabs as tab}
      {@const count = tab.badge ? tab.badge() : 0}
      {@const isActive = mekaarStore.activeTab === tab.id}
      <button
        type="button"
        class="dock-tab-btn {isActive ? 'active' : ''}"
        onclick={() => handleTabClick(tab.id)}
      >
        <div class="dock-icon-wrap">
          {#if tab.icon === 'chat'}
            <ChatCircleDots size={22} weight={isActive ? "fill" : "regular"} />
          {:else if tab.icon === 'contacts'}
            <AddressBook size={22} weight={isActive ? "fill" : "regular"} />
          {:else if tab.icon === 'profile'}
            <User size={22} weight={isActive ? "fill" : "regular"} />
          {:else if tab.icon === 'settings'}
            <GearSix size={22} weight={isActive ? "fill" : "regular"} />
          {/if}

          {#if count > 0}
            <span class="dock-badge">{count > 99 ? '99+' : count}</span>
          {/if}
        </div>
        <span class="dock-label">{tab.label}</span>
      </button>
    {/each}
  </nav>
</div>

<style>
  .scaffold-root {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background: var(--bg-primary);
    color: var(--text-primary);
    position: relative;
    padding-top: env(safe-area-inset-top, 0px);
    box-sizing: border-box;
    overflow-x: hidden;
  }

  .scaffold-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding-bottom: calc(100px + env(safe-area-inset-bottom, 0px));
  }

  /* Floating Action Buttons: SOS on left (72px), Blue FAB on right (56px) */
  .floating-fab-row {
    position: fixed;
    left: 20px;
    right: 20px;
    bottom: calc(96px + env(safe-area-inset-bottom, 0px));
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    pointer-events: none;
    z-index: 45;
  }

  .fab-left, .fab-right {
    pointer-events: auto;
  }

  .fab-circle-blue {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background-color: var(--brand-blue);
    color: #FFFFFF;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 6px 18px rgba(19, 108, 252, 0.38);
    transition: transform 0.15s ease, background-color 0.15s ease;
  }

  .fab-circle-blue:active {
    transform: scale(0.92);
    background-color: var(--brand-blue-hover);
  }

  /* Floating Frosted-Glass 4-Tab Pill Dock */
  .floating-nav-dock {
    position: fixed;
    bottom: calc(16px + env(safe-area-inset-bottom, 0px));
    left: 50%;
    transform: translateX(-50%);
    width: min(320px, calc(100vw - 32px));
    height: 68px;
    background: rgba(255, 255, 255, 0.85);
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    border: 1px solid rgba(0, 0, 0, 0.07);
    border-radius: 100px;
    display: flex;
    align-items: center;
    justify-content: space-around;
    padding: 0 10px;
    box-shadow: 0 6px 24px rgba(21, 38, 65, 0.1);
    z-index: 50;
    box-sizing: border-box;
  }

  :global([data-theme="dark"]) .floating-nav-dock {
    background: rgba(30, 39, 54, 0.88);
    border-color: rgba(255, 255, 255, 0.1);
    box-shadow: 0 8px 28px rgba(0, 0, 0, 0.35);
  }

  .dock-tab-btn {
    flex: 1;
    height: 100%;
    background: transparent;
    border: none;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 3px;
    color: var(--text-secondary);
    transition: all 0.16s ease;
    border-radius: 30px;
    padding: 4px 0;
  }

  .dock-tab-btn:active {
    transform: scale(0.92);
  }

  .dock-tab-btn.active {
    color: var(--brand-blue);
  }

  .dock-tab-btn.active .dock-label {
    font-weight: 700;
    color: var(--brand-blue);
  }

  .dock-icon-wrap {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
  }

  .dock-label {
    font-size: 0.72rem;
    font-weight: 500;
    letter-spacing: 0.2px;
  }

  .dock-badge {
    position: absolute;
    top: -4px;
    right: -8px;
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.65rem;
    font-weight: 700;
    padding: 1px 5px;
    border-radius: 10px;
    min-width: 14px;
    text-align: center;
    line-height: 1.2;
    box-shadow: 0 1px 3px rgba(19, 108, 252, 0.4);
  }

  .privacy-shield-overlay {
    position: fixed;
    inset: 0;
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    background: rgba(15, 20, 28, 0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
  }
</style>
