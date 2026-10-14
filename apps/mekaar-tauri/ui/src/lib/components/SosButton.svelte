<script lang="ts">
  interface Props {
    size?: number;
    isActive?: boolean;
    onTrigger?: () => void;
  }

  let { size = 72, isActive = false, onTrigger }: Props = $props();

  let isHolding = $state(false);
  let holdProgress = $state(0); // 0 to 1
  let holdStartTime = 0;
  let animFrameId: number | null = null;
  const HOLD_DURATION_MS = 2000;

  function startHold(e: MouseEvent | TouchEvent) {
    if (isActive) {
      // 1-tap when already active
      onTrigger?.();
      return;
    }
    isHolding = true;
    holdProgress = 0;
    holdStartTime = performance.now();

    const loop = (now: number) => {
      if (!isHolding) return;
      const elapsed = now - holdStartTime;
      holdProgress = Math.min(1, elapsed / HOLD_DURATION_MS);
      if (holdProgress >= 1) {
        // Complete!
        isHolding = false;
        holdProgress = 0;
        if (navigator.vibrate) {
          navigator.vibrate([100, 50, 200]);
        }
        onTrigger?.();
      } else {
        animFrameId = requestAnimationFrame(loop);
      }
    };
    animFrameId = requestAnimationFrame(loop);
  }

  function cancelHold() {
    isHolding = false;
    holdProgress = 0;
    if (animFrameId) {
      cancelAnimationFrame(animFrameId);
      animFrameId = null;
    }
  }

  const radius = $derived((size / 2) - 4);
  const circumference = $derived(2 * Math.PI * radius);
  const strokeDashoffset = $derived(circumference * (1 - holdProgress));
</script>

<div
  class="sos-wrapper"
  style="width: {size}px; height: {size}px;"
>
  <!-- Background Ripple Pulsing when Active or Holding -->
  {#if isActive || isHolding}
    <div class="sos-pulse-ring" class:holding={isHolding}></div>
  {/if}

  <button
    type="button"
    class="sos-circle-btn {isActive ? 'active' : ''} {isHolding ? 'holding' : ''}"
    style="width: {size}px; height: {size}px; font-size: {Math.round(size * 0.28)}px;"
    onmousedown={startHold}
    onmouseup={cancelHold}
    onmouseleave={cancelHold}
    ontouchstart={startHold}
    ontouchend={cancelHold}
    ontouchcancel={cancelHold}
    aria-label="Tombol Darurat SOS. Tahan 2 detik untuk aktivasi."
  >
    <span class="sos-label">SOS</span>
  </button>

  <!-- Circular Progress Ring (Hold-to-confirm 2s) -->
  {#if isHolding}
    <svg
      class="hold-progress-svg"
      width={size}
      height={size}
      viewBox="0 0 {size} {size}"
    >
      <circle
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke="#FFFFFF"
        stroke-width="4.5"
        stroke-linecap="round"
        stroke-dasharray={circumference}
        stroke-dashoffset={strokeDashoffset}
        transform="rotate(-90 {size / 2} {size / 2})"
      />
    </svg>
  {/if}
</div>

<style>
  .sos-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
  }

  .sos-circle-btn {
    position: relative;
    border-radius: 50%;
    background: radial-gradient(circle at 35% 35%, #FF6B6B 0%, #E11D48 70%, #BE123C 100%);
    border: none;
    color: #FFFFFF;
    font-weight: 800;
    font-family: var(--font-sans);
    letter-spacing: 0.5px;
    cursor: pointer;
    box-shadow: 0 6px 20px rgba(225, 29, 72, 0.42);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.12s cubic-bezier(0.34, 1.56, 0.64, 1);
    z-index: 2;
  }

  .sos-circle-btn:active,
  .sos-circle-btn.holding {
    transform: scale(0.94);
    box-shadow: 0 2px 10px rgba(225, 29, 72, 0.5);
  }

  .sos-circle-btn.active {
    background: radial-gradient(circle at 35% 35%, #FF4D4D 0%, #D92632 70%, #991B1B 100%);
    animation: sos-active-pulse 1.6s infinite ease-in-out;
  }

  .sos-pulse-ring {
    position: absolute;
    inset: -6px;
    border-radius: 50%;
    background: rgba(225, 29, 72, 0.25);
    animation: pulse-ring-anim 1.4s infinite ease-out;
    pointer-events: none;
    z-index: 1;
  }

  .sos-pulse-ring.holding {
    background: rgba(225, 29, 72, 0.4);
    animation: none;
    transform: scale(1.15);
  }

  .hold-progress-svg {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 3;
  }

  @keyframes sos-active-pulse {
    0%, 100% {
      transform: scale(1);
      box-shadow: 0 6px 20px rgba(225, 29, 72, 0.45);
    }
    50% {
      transform: scale(1.06);
      box-shadow: 0 10px 28px rgba(225, 29, 72, 0.65);
    }
  }

  @keyframes pulse-ring-anim {
    0% {
      transform: scale(0.95);
      opacity: 0.8;
    }
    100% {
      transform: scale(1.35);
      opacity: 0;
    }
  }
</style>
