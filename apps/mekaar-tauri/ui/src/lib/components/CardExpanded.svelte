<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    subtitle?: string;
    badge?: string;
    badgeVariant?: 'blue' | 'green' | 'coral' | 'amber';
    expanded?: boolean;
    headerLeading?: Snippet;
    headerTrailing?: Snippet;
    children?: Snippet;
    ontoggle?: (expanded: boolean) => void;
  }

  let {
    title,
    subtitle,
    badge,
    badgeVariant = 'blue',
    expanded = $bindable(false),
    headerLeading,
    headerTrailing,
    children,
    ontoggle,
  }: Props = $props();

  function toggle() {
    expanded = !expanded;
    ontoggle?.(expanded);
  }
</script>

<div class="mekaar-card-expanded" class:is-expanded={expanded}>
  <!-- Header / Trigger -->
  <button
    type="button"
    class="header-trigger"
    aria-expanded={expanded}
    onclick={toggle}
  >
    {#if headerLeading}
      <div class="leading-slot">
        {@render headerLeading()}
      </div>
    {/if}

    <div class="header-text">
      <div class="title-row">
        <span class="card-title">{title}</span>
        {#if badge}
          <span class="card-badge badge-{badgeVariant}">{badge}</span>
        {/if}
      </div>
      {#if subtitle}
        <span class="card-subtitle">{subtitle}</span>
      {/if}
    </div>

    <div class="trailing-area">
      {#if headerTrailing}
        {@render headerTrailing()}
      {/if}
      <span class="chevron" class:rotated={expanded} aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
      </span>
    </div>
  </button>

  <!-- Collapsible Body using CSS Grid animation -->
  <div class="collapsible-wrapper" class:open={expanded}>
    <div class="collapsible-inner">
      <div class="content-divider"></div>
      <div class="body-content">
        {#if children}
          {@render children()}
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .mekaar-card-expanded {
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
    background-color: var(--surface-card);
    box-shadow: var(--shadow-soft);
    transition: border-color var(--motion-fast), box-shadow var(--motion-fast);
    overflow: hidden;
  }

  .mekaar-card-expanded.is-expanded {
    border-color: var(--border-focus);
    box-shadow: var(--shadow-elevated);
  }

  .header-trigger {
    display: flex;
    align-items: center;
    width: 100%;
    padding: var(--space-4);
    gap: var(--space-3);
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    outline: none;
    user-select: none;
  }

  .header-trigger:hover {
    background-color: var(--surface-subtle);
  }

  .leading-slot {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .header-text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .card-title {
    font-size: var(--font-body-size);
    font-weight: 700;
    color: var(--text-primary);
  }

  .card-subtitle {
    font-size: var(--font-caption-size);
    color: var(--text-secondary);
  }

  .card-badge {
    display: inline-flex;
    align-items: center;
    padding: 2px var(--space-2);
    border-radius: var(--radius-pill);
    font-size: var(--font-caption-size);
    font-weight: 600;
    line-height: 1;
  }

  .badge-blue {
    background-color: var(--brand-light-blue);
    color: var(--brand-blue);
  }

  .badge-green {
    background-color: rgba(var(--brand-green-rgb), 0.2);
    color: var(--brand-dark-blue);
  }

  .badge-coral {
    background-color: rgba(var(--sos-coral-rgb), 0.15);
    color: var(--sos-coral);
  }

  .badge-amber {
    background-color: rgba(251, 191, 36, 0.18);
    color: #B45309;
  }

  .trailing-area {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-shrink: 0;
  }

  .chevron {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    color: var(--text-secondary);
    transition: transform var(--motion-normal);
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .chevron svg {
    width: 18px;
    height: 18px;
  }

  /* --- Smooth Grid Row Expansion --- */
  .collapsible-wrapper {
    display: grid;
    grid-template-rows: 0fr;
    transition: grid-template-rows var(--motion-normal);
  }

  .collapsible-wrapper.open {
    grid-template-rows: 1fr;
  }

  .collapsible-inner {
    overflow: hidden;
  }

  .content-divider {
    height: 1px;
    background-color: var(--border-subtle);
    margin: 0 var(--space-4);
  }

  .body-content {
    padding: var(--space-4);
    color: var(--text-primary);
    font-size: var(--font-body-size);
    line-height: var(--font-body-line);
  }
</style>
