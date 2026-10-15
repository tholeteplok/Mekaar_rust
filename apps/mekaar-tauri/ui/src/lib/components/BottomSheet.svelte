<script lang="ts">
  import type { Snippet } from 'svelte';
  import { X } from 'phosphor-svelte';

  interface Props {
    open: boolean;
    title: string;
    onclose: () => void;
    children?: Snippet;
  }

  let { open, title, onclose, children }: Props = $props();
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="sheet-backdrop" onclick={onclose}>
    <div class="sheet-panel" onclick={(e) => e.stopPropagation()}>
      <div class="sheet-handle"></div>
      <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
        <h3 style="font-size: 1.05rem; font-weight: 700; color: var(--text-primary);">{title}</h3>
        <button
          type="button"
          class="close-btn"
          onclick={onclose}
          aria-label="Tutup"
          style="display: flex; align-items: center; justify-content: center;"
        >
          <X size={16} weight="bold" />
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
    backdrop-filter: blur(4px);
    display: flex;
    align-items: flex-end;
    justify-content: center;
    z-index: 1000;
  }
  .sheet-panel {
    width: min(640px, 100%);
    max-height: 85vh;
    overflow-y: auto;
    padding: 16px 20px 28px;
    background: var(--bg-surface, #FFFFFF);
    border-top: 1px solid var(--border-subtle, #E2E8F0);
    border-top-left-radius: 24px;
    border-top-right-radius: 24px;
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
    box-shadow: 0 -10px 40px rgba(15, 23, 42, 0.25);
  }
  .sheet-handle {
    width: 44px;
    height: 5px;
    background: var(--border-subtle, #CBD5E1);
    border-radius: 999px;
    margin: 0 auto 14px;
  }
  .close-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    width: 30px;
    height: 30px;
    font-weight: 700;
    cursor: pointer;
    color: var(--text-secondary);
  }
</style>
