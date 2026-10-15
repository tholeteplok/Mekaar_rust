<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    subtitle?: string;
    badgeText?: string;
    defaultOpen?: boolean;
    children?: Snippet;
  }

  let { title, subtitle = '', badgeText = '', defaultOpen = false, children }: Props = $props();
  let open = $state(false);
  $effect(() => {
    open = defaultOpen;
  });
</script>

<div class="clean-card" style="padding: 16px; margin-bottom: 12px;">
  <button
    type="button"
    onclick={() => (open = !open)}
    class="card-header-btn"
  >
    <div>
      <div class="card-title">{title}</div>
      {#if subtitle}
        <div class="card-subtitle">{subtitle}</div>
      {/if}
    </div>
    <div style="display: flex; align-items: center; gap: 8px;">
      {#if badgeText}
        <span class="aegis-badge">{badgeText}</span>
      {/if}
      <span class="expand-caret {open ? 'open' : ''}">▼</span>
    </div>
  </button>
  {#if open && children}
    <div class="card-expand-body">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .clean-card {
    background: var(--bg-surface);
    border-radius: var(--radius-card);
    border: 0.75px solid var(--border-subtle);
    box-shadow: var(--shadow-card);
    position: relative;
    box-sizing: border-box;
  }


  .card-header-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    text-align: left;
    padding: 0;
  }

  .card-title {
    font-weight: 700;
    font-size: 0.98rem;
    color: var(--text-primary);
  }

  .card-subtitle {
    font-size: 0.82rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .expand-caret {
    font-size: 0.75rem;
    color: var(--text-secondary);
    transition: transform 0.2s ease;
  }

  .expand-caret.open {
    transform: rotate(180deg);
  }

  .card-expand-body {
    margin-top: 14px;
    padding-top: 14px;
    border-top: 1px solid var(--border-light);
  }
</style>
