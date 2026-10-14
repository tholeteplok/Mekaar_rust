<script lang="ts">
  import type { Snippet } from 'svelte';
  import { mekaarStore } from '../stores';

  interface Props {
    title: string;
    subtitle?: string;
    showBack?: boolean;
    onback?: () => void;
    actions?: Snippet;
  }

  let { title, subtitle = '', showBack = false, onback, actions }: Props = $props();
</script>

<header class="mekaar-appbar">
  <div style="display: flex; align-items: center; gap: 10px;">
    {#if showBack}
      <button type="button" class="back-btn" onclick={onback} title="Kembali">←</button>
    {/if}
    <div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <h1 class="appbar-title">{title}</h1>
        <span
          class="sketch-badge"
          style="background: {mekaarStore.serverStatus.online ? '#DCFCE7' : '#FEF3C7'}; color: #1C1B18;"
          title="Endpoint: {mekaarStore.serverStatus.wsEndpoint}"
        >
          <span
            style="width: 7px; height: 7px; border-radius: 99px; background: {mekaarStore.serverStatus.online ? '#10B981' : '#F59E0B'};"
          ></span>
          {mekaarStore.serverStatus.mode === 'tailscale' ? 'Tailscale 100.101.96.120:8085' : 'Local 127.0.0.1:8085'}
        </span>
      </div>
      {#if subtitle}
        <p class="appbar-sub">{subtitle}</p>
      {/if}
    </div>
  </div>

  <div style="display: flex; align-items: center; gap: 8px;">
    {#if actions}
      {@render actions()}
    {/if}
  </div>
</header>

<style>
  .mekaar-appbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 18px;
    background: var(--register-header);
    border-bottom: 2px solid var(--sketch-border-color);
    position: sticky;
    top: 0;
    z-index: 30;
  }
  .appbar-title {
    font-family: var(--font-crayon);
    font-size: 1.15rem;
    font-weight: 700;
  }
  .appbar-sub {
    font-size: 0.78rem;
    color: var(--text-secondary);
  }
  .back-btn {
    background: var(--bg-secondary);
    border: 2px solid var(--sketch-border-color);
    border-radius: 10px;
    width: 36px;
    height: 36px;
    font-weight: 800;
    cursor: pointer;
    box-shadow: var(--sketch-shadow-sm);
  }
</style>
