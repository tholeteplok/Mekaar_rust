<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger' | 'sos';
    size?: 'sm' | 'md' | 'lg';
    fullWidth?: boolean;
    disabled?: boolean;
    loading?: boolean;
    onclick?: (e: MouseEvent) => void;
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    fullWidth = false,
    disabled = false,
    loading = false,
    onclick,
    children,
  }: Props = $props();
</script>

<button
  type="button"
  class="mekaar-btn variant-{variant} size-{size}"
  class:full-width={fullWidth}
  class:loading
  {disabled}
  {onclick}
>
  {#if loading}
    <span class="spinner" aria-hidden="true"></span>
  {/if}
  {#if children}
    {@render children()}
  {/if}
</button>

<style>
  .mekaar-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    font-family: var(--font-family);
    font-weight: 600;
    border-radius: var(--radius-pill);
    border: 1px solid transparent;
    cursor: pointer;
    transition: all var(--motion-fast);
    outline: none;
    white-space: nowrap;
    user-select: none;
    min-height: 44px; /* Touch target minimum 44dp */
  }

  .mekaar-btn:active:not(:disabled) {
    transform: scale(0.97);
  }

  .full-width {
    width: 100%;
  }

  /* --- Sizes --- */
  .size-sm {
    padding: var(--space-1) var(--space-3);
    font-size: var(--font-label-size);
    min-height: 36px;
  }

  .size-md {
    padding: var(--space-2) var(--space-5);
    font-size: var(--font-body-size);
    min-height: 46px;
  }

  .size-lg {
    padding: var(--space-3) var(--space-6);
    font-size: var(--font-h2-size);
    min-height: 54px;
  }

  /* --- Variants --- */
  .variant-primary {
    background-color: var(--brand-blue);
    color: var(--text-on-blue);
    box-shadow: 0 2px 8px rgba(var(--brand-blue-rgb), 0.25);
  }
  .variant-primary:hover:not(:disabled) {
    filter: brightness(1.08);
  }

  .variant-secondary {
    background-color: var(--surface-card);
    color: var(--text-primary);
    border-color: var(--border-subtle);
    box-shadow: var(--shadow-soft);
  }
  .variant-secondary:hover:not(:disabled) {
    background-color: var(--surface-subtle);
  }

  .variant-outline {
    background-color: transparent;
    color: var(--brand-blue);
    border-color: var(--brand-blue);
  }
  .variant-outline:hover:not(:disabled) {
    background-color: rgba(var(--brand-blue-rgb), 0.06);
  }

  .variant-ghost {
    background-color: transparent;
    color: var(--text-secondary);
  }
  .variant-ghost:hover:not(:disabled) {
    background-color: var(--surface-subtle);
    color: var(--text-primary);
  }

  .variant-danger {
    background-color: var(--sos-deep);
    color: var(--text-on-coral);
  }

  .variant-sos {
    background-color: var(--sos-coral);
    color: var(--text-on-coral);
    box-shadow: 0 4px 16px rgba(var(--sos-coral-rgb), 0.4);
    font-weight: 700;
  }

  .mekaar-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  /* --- Spinner --- */
  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: spin 700ms linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
