<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    fullWidth?: boolean;
    onclick?: () => void;
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    fullWidth = false,
    onclick,
    children
  }: Props = $props();
</script>

<button
  type="button"
  class="mekaar-btn {variant} {size} {fullWidth ? 'full' : ''}"
  {disabled}
  {onclick}
>
  {#if children}
    {@render children()}
  {/if}
</button>

<style>
  .mekaar-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-family: var(--font-sans);
    font-weight: 700;
    border: 2px solid var(--sketch-border-color);
    border-radius: 12px;
    box-shadow: var(--sketch-shadow-sm);
    cursor: pointer;
    transition: transform 0.1s ease, box-shadow 0.1s ease;
  }
  .mekaar-btn:hover:not(:disabled) {
    transform: translate(-1px, -1px);
    box-shadow: var(--sketch-shadow);
  }
  .mekaar-btn:active:not(:disabled) {
    transform: translate(1px, 1px);
    box-shadow: 1px 1px 0px var(--sketch-border-color);
  }
  .mekaar-btn:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
  .mekaar-btn.sm { padding: 6px 12px; font-size: 0.8rem; }
  .mekaar-btn.md { padding: 9px 16px; font-size: 0.9rem; }
  .mekaar-btn.lg { padding: 12px 22px; font-size: 1rem; }
  .mekaar-btn.full { width: 100%; }

  .mekaar-btn.primary {
    background: var(--accent-primary);
    color: #FFFFFF;
  }
  .mekaar-btn.secondary {
    background: var(--bg-surface);
    color: var(--text-primary);
  }
  .mekaar-btn.danger {
    background: var(--accent-danger);
    color: #FFFFFF;
  }
  .mekaar-btn.ghost {
    background: transparent;
    border-color: transparent;
    box-shadow: none;
    color: var(--text-primary);
  }
</style>
