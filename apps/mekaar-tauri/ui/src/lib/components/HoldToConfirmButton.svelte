<script lang="ts">
  interface Props {
    label?: string;
    holdDurationMs?: number;
    onconfirm: () => void;
  }

  let {
    label = 'TAHAN 2 DETIK UNTUK AKTIFKAN SOS DARURAT',
    holdDurationMs = 2000,
    onconfirm
  }: Props = $props();

  let progress = $state(0);
  let holding = $state(false);
  let timer: ReturnType<typeof setInterval> | null = null;

  function startHold() {
    holding = true;
    const stepMs = 40;
    const increment = (stepMs / holdDurationMs) * 100;
    timer = setInterval(() => {
      progress = Math.min(100, progress + increment);
      if (progress >= 100) {
        stopHold();
        onconfirm();
      }
    }, stepMs);
  }

  function stopHold() {
    holding = false;
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
    progress = 0;
  }
</script>

<button
  type="button"
  class="sos-hold-btn sketch-card"
  onmousedown={startHold}
  onmouseup={stopHold}
  onmouseleave={stopHold}
  ontouchstart={startHold}
  ontouchend={stopHold}
>
  <div class="progress-bar" style="width: {progress}%;"></div>
  <span class="btn-content">
    🚨 {holding ? `MENAHAN... (${Math.round(progress)}%)` : label}
  </span>
</button>

<style>
  .sos-hold-btn {
    position: relative;
    width: 100%;
    padding: 18px 20px;
    background: #E11D48;
    color: #FFFFFF;
    font-family: var(--font-sans);
    font-weight: 800;
    font-size: 1rem;
    overflow: hidden;
    cursor: pointer;
    text-align: center;
  }
  .progress-bar {
    position: absolute;
    inset: 0;
    background: #881337;
    transition: width 0.04s linear;
  }
  .btn-content {
    position: relative;
    z-index: 2;
  }
</style>
