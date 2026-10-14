<script lang="ts">
  import type { Snippet } from 'svelte';

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
    <div class="sheet-panel sketch-card" onclick={(e) => e.stopPropagation()}>
      <div class="sheet-handle"></div>
      <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
        <h3 style="font-size: 1.05rem; font-weight: 700;">{title}</h3>
        <button type="button" class="close-btn" onclick={onclose}>✕</button>
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
    background: rgba(15, 20, 28, 0.55);
    display: flex;
    align-items: flex-end;
    justify-content: center;
    z-index: 90;
  }
  .sheet-panel {
    width: min(640px, 100%);
    max-height: 82vh;
    overflow-y: auto;
    padding: 16px 20px 24px;
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }
  .sheet-handle {
    width: 44px;
    height: 5px;
    background: var(--text-muted);
    border-radius: 999px;
    margin: 0 auto 12px;
  }
  .close-btn {
    background: var(--bg-secondary);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 8px;
    width: 30px;
    height: 30px;
    font-weight: 700;
    cursor: pointer;
  }
</style>
