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
  let open = $state(defaultOpen);
</script>

<div class="sketch-card" style="padding: 14px; margin-bottom: 12px;">
  <button
    type="button"
    onclick={() => (open = !open)}
    style="width: 100%; display: flex; align-items: center; justify-content: space-between; background: none; border: none; color: inherit; cursor: pointer; text-align: left;"
  >
    <div>
      <div style="font-weight: 700; font-size: 0.98rem;">{title}</div>
      {#if subtitle}
        <div style="font-size: 0.82rem; color: var(--text-secondary);">{subtitle}</div>
      {/if}
    </div>
    <div style="display: flex; align-items: center; gap: 8px;">
      {#if badgeText}
        <span class="sketch-badge" style="background: var(--accent-secondary);">{badgeText}</span>
      {/if}
      <span style="font-weight: 700;">{open ? '▲' : '▼'}</span>
    </div>
  </button>
  {#if open && children}
    <div style="margin-top: 12px; padding-top: 12px; border-top: 1.5px dashed var(--sketch-border-color);">
      {@render children()}
    </div>
  {/if}
</div>
