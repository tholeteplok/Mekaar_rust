<script lang="ts">
  import type { Component } from 'svelte';
  import MikaMascot, { type MikaPoseName } from './MikaMascot.svelte';

  interface Props {
    pose?: MikaPoseName;
    title: string;
    message: string;
    mascotSize?: number;
    actionLabel?: string;
    actionIcon?: Component<{ size?: number; weight?: string; color?: string }>;
    onaction?: () => void;
  }

  let {
    pose = 'huft',
    title,
    message,
    mascotSize = 130,
    actionLabel,
    actionIcon: ActionIcon,
    onaction
  }: Props = $props();
</script>

<div class="empty-state-root">
  <div class="mascot-container">
    <MikaMascot {pose} size={mascotSize} alt={title} />
  </div>

  <h3 class="empty-state-title">{title}</h3>
  <p class="empty-state-desc">{message}</p>

  {#if actionLabel && onaction}
    <div class="empty-state-action-box">
      <button
        type="button"
        class="empty-state-btn"
        onclick={onaction}
      >
        {#if ActionIcon}
          <ActionIcon size={18} weight="bold" color="#FFFFFF" />
        {/if}
        <span>{actionLabel}</span>
      </button>
    </div>
  {/if}
</div>

<style>
  .empty-state-root {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 48px 24px 32px 24px;
    margin: 16px auto;
    max-width: 420px;
    user-select: none;
    -webkit-user-select: none;
  }

  .mascot-container {
    margin-bottom: 20px;
  }

  .empty-state-title {
    font-family: var(--font-active);
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 8px;
    letter-spacing: -0.2px;
  }

  .empty-state-desc {
    font-size: 0.88rem;
    color: var(--text-secondary);
    line-height: 1.5;
    max-width: 320px;
    margin: 0 auto;
  }

  .empty-state-action-box {
    margin-top: 24px;
  }

  .empty-state-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 11px 22px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-blue);
    color: #FFFFFF;
    font-size: 0.9rem;
    font-weight: 600;
    border: none;
    cursor: pointer;
    box-shadow: 0 4px 14px rgba(2, 132, 199, 0.28);
    transition: transform 0.15s ease, background-color 0.15s ease;
  }

  .empty-state-btn:active {
    transform: scale(0.96);
  }
</style>
