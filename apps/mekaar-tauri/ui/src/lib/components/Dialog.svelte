<script lang="ts">
  import type { Snippet } from 'svelte';
  import { X } from 'phosphor-svelte';

  interface Props {
    open: boolean;
    title: string;
    subtitle?: string;
    maxWidth?: string;
    onclose: () => void;
    children?: Snippet;
  }

  let {
    open,
    title,
    subtitle = '',
    maxWidth = 'min(520px, 100%)',
    onclose,
    children
  }: Props = $props();
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="dialog-backdrop" onclick={onclose}>
    <div
      class="dialog-box"
      style="width: {maxWidth};"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="dialog-header-row">
        <div class="dialog-title-col">
          <h3 class="dialog-title">{title}</h3>
          {#if subtitle}
            <p class="dialog-subtitle">{subtitle}</p>
          {/if}
        </div>
        <button
          type="button"
          class="close-btn"
          onclick={onclose}
          aria-label="Tutup"
        >
          <X size={18} weight="bold" />
        </button>
      </div>
      {#if children}
        {@render children()}
      {/if}
    </div>
  </div>
{/if}

<style>
  .dialog-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 20, 28, 0.6);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 1000;
  }
  .dialog-box {
    padding: 24px;
    background: var(--bg-surface, #FFFFFF);
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: var(--radius-card, 20px);
    box-shadow: 0 16px 40px rgba(15, 23, 42, 0.25);
    box-sizing: border-box;
    max-height: 90vh;
    overflow-y: auto;
    animation: dialogPopIn 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }
  @keyframes dialogPopIn {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
  .dialog-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 16px;
  }
  .dialog-title-col {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .dialog-title {
    font-family: var(--font-active);
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.3;
    margin: 0;
  }
  .dialog-subtitle {
    font-size: 0.82rem;
    color: var(--text-secondary);
    margin: 2px 0 0 0;
    line-height: 1.35;
  }
  .close-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: 50%;
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    font-weight: 700;
    cursor: pointer;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: background-color 0.15s ease, transform 0.1s ease;
  }
  .close-btn:hover {
    background: var(--border-light);
  }
  .close-btn:active {
    transform: scale(0.92);
  }
</style>
