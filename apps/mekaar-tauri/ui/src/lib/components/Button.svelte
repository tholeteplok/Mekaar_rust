<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    fullWidth?: boolean;
    type?: 'button' | 'submit' | 'reset';
    title?: string;
    onclick?: () => void;
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    fullWidth = false,
    type = 'button',
    title,
    onclick,
    children
  }: Props = $props();
</script>

<button
  {type}
  {title}
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
    font-family: var(--font-active);
    font-weight: 600;
    border-radius: var(--radius-pill);
    cursor: pointer;
    box-sizing: border-box;
    transition: all 0.15s ease;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
    text-decoration: none;
    line-height: 1.2;
    min-height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
  }

  .mekaar-btn:active:not(:disabled) {
    transform: scale(0.97);
  }

  .mekaar-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Sizes with Minimum Touch Target 40px enforcement */
  .mekaar-btn.sm {
    min-height: var(--touch-target-min, 40px);
    padding: 8px 16px;
    font-size: 0.84rem;
  }

  .mekaar-btn.md {
    min-height: var(--touch-target-base, 44px);
    padding: 11px 22px;
    font-size: 0.92rem;
  }

  .mekaar-btn.lg {
    min-height: 50px;
    padding: 14px 28px;
    font-size: 1rem;
  }

  .mekaar-btn.full {
    width: 100%;
  }

  /* Semantic Variants */
  .mekaar-btn.primary {
    background-color: var(--brand-blue);
    color: #FFFFFF;
    border: none;
    box-shadow: 0 2px 8px rgba(19, 108, 252, 0.25);
  }
  .mekaar-btn.primary:hover:not(:disabled) {
    background-color: var(--brand-blue-hover);
  }

  .mekaar-btn.secondary {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-subtle);
  }
  .mekaar-btn.secondary:hover:not(:disabled) {
    background-color: var(--border-light);
  }

  .mekaar-btn.danger {
    background-color: var(--sos-coral);
    color: #FFFFFF;
    border: none;
    box-shadow: 0 2px 8px rgba(255, 93, 93, 0.3);
  }
  .mekaar-btn.danger:hover:not(:disabled) {
    background-color: var(--sos-deep);
  }

  .mekaar-btn.ghost {
    background: transparent;
    border: none;
    box-shadow: none;
    color: var(--text-primary);
  }
  .mekaar-btn.ghost:hover:not(:disabled) {
    background-color: rgba(19, 108, 252, 0.06);
  }
</style>
