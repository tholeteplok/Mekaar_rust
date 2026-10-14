<script lang="ts">
  import { mekaarStore } from '../stores';
  import MikaMascot, { type MikaPoseName } from '../components/MikaMascot.svelte';

  interface Slide {
    pose: MikaPoseName;
    title: string;
    desc: string;
  }

  const slides: Slide[] = [
    {
      pose: 'hi',
      title: 'Apa itu MEKAAR?',
      desc: 'Aplikasi perlindungan darurat: chat pribadi terenkripsi, tombol SOS satu ketukan, dan pelacakan lokasi saat Anda benar-benar dalam bahaya.'
    },
    {
      pose: 'love',
      title: 'Tambahkan Guardian Anda',
      desc: 'Guardian tidak bisa mengintai Anda. Izin GPS dan audio hanya aktif saat Anda menekan SOS, lalu aksesnya tercatat di Riwayat SOS.'
    },
    {
      pose: 'shield',
      title: 'Kunci dengan PIN 6 digit',
      desc: 'Lindungi aplikasi dengan PIN 6 digit. SOS tetap bisa diakses saat aplikasi terkunci, dan 5 kali salah PIN akan mengunci aplikasi selama 30 menit.'
    }
  ];

  let currentSlide = $state(0);

  function nextSlide() {
    if (currentSlide < slides.length - 1) {
      currentSlide++;
    } else {
      finishOnboarding();
    }
  }

  function finishOnboarding() {
    mekaarStore.advanceFtue('pin_setup');
  }
</script>

<div class="onboarding-container">
  <!-- Top Bar: Skip Button -->
  <div class="top-nav-bar">
    <button type="button" class="skip-btn" onclick={finishOnboarding}>
      Lewati
    </button>
  </div>

  <!-- Carousel Slide Area -->
  <div class="slide-content-viewport">
    <div class="mascot-stage">
      <MikaMascot
        pose={slides[currentSlide].pose}
        size={190}
        alt="Ilustrasi {slides[currentSlide].title}"
      />
    </div>

    <div class="text-stage">
      <h1 class="slide-title">{slides[currentSlide].title}</h1>
      <p class="slide-desc">{slides[currentSlide].desc}</p>
    </div>
  </div>

  <!-- Bottom Navigation: Dots & Next/Start Button -->
  <div class="bottom-action-bar">
    <div class="dots-indicator">
      {#each slides as _, i}
        <button
          type="button"
          class="dot {currentSlide === i ? 'active' : ''}"
          onclick={() => (currentSlide = i)}
          aria-label="Slide {i + 1}"
        ></button>
      {/each}
    </div>

    <button
      type="button"
      class="btn-primary {currentSlide === slides.length - 1 ? 'start-btn' : 'next-btn'}"
      onclick={nextSlide}
    >
      {#if currentSlide === slides.length - 1}
        Mulai Sekarang ➔
      {:else}
        Lanjut ➔
      {/if}
    </button>
  </div>
</div>

<style>
  .onboarding-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: calc(24px + env(safe-area-inset-top, 0px)) 24px calc(36px + env(safe-area-inset-bottom, 0px));
    background-color: var(--bg-primary);
    box-sizing: border-box;
    user-select: none;
    -webkit-user-select: none;
  }

  .top-nav-bar {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    height: 40px;
  }

  .skip-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
    padding: 8px 12px;
    border-radius: var(--radius-pill);
    transition: color 0.15s ease;
  }

  .skip-btn:active {
    color: var(--brand-blue);
  }

  .slide-content-viewport {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 10px 8px;
  }

  .mascot-stage {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 220px;
    margin-bottom: 24px;
  }

  .text-stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    max-width: 340px;
  }

  .slide-title {
    font-family: var(--font-sans);
    font-size: 1.65rem;
    font-weight: 800;
    color: var(--text-primary);
    letter-spacing: -0.3px;
    margin-bottom: 12px;
    line-height: 1.25;
  }

  .slide-desc {
    font-size: 0.96rem;
    color: var(--text-secondary);
    line-height: 1.55;
  }

  .bottom-action-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 20px;
    gap: 16px;
  }

  .dots-indicator {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #CBD5E1;
    border: none;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
    padding: 0;
  }

  .dot.active {
    width: 26px;
    border-radius: 999px;
    background-color: var(--brand-blue);
  }

  .next-btn {
    padding: 12px 28px;
    font-size: 0.98rem;
  }

  .start-btn {
    padding: 13px 32px;
    font-size: 1.02rem;
    box-shadow: 0 4px 16px rgba(19, 108, 252, 0.35);
  }
</style>
