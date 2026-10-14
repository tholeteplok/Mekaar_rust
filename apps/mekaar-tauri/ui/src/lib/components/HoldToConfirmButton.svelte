<script lang="ts">
  interface Props {
    durationMs?: number;
    size?: number;
    label?: string;
    sublabel?: string;
    disabled?: boolean;
    onconfirm?: () => void;
  }

  let {
    durationMs = 2000,
    size = 76,
    label = 'SOS',
    sublabel = 'TAHAN 2 DETIK',
    disabled = false,
    onconfirm,
  }: Props = $props();

  let holding = $state(false);
  let progress = $state(0);
  let startTime = $state(0);
  let animationFrameId: number | null = null;
  let confirmed = $state(false);

  // SVG dimensions
  const strokeWidth = 5;
  const radius = $derived((size - strokeWidth * 2) / 2);
  const circumference = $derived(2 * Math.PI * radius);
  const strokeDashoffset = $derived(circumference - (progress / 100) * circumference);

  function startHold(e: PointerEvent | TouchEvent) {
    if (disabled || confirmed) return;
    e.preventDefault();
    holding = true;
    progress = 0;
    startTime = performance.now();

    function update() {
      if (!holding) return;
      const elapsed = performance.now() - startTime;
      const pct = Math.min(100, (elapsed / durationMs) * 100);
      progress = pct;

      if (pct >= 100) {
        completeHold();
      } else {
        animationFrameId = requestAnimationFrame(update);
      }
    }

    animationFrameId = requestAnimationFrame(update);
  }

  function endHold() {
    if (confirmed) return;
    holding = false;
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
    // Smooth reset
    progress = 0;
  }

  function completeHold() {
    holding = false;
    confirmed = true;
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
    progress = 100;
    onconfirm?.();

    // Reset confirmed state after 3 seconds so it can be re-triggered if needed
    setTimeout(() => {
      confirmed = false;
      progress = 0;
    }, 3000);
  }
</script>

<div class="hold-container" style="--btn-size: {size}px;">
  <button
    type="button"
    class="hold-button"
    class:holding
    class:confirmed
    class:disabled
    {disabled}
    aria-label="{label} - {sublabel}"
    onpointerdown={startHold}
    onpointerup={endHold}
    onpointerleave={endHold}
    onpointercancel={endHold}
  >
    <!-- SVG Circular Progress Ring -->
    <svg class="progress-ring" width={size} height={size}>
      <!-- Background track -->
      <circle
        class="progress-ring-track"
        cx={size / 2}
        cy={size / 2}
        r={radius}
        stroke-width={strokeWidth}
      />
      <!-- Active animated progress -->
      <circle
        class="progress-ring-indicator"
        cx={size / 2}
        cy={size / 2}
        r={radius}
        stroke-width={strokeWidth}
        stroke-dasharray={circumference}
        stroke-dashoffset={strokeDashoffset}
      />
    </svg>

    <!-- Center Icon / Label -->
    <div class="button-content">
      <span class="label">{label}</span>
      {#if holding}
        <span class="pct">{Math.round(progress)}%</span>
      {/if}
    </div>
  </button>

  {#if sublabel}
    <span class="sublabel">{sublabel}</span>
  {/if}
</div>

<style>
  .hold-container {
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    user-select: none;
    -webkit-user-select: none;
  }

  .hold-button {
    position: relative;
    width: var(--btn-size);
    height: var(--btn-size);
    border-radius: var(--radius-pill);
    border: none;
    background-color: var(--sos-coral);
    box-shadow: 0 4px 20px rgba(var(--sos-coral-rgb), 0.35);
    cursor: pointer;
    outline: none;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform var(--motion-fast), box-shadow var(--motion-fast);
    touch-action: none;
  }

  .hold-button:hover:not(:disabled) {
    box-shadow: 0 6px 24px rgba(var(--sos-coral-rgb), 0.45);
  }

  .hold-button.holding {
    transform: scale(0.96);
    box-shadow: 0 2px 14px rgba(var(--sos-coral-rgb), 0.6);
  }

  .hold-button.confirmed {
    animation: sosPulse 600ms infinite alternate;
  }

  .hold-button.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .progress-ring {
    position: absolute;
    inset: 0;
    transform: rotate(-90deg); /* Start from top */
    pointer-events: none;
  }

  .progress-ring-track {
    fill: transparent;
    stroke: rgba(255, 255, 255, 0.25);
  }

  .progress-ring-indicator {
    fill: transparent;
    stroke: #FFFFFF;
    stroke-linecap: round;
    transition: stroke-dashoffset 50ms linear;
  }

  .button-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #FFFFFF;
    pointer-events: none;
  }

  .label {
    font-size: var(--font-h2-size);
    font-weight: 800;
    letter-spacing: 1px;
    line-height: 1;
  }

  .pct {
    font-size: var(--font-caption-size);
    font-weight: 600;
    margin-top: 2px;
  }

  .sublabel {
    font-size: var(--font-caption-size);
    font-weight: 700;
    color: var(--sos-coral);
    letter-spacing: 0.5px;
    text-transform: uppercase;
  }

  @keyframes sosPulse {
    0% {
      transform: scale(1);
      box-shadow: 0 0 0 0 rgba(var(--sos-coral-rgb), 0.7);
    }
    100% {
      transform: scale(1.05);
      box-shadow: 0 0 0 16px rgba(var(--sos-coral-rgb), 0);
    }
  }
</style>
