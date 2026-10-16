<script lang="ts">
  import type { Snippet } from 'svelte';
  import { mekaarStore } from '../stores';

  interface Props {
    title: string;
    subtitle?: string;
    showBack?: boolean;
    showAegisBadge?: boolean;
    onback?: () => void;
    actions?: Snippet;
  }

  let {
    title,
    subtitle = '',
    showBack = false,
    showAegisBadge = false,
    onback,
    actions
  }: Props = $props();
</script>

<header class="clean-appbar">
  <div class="appbar-left">
    {#if showBack}
      <button type="button" class="back-circle-btn" onclick={onback} aria-label="Kembali">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <line x1="19" y1="12" x2="5" y2="12"></line>
          <polyline points="12 19 5 12 12 5"></polyline>
        </svg>
      </button>
    {/if}
    <div class="title-col">
      <h1 class="appbar-title">{title}</h1>
      {#if subtitle}
        <span class="appbar-subtitle">{subtitle}</span>
      {/if}
    </div>
  </div>

  <div class="appbar-right">
    {#if showAegisBadge}
      <div class="aegis-badge" title="Terenkripsi E2EE • Mekaar Mesh Network">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
        </svg>
        <span>Aegis E2EE</span>
      </div>
    {/if}

    {#if actions}
      {@render actions()}
    {/if}
  </div>
</header>

<style>
  .clean-appbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: calc(12px + env(safe-area-inset-top, 0px)) 20px 10px 20px;
    background-color: var(--bg-primary);
    position: sticky;
    top: 0;
    z-index: 30;
    box-sizing: border-box;
  }

  .appbar-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .title-col {
    display: flex;
    flex-direction: column;
  }

  .appbar-title {
    font-family: var(--font-active);
    font-size: 1.55rem;
    font-weight: 800;
    color: var(--text-primary);
    letter-spacing: -0.3px;
    line-height: 1.2;
  }

  .appbar-subtitle {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin-top: 1px;
  }

  .appbar-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .aegis-badge {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 12px;
    border-radius: var(--radius-pill);
    background-color: var(--safe-teal-bg);
    border: 1px solid var(--safe-teal-border);
    color: var(--safe-teal-ink);
    font-size: 0.78rem;
    font-weight: 700;
    box-shadow: 0 1px 4px rgba(45, 212, 191, 0.15);
  }

  .back-circle-btn {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: #FFFFFF;
    border: 1px solid var(--border-light);
    color: var(--text-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(21, 38, 65, 0.06);
    transition: transform 0.12s ease;
  }

  .back-circle-btn:active {
    transform: scale(0.92);
  }
</style>
