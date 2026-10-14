<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open?: boolean;
    title: string;
    description?: string;
    variant?: 'default' | 'danger' | 'sos';
    dismissible?: boolean;
    headerSlot?: Snippet;
    children?: Snippet;
    actions?: Snippet;
    onclose?: () => void;
  }

  let {
    open = $bindable(false),
    title,
    description,
    variant = 'default',
    dismissible = true,
    headerSlot,
    children,
    actions,
    onclose,
  }: Props = $props();

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && dismissible) {
      close();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open && dismissible) {
      close();
    }
  }

  function close() {
    open = false;
    onclose?.();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="mekaar-dialog-backdrop"
    onclick={handleBackdropClick}
    role="presentation"
  >
    <div
      class="mekaar-dialog-box variant-{variant}"
      role="dialog"
      aria-modal="true"
      aria-labelledby="dialog-title"
    >
      <!-- Header -->
      <div class="dialog-header">
        {#if headerSlot}
          {@render headerSlot()}
        {/if}
        <div class="header-titles">
          <h2 id="dialog-title" class="dialog-title">{title}</h2>
          {#if description}
            <p class="dialog-desc">{description}</p>
          {/if}
        </div>
        {#if dismissible}
          <button
            type="button"
            class="close-btn"
            aria-label="Tutup"
            onclick={close}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        {/if}
      </div>

      <!-- Body -->
      {#if children}
        <div class="dialog-body">
          {@render children()}
        </div>
      {/if}

      <!-- Actions Footer -->
      {#if actions}
        <div class="dialog-actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .mekaar-dialog-backdrop {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-4);
    background-color: rgba(21, 38, 65, 0.55);
    backdrop-filter: blur(8px);
    animation: fadeIn var(--motion-fast) ease-out;
  }

  .mekaar-dialog-box {
    width: 100%;
    max-width: 480px;
    background-color: var(--surface-card);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-elevated);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: popIn var(--motion-bounce);
  }

  .variant-danger {
    border-color: var(--sos-coral);
  }

  .variant-sos {
    background-color: var(--surface-card);
    border: 2px solid var(--sos-coral);
    box-shadow: 0 8px 32px rgba(var(--sos-coral-rgb), 0.3);
  }

  .dialog-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-3);
    padding: var(--space-5) var(--space-5) var(--space-3);
  }

  .header-titles {
    flex: 1;
  }

  .dialog-title {
    font-size: var(--font-h2-size);
    line-height: var(--font-h2-line);
    font-weight: 700;
    color: var(--text-primary);
  }

  .dialog-desc {
    margin-top: 4px;
    font-size: var(--font-label-size);
    line-height: var(--font-label-line);
    color: var(--text-secondary);
  }

  .close-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--radius-pill);
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background-color var(--motion-fast);
  }

  .close-btn:hover {
    background-color: var(--surface-subtle);
    color: var(--text-primary);
  }

  .close-btn svg {
    width: 18px;
    height: 18px;
  }

  .dialog-body {
    padding: var(--space-3) var(--space-5) var(--space-4);
    color: var(--text-primary);
    font-size: var(--font-body-size);
    overflow-y: auto;
    max-height: 60vh;
  }

  .dialog-actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-5) var(--space-5);
    border-top: 1px solid var(--border-subtle);
    background-color: var(--surface-subtle);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: scale(0.94) translateY(8px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
</style>
