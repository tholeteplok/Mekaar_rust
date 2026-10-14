<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    interactive?: boolean;
    padding?: string;
    onclick?: () => void;
    children?: Snippet;
  }

  let { interactive = false, padding = '14px', onclick, children }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="sketch-card {interactive ? 'sketch-card-interactive' : ''}"
  style="padding: {padding}; cursor: {interactive ? 'pointer' : 'default'};"
  {onclick}
>
  {#if children}
    {@render children()}
  {/if}
</div>
