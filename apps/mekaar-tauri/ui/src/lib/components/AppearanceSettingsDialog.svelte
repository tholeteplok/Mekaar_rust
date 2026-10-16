<script lang="ts">
  import { mekaarStore } from '../stores';
  import BottomSheet from './BottomSheet.svelte';
  import {
    Sun,
    Moon,
    Desktop,
    Check,
    Checks
  } from 'phosphor-svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open, onclose }: Props = $props();

  const fontOptions = [
    {
      key: 'Plus Jakarta Sans',
      name: 'Plus Jakarta Sans',
      desc: 'Default MEKAAR — Modern & Seimbang',
      sample: 'AaBbCc 123'
    },
    {
      key: 'Inter',
      name: 'Inter',
      desc: 'Netral & Keterbacaan Tinggi',
      sample: 'AaBbCc 123'
    },
    {
      key: 'Comic Neue',
      name: 'Comic Neue',
      desc: 'Playful & Santai Gaya Komik',
      sample: 'AaBbCc 123'
    },
    {
      key: 'Fredoka',
      name: 'Fredoka',
      desc: 'Membulat, Imut & Ramah',
      sample: 'AaBbCc 123'
    },
    {
      key: 'JetBrains Mono',
      name: 'JetBrains Mono',
      desc: 'Taktis, Monospace Kriptografis',
      sample: 'AaBbCc 123'
    }
  ];

  const accentColors = [
    { name: 'Biru Mekaar', hex: '#136CFC' },
    { name: 'Hijau Guardian', hex: '#0D9488' },
    { name: 'Ungu Kripto', hex: '#7C3AED' },
    { name: 'Oranye Taktis', hex: '#EA580C' },
    { name: 'Slate Gelap', hex: '#1E293B' }
  ];

  function handleThemeChange(theme: 'light' | 'dark' | 'system') {
    mekaarStore.setThemePreference(theme);
  }

  function handleFontChange(fontKey: string) {
    mekaarStore.setFontFamily(fontKey);
  }

  function handleScaleChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    mekaarStore.setFontSizeScale(val);
  }

  function handleColorChange(color: string) {
    mekaarStore.setAccentColor(color);
  }
</script>

<BottomSheet
  {open}
  title="Tampilan, Tema & Font"
  subtitle="Personalisasi suasana visual, ukuran huruf & gaya obrolan"
  {onclose}
  maxWidth="560px"
>
  <div class="appearance-scroll-body">
    <!-- 1. LIVE CHAT PREVIEW BOX (Telegram Standard) -->
    <div class="preview-section-box">
      <div class="preview-header-tag">
        <span>PRATINJAU OBROLAN REAL-TIME</span>
        <span class="preview-active-tag">{mekaarStore.fontFamily} • {Math.round(mekaarStore.fontSizeScale * 100)}%</span>
      </div>

      <div
        class="live-chat-viewport"
        style="font-family: var(--font-active); font-size: calc(14.5px * var(--font-scale, 1));"
      >
        <!-- Incoming Bubble -->
        <div class="preview-bubble-row row-left">
          <div class="preview-avatar">A</div>
          <div class="preview-bubble bubble-peer">
            <span class="bubble-sender-name">Ayu Lestari</span>
            <p class="bubble-msg-text">Halo! Bagaimana keterbacaan gaya font dan warna baru ini?</p>
            <div class="bubble-meta-row">
              <span class="bubble-time">11:40</span>
            </div>
          </div>
        </div>

        <!-- Outgoing Bubble (Adaptive with selected Accent Color) -->
        <div class="preview-bubble-row row-right">
          <div class="preview-bubble bubble-self" style="background-color: {mekaarStore.accentColor};">
            <p class="bubble-msg-text">Teks terlihat sangat jernih, rapi, dan mudah dibaca!</p>
            <div class="bubble-meta-row self-meta">
              <span class="bubble-time">11:41</span>
              <span class="bubble-read-ticks" style="display: flex; align-items: center;">
                <Checks size={15} weight="bold" color="rgba(255, 255, 255, 0.9)" />
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 2. MODE TEMA (Light / Dark / System) -->
    <div class="section-group">
      <span class="group-title-label">MODE TEMA</span>
      <div class="theme-preset-grid">
        <button
          type="button"
          class="theme-card-btn {mekaarStore.themePreference === 'light' ? 'active-theme' : ''}"
          onclick={() => handleThemeChange('light')}
        >
          <span class="theme-icon" style="display: flex; align-items: center; justify-content: center;">
            <Sun size={24} weight="bold" color="#F59E0B" />
          </span>
          <span class="theme-name">Terang</span>
          <span class="theme-desc">Kanvas bersih & kontras tinggi</span>
        </button>

        <button
          type="button"
          class="theme-card-btn {mekaarStore.themePreference === 'dark' ? 'active-theme' : ''}"
          onclick={() => handleThemeChange('dark')}
        >
          <span class="theme-icon" style="display: flex; align-items: center; justify-content: center;">
            <Moon size={24} weight="bold" color="#818CF8" />
          </span>
          <span class="theme-name">Gelap</span>
          <span class="theme-desc">OLED hitam & hemat baterai</span>
        </button>

        <button
          type="button"
          class="theme-card-btn {mekaarStore.themePreference === 'system' ? 'active-theme' : ''}"
          onclick={() => handleThemeChange('system')}
        >
          <span class="theme-icon" style="display: flex; align-items: center; justify-content: center;">
            <Desktop size={24} weight="bold" color="var(--brand-blue, #0284C7)" />
          </span>
          <span class="theme-name">Sistem</span>
          <span class="theme-desc">Otomatis sinkron preferensi OS</span>
        </button>
      </div>
    </div>

    <!-- 3. PILIHAN GAYA FONT (5 Family Presets - Paritas Mekaar_chat) -->
    <div class="section-group">
      <span class="group-title-label">TIPOGRAFI & GAYA TULISAN</span>
      <div class="font-selector-list">
        {#each fontOptions as f}
          <button
            type="button"
            class="font-item-row {mekaarStore.fontFamily === f.key ? 'font-selected' : ''}"
            onclick={() => handleFontChange(f.key)}
          >
            <div class="font-left-info">
              <span class="font-display-name" style="font-family: '{f.key}', sans-serif;">
                {f.name}
              </span>
              <span class="font-desc-sub">{f.desc}</span>
            </div>
            <div class="font-sample-preview" style="font-family: '{f.key}', sans-serif;">
              {f.sample}
            </div>
            <div class="font-radio-check">
              {mekaarStore.fontFamily === f.key ? '●' : '○'}
            </div>
          </button>
        {/each}
      </div>
    </div>

    <!-- 4. SLIDER UKURAN TEKS (Telegram Style Text Size Slider) -->
    <div class="section-group">
      <div class="slider-header-row">
        <span class="group-title-label">UKURAN TEKS OBROLAN</span>
        <span class="slider-value-pill">{Math.round(mekaarStore.fontSizeScale * 100)}%</span>
      </div>

      <div class="slider-container-card">
        <span class="size-hint-label size-small">A</span>
        <input
          type="range"
          min="0.85"
          max="1.25"
          step="0.05"
          value={mekaarStore.fontSizeScale}
          oninput={handleScaleChange}
          class="text-size-range-input"
          aria-label="Pengatur Ukuran Teks"
        />
        <span class="size-hint-label size-large">A</span>
      </div>
      <div class="slider-ticks-row">
        <span>Kecil (85%)</span>
        <span>Standar (100%)</span>
        <span>Besar (115%)</span>
        <span>Ekstra (125%)</span>
      </div>
    </div>

    <!-- 5. PALET WARNA AKSEN OBROLAN -->
    <div class="section-group">
      <span class="group-title-label">WARNA AKSEN GELEMBUNG PESAN</span>
      <div class="color-palette-row">
        {#each accentColors as c}
          <button
            type="button"
            class="color-circle-btn {mekaarStore.accentColor === c.hex ? 'color-active' : ''}"
            style="background-color: {c.hex};"
            onclick={() => handleColorChange(c.hex)}
            title={c.name}
            aria-label={c.name}
          >
            {#if mekaarStore.accentColor === c.hex}
              <span class="white-check-mark" style="display: flex; align-items: center; justify-content: center;">
                <Check size={16} weight="bold" color="#FFFFFF" />
              </span>
            {/if}
          </button>
        {/each}
      </div>
    </div>

    <!-- Footer Button -->
    <div class="modal-footer-row">
      <button type="button" class="btn-primary apply-close-btn" onclick={onclose}>
        Selesai & Terapkan
      </button>
    </div>
  </div>
</BottomSheet>

<style>
  .appearance-scroll-body {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-height: 70vh;
    overflow-y: auto;
  }

  /* 1. Live Chat Preview Box */
  .preview-section-box {
    background: var(--bg-primary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-lg);
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .preview-header-tag {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.75rem;
    font-weight: 800;
    font-family: var(--font-active);
    letter-spacing: 0.05em;
    color: var(--text-secondary);
  }

  .preview-active-tag {
    background: var(--bg-card);
    color: var(--brand-blue);
    padding: 2px 8px;
    border-radius: var(--radius-pill);
    border: 0.75px solid var(--border-light);
  }

  .live-chat-viewport {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 8px 4px;
    background: var(--bg-card);
    border-radius: var(--radius-md);
    border: 1px dashed var(--border-light);
  }

  .preview-bubble-row {
    display: flex;
    align-items: flex-end;
    gap: 8px;
  }

  .row-left {
    justify-content: flex-start;
  }

  .row-right {
    justify-content: flex-end;
  }

  .preview-avatar {
    width: 30px;
    height: 30px;
    border-radius: var(--radius-pill);
    background: #BFDBFE;
    color: #1E3A8A;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8rem;
    flex-shrink: 0;
  }

  .preview-bubble {
    max-width: 80%;
    padding: 10px 14px;
    border-radius: var(--radius-lg);
    position: relative;
    line-height: 1.4;
  }

  .bubble-peer {
    background: var(--bg-card);
    color: var(--text-primary);
    border: 1px solid var(--border-light);
    border-bottom-left-radius: 4px;
  }

  .bubble-self {
    color: #FFFFFF;
    border-bottom-right-radius: 4px;
    box-shadow: 0 2px 8px rgba(19, 108, 252, 0.25);
  }

  .bubble-sender-name {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--brand-blue);
    display: block;
    margin-bottom: 2px;
  }

  .bubble-msg-text {
    margin: 0;
  }

  .bubble-meta-row {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 4px;
    margin-top: 4px;
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .self-meta {
    color: rgba(255, 255, 255, 0.85);
  }

  .bubble-read-ticks {
    font-weight: 700;
  }

  /* Section Groups */
  .section-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .group-title-label {
    font-size: 0.78rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-secondary);
    letter-spacing: 0.04em;
  }

  /* 2. Theme Presets Grid */
  .theme-preset-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
  }

  .theme-card-btn {
    min-height: 48px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 12px 8px;
    border-radius: var(--radius-md);
    border: 1.5px solid var(--border-light);
    background: var(--bg-card);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .theme-card-btn:hover {
    border-color: var(--brand-blue);
    transform: translateY(-2px);
  }

  .active-theme {
    border-color: var(--brand-blue);
    background: var(--brand-blue-subtle);
    box-shadow: 0 4px 14px rgba(19, 108, 252, 0.15);
  }

  .theme-icon {
    font-size: 1.6rem;
    margin-bottom: 4px;
  }

  .theme-name {
    font-size: 0.88rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary);
  }

  .theme-desc {
    font-size: 0.7rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  /* 3. Font Family Selector */
  .font-selector-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: var(--bg-card);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .font-item-row {
    min-height: 48px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    border: none;
    background: transparent;
    border-bottom: 1px solid var(--border-light);
    cursor: pointer;
    transition: background 0.15s ease;
    text-align: left;
  }

  .font-item-row:last-child {
    border-bottom: none;
  }

  .font-item-row:hover {
    background: var(--bg-primary);
  }

  .font-selected {
    background: var(--brand-blue-subtle);
  }

  .font-left-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }

  .font-display-name {
    font-size: 0.94rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .font-desc-sub {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .font-sample-preview {
    font-size: 0.85rem;
    color: var(--text-secondary);
    padding: 0 12px;
  }

  .font-radio-check {
    font-size: 1.1rem;
    color: var(--brand-blue);
  }

  /* 4. Text Size Range Slider */
  .slider-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .slider-value-pill {
    background: var(--brand-blue-subtle);
    color: var(--brand-blue);
    font-size: 0.82rem;
    font-weight: 800;
    padding: 2px 10px;
    border-radius: var(--radius-pill);
  }

  .slider-container-card {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 12px 16px;
    background: var(--bg-primary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-light);
  }

  .size-hint-label {
    font-weight: 800;
    color: var(--text-secondary);
  }

  .size-small {
    font-size: 0.85rem;
  }

  .size-large {
    font-size: 1.3rem;
  }

  .text-size-range-input {
    flex: 1;
    min-height: var(--touch-target-min, 40px);
    accent-color: var(--brand-blue);
    cursor: pointer;
  }

  .slider-ticks-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.72rem;
    color: var(--text-secondary);
    padding: 0 4px;
  }

  /* 5. Color Palette Circles */
  .color-palette-row {
    display: flex;
    gap: 14px;
    padding: 6px 0;
  }

  .color-circle-btn {
    width: 40px;
    height: 40px;
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    border-radius: var(--radius-pill);
    border: 3px solid transparent;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.2s ease, border-color 0.2s ease;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
  }

  .color-circle-btn:hover {
    transform: scale(1.1);
  }

  .color-active {
    border-color: #FFFFFF;
    outline: 2.5px solid var(--brand-blue);
  }

  .white-check-mark {
    color: #FFFFFF;
    font-size: 0.95rem;
    font-weight: 800;
  }

  /* Modal Footer */
  .modal-footer-row {
    padding-top: 10px;
    display: flex;
    justify-content: flex-end;
  }

  .apply-close-btn {
    width: 100%;
    min-height: var(--touch-target-min, 40px);
    padding: 10px 16px;
    font-size: 0.92rem;
    font-weight: 700;
    font-family: var(--font-active);
    background-color: var(--brand-blue);
    color: #FFFFFF;
    border: none;
    border-radius: var(--radius-pill);
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(37, 99, 235, 0.25);
  }

  .apply-close-btn:hover {
    background-color: #1D4ED8;
  }
</style>
