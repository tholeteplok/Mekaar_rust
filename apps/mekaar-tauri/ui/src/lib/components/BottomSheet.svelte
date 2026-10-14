<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open?: boolean;
    title?: string;
    dismissible?: boolean;
    children?: Snippet;
    onclose?: () => void;
  }

  let {
    open = $bindable(false),
    title,
    dismissible = true,
    children,
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
    class="mekaar-sheet-backdrop"
    onclick={handleBackdropClick}
    role="presentation"
  >
    <div
      class="mekaar-sheet-container"
      role="dialog"
      aria-modal="true"
    >
      <!-- Drag handle indicator -->
      <div class="sheet-handle-wrapper">
        <div class="sheet-handle"></div>
      </div>

      {#if title}
        <div class="sheet-header">
          <h3 class="sheet-title">{title}</h3>
        </div>
      {/if}

      <div class="sheet-content">
        {#if children}
          {@render children()}
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .mekaar-sheet-backdrop {
    position: fixed;
    inset: 0;
    z-index: 95;
    background-color: rgba(21, 38, 65, 0.55);
    backdrop-filter: blur(6px);
    display: flex;
    align-items: flex-end;
    justify-content: center;
    animation: fadeIn var(--motion-fast) ease-out;
  }

  .mekaar-sheet-container {
    width: 100%;
    max-width: 600px;
    background-color: var(--surface-card);
    border-top: 1px solid var(--border-subtle);
    border-top-left-radius: var(--radius-lg);
    border-top-right-radius: var(--radius-lg);
    box-shadow: var(--shadow-elevated);
    padding: var(--space-3) var(--space-5) var(--space-6);
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    animation: slideUp var(--motion-bounce);
  }

  .sheet-handle-wrapper {
    display: flex;
    justify-content: center;
    padding: var(--space-1) 0 var(--space-3);
  }

  .sheet-handle {
    width: 40px;
    height: 4px;
    border-radius: var(--radius-pill);
    background-color: var(--border-subtle);
  }

  .sheet-header {
    margin-bottom: var(--space-3);
  }

  .sheet-title {
    font-size: var(--font-h2-size);
    font-weight: 700;
    color: var(--text-primary);
  }

  .sheet-content {
    overflow-y: auto;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(100%);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
