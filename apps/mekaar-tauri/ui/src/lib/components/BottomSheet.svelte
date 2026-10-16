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
    maxWidth = 'min(640px, 100%)',
    onclose,
    children
  }: Props = $props();
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="sheet-backdrop" onclick={onclose}>
    <div
      class="sheet-panel"
      style="width: {maxWidth};"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="sheet-handle"></div>
      <div class="sheet-header-row">
        <div class="sheet-title-col">
          <h3 class="sheet-title">{title}</h3>
          {#if subtitle}
            <p class="sheet-subtitle">{subtitle}</p>
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
  .sheet-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 20, 28, 0.6);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    display: flex;
    align-items: flex-end;
    justify-content: center;
    z-index: 1000;
  }
  .sheet-panel {
    max-height: 85vh;
    overflow-y: auto;
    padding: 16px 20px 28px;
    background: var(--bg-surface, #FFFFFF);
    border-top: 1px solid var(--border-subtle, #E2E8F0);
    border-top-left-radius: 24px;
    border-top-right-radius: 24px;
    box-shadow: 0 -10px 40px rgba(15, 23, 42, 0.25);
    box-sizing: border-box;
    animation: sheetSlideUp 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }
  @keyframes sheetSlideUp {
    from { transform: translateY(100%); }
    to { transform: translateY(0); }
  }
  .sheet-handle {
    width: 44px;
    height: 5px;
    background: var(--border-subtle, #CBD5E1);
    border-radius: 999px;
    margin: 0 auto 14px;
  }
  .sheet-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 14px;
  }
  .sheet-title-col {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .sheet-title {
    font-family: var(--font-active);
    font-size: 1.08rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.3;
    margin: 0;
  }
  .sheet-subtitle {
    font-size: 0.8rem;
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
