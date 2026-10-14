<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    subtitle?: string;
    showBack?: boolean;
    register?: 'core' | 'room' | 'sos';
    elevated?: boolean;
    onback?: () => void;
    leading?: Snippet;
    actions?: Snippet;
  }

  let {
    title,
    subtitle,
    showBack = false,
    register = 'core',
    elevated = false,
    onback,
    leading,
    actions,
  }: Props = $props();
</script>

<header
  class="mekaar-appbar register-{register}"
  class:elevated
>
  <div class="appbar-content">
    <!-- Leading Area -->
    <div class="leading-area">
      {#if leading}
        {@render leading()}
      {:else if showBack}
        <button
          type="button"
          class="icon-btn"
          aria-label="Kembali"
          onclick={onback}
        >
          <svg class="back-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
        </button>
      {/if}
    </div>

    <!-- Title / Subtitle Area -->
    <div class="title-area">
      <h1 class="title">{title}</h1>
      {#if subtitle}
        <p class="subtitle">{subtitle}</p>
      {/if}
    </div>

    <!-- Actions Area -->
    <div class="actions-area">
      {#if actions}
        {@render actions()}
      {/if}
    </div>
  </div>
</header>

<style>
  .mekaar-appbar {
    position: sticky;
    top: 0;
    z-index: 50;
    width: 100%;
    background-color: var(--surface-canvas);
    border-bottom: 1px solid var(--border-subtle);
    backdrop-filter: blur(12px);
    transition: background-color var(--motion-normal), border-color var(--motion-normal);
  }

  .mekaar-appbar.elevated {
    box-shadow: var(--shadow-soft);
  }

  .appbar-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 56px;
    padding: var(--space-2) var(--space-4);
    gap: var(--space-3);
    max-width: 1200px;
    margin: 0 auto;
  }

  .leading-area {
    display: flex;
    align-items: center;
    min-width: 40px;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    color: var(--text-primary);
    cursor: pointer;
    transition: background-color var(--motion-fast);
  }

  .icon-btn:hover {
    background-color: var(--surface-subtle);
  }

  .icon-btn:active {
    transform: scale(0.95);
  }

  .back-icon {
    width: 22px;
    height: 22px;
  }

  .title-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
  }

  .title {
    font-size: var(--font-h2-size);
    line-height: var(--font-h2-line);
    font-weight: var(--font-h2-weight);
    color: var(--text-primary);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .subtitle {
    font-size: var(--font-caption-size);
    line-height: var(--font-caption-line);
    font-weight: var(--font-caption-weight);
    color: var(--text-secondary);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .actions-area {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-width: 40px;
    justify-content: flex-end;
  }

  /* Register overrides */
  .register-sos {
    background-color: var(--surface-canvas);
    border-bottom: 2px solid var(--sos-coral);
  }

  .register-sos .title {
    color: var(--text-primary);
    letter-spacing: 0.5px;
  }
</style>
