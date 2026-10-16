<script lang="ts">
  import { onMount } from 'svelte';
  import { mekaarStore } from '../stores';
  import BrandIcon from '../components/BrandIcon.svelte';

  let isLoaded = $state(false);

  onMount(() => {
    // Efek fade-in halus saat startup
    const timerAnim = setTimeout(() => {
      isLoaded = true;
    }, 100);

    // Delay 900ms untuk brand impression & inisialisasi deterministik
    const timerAdvance = setTimeout(() => {
      const nextStep = mekaarStore.resolveSplashNextStep();
      mekaarStore.advanceFtue(nextStep);
    }, 1000);

    return () => {
      clearTimeout(timerAnim);
      clearTimeout(timerAdvance);
    };
  });
</script>

<div class="splash-container">
  <div class="splash-content {isLoaded ? 'visible' : ''}">
    <!-- Emblem Brand Icon Resmi MEKAAR -->
    <div class="icon-glow-wrap">
      <BrandIcon size={84} variant="raw" alt="Logo MEKAAR" />
    </div>

    <!-- Wordmark & Tagline Resmi (Mekaar_chat parity) -->
    <div class="brand-text-block">
      <h1 class="brand-wordmark">
        <span class="part-yellow">Mek</span><span class="part-cyan">aar</span>
      </h1>
      <p class="tagline">Bicara bebas. Tetap aman.</p>
    </div>

    <!-- Indicator Subtle Loading -->
    <div class="loading-bar">
      <div class="loading-progress"></div>
    </div>
  </div>

  <div class="footer-note">
    <span>E2EE & Peer-to-Peer Relay</span>
  </div>
</div>

<style>
  .splash-container {
    position: fixed;
    inset: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    background: radial-gradient(circle at 50% 35%, #17243B 0%, #0B111A 100%);
    color: #FFFFFF;
    padding: calc(48px + env(safe-area-inset-top, 0px)) 24px calc(32px + env(safe-area-inset-bottom, 0px));
    box-sizing: border-box;
    z-index: 9999;
    user-select: none;
    -webkit-user-select: none;
  }

  .splash-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    opacity: 0;
    transform: translateY(14px) scale(0.97);
    transition: opacity 0.45s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.45s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .splash-content.visible {
    opacity: 1;
    transform: translateY(0) scale(1);
  }

  .icon-glow-wrap {
    width: 108px;
    height: 108px;
    border-radius: 26px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.12);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 12px 36px rgba(19, 108, 252, 0.28);
    margin-bottom: 24px;
    backdrop-filter: blur(8px);
  }

  .brand-text-block {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 32px;
  }

  .brand-wordmark {
    font-family: var(--font-sans);
    font-size: 2.4rem;
    font-weight: 800;
    letter-spacing: -0.5px;
    margin: 0;
    line-height: 1.1;
  }

  .part-yellow {
    color: #FBBF24; /* Mekaar brand yellow */
  }

  .part-cyan {
    color: #38BDF8; /* Mekaar brand cyan */
  }

  .tagline {
    font-size: 0.95rem;
    color: rgba(255, 255, 255, 0.72);
    font-weight: 500;
    letter-spacing: 0.2px;
    margin-top: 8px;
  }

  .loading-bar {
    width: 140px;
    height: 4px;
    background: rgba(255, 255, 255, 0.12);
    border-radius: 999px;
    overflow: hidden;
    position: relative;
  }

  .loading-progress {
    position: absolute;
    height: 100%;
    width: 40%;
    background: linear-gradient(90deg, #38BDF8, #FBBF24);
    border-radius: 999px;
    animation: splash-sweep 1.2s infinite ease-in-out;
  }

  @keyframes splash-sweep {
    0% {
      left: -40%;
    }
    100% {
      left: 100%;
    }
  }

  .footer-note {
    font-size: 0.76rem;
    color: rgba(255, 255, 255, 0.4);
    font-family: var(--font-mono);
    letter-spacing: 0.5px;
  }
</style>
