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
  <div class="dialog-backdrop" onclick={onclose}>
    <div class="dialog-box sketch-card" onclick={(e) => e.stopPropagation()}>
      <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;">
        <h3 style="font-size: 1.08rem; font-weight: 700;">{title}</h3>
        <button type="button" class="close-btn" onclick={onclose}>✕</button>
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
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 100;
  }
  .dialog-box {
    width: min(480px, 100%);
    padding: 20px;
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
