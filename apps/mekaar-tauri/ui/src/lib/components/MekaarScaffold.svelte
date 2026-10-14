<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { MainTabType } from '../types';
  import { mekaarStore } from '../stores';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();

  const navItems: { id: MainTabType; label: string; icon: string; badge?: () => number }[] = [
    {
      id: 'chat',
      label: 'Obrolan',
      icon: '💬',
      badge: () =>
        mekaarStore.conversations.reduce((acc, c) => acc + c.unreadCount, 0) +
        mekaarStore.chatRequests.filter((r) => r.status === 'pending').length
    },
    { id: 'map', label: 'Peta', icon: '🗺️' },
    { id: 'guardian', label: 'Guardian', icon: '🛡️' },
    { id: 'sos', label: 'SOS', icon: '🚨' },
    { id: 'settings', label: 'Pengaturan', icon: '⚙️' }
  ];
</script>

<div
  class="scaffold-root"
  data-register={mekaarStore.activeRegister}
  data-theme={mekaarStore.themeMode}
  style="--font-scale: {mekaarStore.fontScale}; font-family: '{mekaarStore.fontFamily}', sans-serif;"
>
  {#if mekaarStore.privacyBlurOnUnfocus && !mekaarStore.windowFocused}
    <div class="privacy-shield-overlay">
      <div class="sketch-card" style="padding: 24px; text-align: center; max-width: 360px;">
        <div style="font-size: 2.2rem; margin-bottom: 8px;">🛡️</div>
        <h3 style="font-family: var(--font-crayon); margin-bottom: 6px;">Layar Dilindungi (Screen Shield)</h3>
        <p style="font-size: 0.86rem; color: var(--text-secondary);">
          Pratinjau pesan disembunyikan secara otomatis saat jendela aplikasi tidak aktif.
        </p>
      </div>
    </div>
  {/if}

  <main class="scaffold-body">
    {#if children}
      {@render children()}
    {/if}
  </main>

  <nav class="bottom-nav" aria-label="Navigasi Utama Mekaar">
    {#each navItems as item}
      {@const count = item.badge ? item.badge() : 0}
      <button
        type="button"
        class="nav-tab {mekaarStore.activeTab === item.id ? 'active' : ''} {item.id === 'sos' ? 'sos-tab' : ''}"
        onclick={() => {
          if (item.id !== 'chat') {
            mekaarStore.selectedConversationId = null;
          }
          mekaarStore.setTab(item.id);
        }}
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{item.label}</span>
        {#if count > 0}
          <span class="nav-badge">{count}</span>
        {/if}
      </button>
    {/each}
  </nav>
</div>

<style>
  .scaffold-root {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background: var(--register-bg);
    color: var(--text-primary);
    position: relative;
    padding-top: env(safe-area-inset-top, 0px);
    box-sizing: border-box;
  }
  .scaffold-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding-bottom: calc(74px + env(safe-area-inset-bottom, 0px));
  }
  .bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: calc(68px + env(safe-area-inset-bottom, 0px));
    background: var(--bg-surface);
    border-top: 2.5px solid var(--sketch-border-color);
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    padding: 6px 10px calc(6px + env(safe-area-inset-bottom, 0px));
    gap: 6px;
    z-index: 40;
    box-sizing: border-box;
  }
  .nav-tab {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
    background: transparent;
    border: 2px solid transparent;
    border-radius: 12px;
    cursor: pointer;
    font-family: var(--font-sans);
    color: var(--text-secondary);
    transition: all 0.12s ease;
  }
  .nav-tab.active {
    background: var(--accent-secondary);
    color: var(--text-primary);
    border-color: var(--sketch-border-color);
    box-shadow: var(--sketch-shadow-sm);
    font-weight: 700;
  }
  .nav-tab.sos-tab.active {
    background: #FFE4E6;
    color: #9F1239;
  }
  .nav-icon {
    font-size: 1.15rem;
  }
  .nav-label {
    font-size: 0.74rem;
  }
  .nav-badge {
    position: absolute;
    top: 3px;
    right: calc(50% - 22px);
    background: var(--accent-danger);
    color: #FFFFFF;
    font-size: 0.68rem;
    font-weight: 800;
    padding: 0 6px;
    border-radius: 999px;
    border: 1.5px solid var(--sketch-border-color);
  }
  .privacy-shield-overlay {
    position: fixed;
    inset: 0;
    backdrop-filter: blur(14px);
    background: rgba(15, 20, 28, 0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
  }
</style>
