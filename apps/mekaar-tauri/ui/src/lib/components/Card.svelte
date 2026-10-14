<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'default' | 'subtle' | 'elevated' | 'danger' | 'sos';
    padding?: 'none' | 'sm' | 'md' | 'lg';
    interactive?: boolean;
    onclick?: (e: MouseEvent) => void;
    children?: Snippet;
  }

  let {
    variant = 'default',
    padding = 'md',
    interactive = false,
    onclick,
    children,
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  class="mekaar-card variant-{variant} padding-{padding}"
  class:interactive
  role={interactive ? 'button' : undefined}
  tabindex={interactive ? 0 : undefined}
  {onclick}
  onkeydown={interactive && onclick ? (e) => { if (e.key === 'Enter' || e.key === ' ') onclick(e as unknown as MouseEvent); } : undefined}
>
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .mekaar-card {
    position: relative;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
    background-color: var(--surface-card);
    transition: transform var(--motion-fast), box-shadow var(--motion-fast), border-color var(--motion-fast);
    box-shadow: var(--shadow-soft);
    overflow: hidden;
  }

  /* --- Paddings --- */
  .padding-none {
    padding: 0;
  }
  .padding-sm {
    padding: var(--space-3);
  }
  .padding-md {
    padding: var(--space-4);
  }
  .padding-lg {
    padding: var(--space-6);
  }

  /* --- Variants --- */
  .variant-default {
    background-color: var(--surface-card);
  }

  .variant-subtle {
    background-color: var(--surface-subtle);
    border-color: transparent;
    box-shadow: none;
  }

  .variant-elevated {
    background-color: var(--surface-elevated);
    box-shadow: var(--shadow-elevated);
  }

  .variant-danger {
    background-color: var(--surface-card);
    border-color: var(--sos-coral);
    box-shadow: 0 4px 16px rgba(var(--sos-coral-rgb), 0.12);
  }

  .variant-sos {
    background-color: var(--surface-subtle);
    border: 1.5px solid var(--sos-coral);
    box-shadow: var(--shadow-elevated);
  }

  /* --- Interactive --- */
  .interactive {
    cursor: pointer;
  }

  .interactive:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-elevated);
    border-color: var(--border-focus);
  }

  .interactive:active {
    transform: translateY(0);
  }
</style>
