<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    theme?: 'light' | 'dark';
    register?: 'core' | 'room' | 'sos';
    appbar?: Snippet;
    bottomNav?: Snippet;
    floatingAction?: Snippet;
    children?: Snippet;
  }

  let {
    theme = 'light',
    register = 'core',
    appbar,
    bottomNav,
    floatingAction,
    children,
  }: Props = $props();
</script>

<div
  class="mekaar-scaffold"
  data-theme={theme}
  data-register={register}
>
  {#if appbar}
    <div class="scaffold-appbar">
      {@render appbar()}
    </div>
  {/if}

  <main class="scaffold-body">
    {#if children}
      {@render children()}
    {/if}
  </main>

  {#if floatingAction}
    <div class="scaffold-fab">
      {@render floatingAction()}
    </div>
  {/if}

  {#if bottomNav}
    <nav class="scaffold-bottom-nav">
      {@render bottomNav()}
    </nav>
  {/if}
</div>

<style>
  .mekaar-scaffold {
    display: flex;
    flex-direction: column;
    width: 100%;
    min-height: 100vh;
    background-color: var(--surface-canvas);
    color: var(--text-primary);
    position: relative;
    padding-top: env(safe-area-inset-top, 0px);
    padding-bottom: env(safe-area-inset-bottom, 0px);
    padding-left: env(safe-area-inset-left, 0px);
    padding-right: env(safe-area-inset-right, 0px);
    box-sizing: border-box;
    transition: background-color var(--motion-normal), color var(--motion-normal);
  }

  .scaffold-appbar {
    flex-shrink: 0;
  }

  .scaffold-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    position: relative;
    overflow-y: auto;
  }

  .scaffold-fab {
    position: fixed;
    bottom: calc(var(--space-6) + 56px);
    right: var(--space-5);
    z-index: 40;
  }

  .scaffold-bottom-nav {
    flex-shrink: 0;
    position: sticky;
    bottom: 0;
    z-index: 45;
    background-color: var(--surface-card);
    border-top: 1px solid var(--border-subtle);
    backdrop-filter: blur(12px);
  }
</style>
