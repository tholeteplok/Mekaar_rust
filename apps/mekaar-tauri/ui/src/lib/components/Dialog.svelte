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
  <div class="dialog-backdrop" onclick={onclose}>
    <div class="dialog-box" onclick={(e) => e.stopPropagation()}>
      <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;">
        <h3 style="font-size: 1.08rem; font-weight: 700; color: var(--text-primary);">{title}</h3>
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
  .dialog-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 20, 28, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 1000;
  }
  .dialog-box {
    width: min(480px, 100%);
    padding: 24px;
    background: var(--bg-surface, #FFFFFF);
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: var(--radius-card, 20px);
    box-shadow: 0 16px 40px rgba(15, 23, 42, 0.25);
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
