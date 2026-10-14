<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    interactive?: boolean;
    withTicks?: boolean;
    padding?: string;
    style?: string;
    onclick?: () => void;
    children?: Snippet;
  }

  let {
    interactive = false,
    withTicks = true,
    padding = '16px',
    style = '',
    onclick,
    children
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="clean-card {withTicks ? 'with-ticks' : ''} {interactive ? 'interactive' : ''}"
  style="padding: {padding}; cursor: {interactive ? 'pointer' : 'default'}; {style}"
  {onclick}
>
  {#if withTicks}
    <span class="pencil-tick tick-bl">⌞</span>
    <span class="pencil-tick tick-tr">⌝</span>
  {/if}
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .clean-card {
    background: var(--bg-surface);
    border-radius: var(--radius-card);
    border: 0.75px solid var(--border-subtle);
    box-shadow: var(--shadow-card);
    position: relative;
    transition: box-shadow 0.2s ease, transform 0.15s ease;
    box-sizing: border-box;
  }

  .clean-card.interactive:active {
    transform: scale(0.985);
  }

  .pencil-tick {
    position: absolute;
    color: var(--sketch-grey);
    font-size: 13px;
    font-family: monospace;
    line-height: 1;
    pointer-events: none;
    user-select: none;
  }

  .tick-bl {
    bottom: -6px;
    left: -6px;
  }

  .tick-tr {
    top: -6px;
    right: -6px;
  }
</style>
