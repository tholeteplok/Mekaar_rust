<script lang="ts">
  import { mekaarStore } from '../stores';
  import {
    Palette,
    Sun,
    Moon,
    Desktop,
    Check,
    Checks,
    X
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

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="appearance-backdrop" onclick={onclose}>
    <div class="appearance-modal" onclick={(e) => e.stopPropagation()}>
      <!-- Header -->
      <div class="modal-header-row">
        <div class="header-title-wrap">
          <span class="header-icon" style="display: flex; align-items: center;">
            <Palette size={24} weight="bold" color="var(--brand-blue, #0284C7)" />
          </span>
          <div>
            <h3 class="dialog-title">Tampilan, Tema & Font</h3>
            <p class="dialog-sub">Personalisasi suasana visual, ukuran huruf & gaya obrolan</p>
          </div>
        </div>
        <button
          type="button"
          class="close-x-btn"
          onclick={onclose}
          aria-label="Tutup Tampilan"
          style="display: flex; align-items: center; justify-content: center;"
        >
          <X size={18} weight="bold" />
        </button>
      </div>

      <!-- Scrollable Body -->
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
      </div>

      <!-- Footer Button -->
      <div class="modal-footer-row">
        <button type="button" class="btn-primary apply-close-btn" onclick={onclose}>
          Selesai & Terapkan
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .appearance-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.65);
    backdrop-filter: blur(5px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 1000;
  }

  .appearance-modal {
    width: min(540px, 100%);
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    background: var(--bg-surface, #FFFFFF);
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: var(--radius-card, 24px);
    box-shadow: 0 20px 50px rgba(15, 23, 42, 0.28);
    overflow: hidden;
  }

  .modal-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 18px 22px;
    border-bottom: 1px solid var(--border-subtle, #E2E8F0);
    background: var(--bg-surface, #FFFFFF);
  }

  .header-title-wrap {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-icon {
    font-size: 1.6rem;
  }

  .dialog-title {
    font-size: 1.12rem;
    font-weight: 800;
    color: var(--text-primary);
    margin: 0;
  }

  .dialog-sub {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin: 2px 0 0;
  }

  .close-x-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: 999px;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
    color: var(--text-secondary);
  }

  .appearance-scroll-body {
    flex: 1;
    overflow-y: auto;
    padding: 18px 22px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  /* 1. Live Chat Preview Box */
  .preview-section-box {
    background: var(--bg-secondary, #F1F5F9);
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: 18px;
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
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .preview-active-tag {
    background: var(--bg-surface);
    color: var(--brand-blue);
    padding: 2px 8px;
    border-radius: 999px;
    border: 0.75px solid var(--border-subtle);
  }

  .live-chat-viewport {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 8px 4px;
    background: #FAF9F6;
    border-radius: 14px;
    border: 1px dashed var(--border-subtle);
  }

  :global([data-theme="dark"]) .live-chat-viewport {
    background: #0B0F17;
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
    border-radius: 999px;
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
    border-radius: 16px;
    position: relative;
    line-height: 1.4;
  }

  .bubble-peer {
    background: var(--bg-surface, #FFFFFF);
    color: var(--text-primary);
    border: 1px solid var(--border-subtle, #E2E8F0);
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
    color: var(--text-muted);
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
    color: var(--text-muted);
    letter-spacing: 0.04em;
  }

  /* 2. Theme Presets Grid */
  .theme-preset-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
  }

  .theme-card-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 12px 8px;
    border-radius: 14px;
    border: 1.5px solid var(--border-subtle);
    background: var(--bg-surface);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .theme-card-btn:hover {
    border-color: var(--brand-blue);
    transform: translateY(-2px);
  }

  .active-theme {
    border-color: var(--brand-blue);
    background: var(--brand-blue-light, #EFF6FF);
    box-shadow: 0 4px 14px rgba(19, 108, 252, 0.15);
  }

  .theme-icon {
    font-size: 1.6rem;
    margin-bottom: 4px;
  }

  .theme-name {
    font-size: 0.88rem;
    font-weight: 700;
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
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: 16px;
    overflow: hidden;
  }

  .font-item-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    border: none;
    background: transparent;
    border-bottom: 1px solid var(--border-subtle);
    cursor: pointer;
    transition: background 0.15s ease;
    text-align: left;
  }

  .font-item-row:last-child {
    border-bottom: none;
  }

  .font-item-row:hover {
    background: var(--bg-secondary);
  }

  .font-selected {
    background: var(--brand-blue-light, #EFF6FF);
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
    color: var(--text-muted);
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
    background: var(--brand-blue-light);
    color: var(--brand-blue);
    font-size: 0.82rem;
    font-weight: 800;
    padding: 2px 10px;
    border-radius: 999px;
  }

  .slider-container-card {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 12px 16px;
    background: var(--bg-secondary);
    border-radius: 14px;
    border: 1px solid var(--border-subtle);
  }

  .size-hint-label {
    font-weight: 800;
    color: var(--text-muted);
  }

  .size-small {
    font-size: 0.85rem;
  }

  .size-large {
    font-size: 1.3rem;
  }

  .text-size-range-input {
    flex: 1;
    accent-color: var(--brand-blue);
    cursor: pointer;
  }

  .slider-ticks-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.72rem;
    color: var(--text-muted);
    padding: 0 4px;
  }

  /* 5. Color Palette Circles */
  .color-palette-row {
    display: flex;
    gap: 14px;
    padding: 6px 0;
  }

  .color-circle-btn {
    width: 38px;
    height: 38px;
    border-radius: 999px;
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
    padding: 14px 22px;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    justify-content: flex-end;
    background: var(--bg-surface);
  }

  .apply-close-btn {
    width: 100%;
    padding: 12px;
    font-size: 0.92rem;
  }
</style>
