<script lang="ts">
  import { mekaarStore } from '../stores';
  import BottomSheet from './BottomSheet.svelte';
  import type { Conversation } from '../types';
  import {
    ShieldCheck,
    DeviceMobileSlash,
    Prohibit,
    EyeSlash,
    Hourglass,
    Alarm,
    Fire,
    Copy,
    Check
  } from 'phosphor-svelte';

  interface Props {
    open: boolean;
    conversation: Conversation | null;
    onclose: () => void;
    onopenSas?: () => void;
  }

  let { open, conversation, onclose, onopenSas }: Props = $props();

  let isScreenProtected = $state(false);
  let isForwardingProtected = $state(false);
  let isViewOnceDefault = $state(false);
  let disappearingSeconds = $state(0);
  let scheduledWipeMode = $state<'off' | 'one_shot' | 'daily'>('off');
  let scheduledWipeTime = $state('23:59');
  let isBurnOnExit = $state(false);
  let burnOnExitScope = $state<'local' | 'mutual'>('local');

  let toastMessage = $state('');

  function showToast(msg: string) {
    toastMessage = msg;
    setTimeout(() => {
      if (toastMessage === msg) toastMessage = '';
    }, 2800);
  }

  $effect(() => {
    if (conversation && open) {
      isScreenProtected = !!conversation.isScreenProtected;
      isForwardingProtected = !!conversation.isForwardingProtected;
      isViewOnceDefault = !!conversation.isViewOnceDefault;
      disappearingSeconds = conversation.disappearingSeconds || 0;
      scheduledWipeMode = conversation.scheduledWipeMode || 'off';
      scheduledWipeTime = conversation.scheduledWipeTime || '23:59';
      isBurnOnExit = !!conversation.isBurnOnExit;
      burnOnExitScope = conversation.burnOnExitScope || 'local';
    }
  });

  const disappearingOptions = [
    { label: 'Nonaktif', sec: 0 },
    { label: '30 dtk', sec: 30 },
    { label: '5 mnt', sec: 300 },
    { label: '1 jam', sec: 3600 },
    { label: '24 jam', sec: 86400 },
    { label: '7 hari', sec: 604800 }
  ];

  function handleSave() {
    if (!conversation) return;
    mekaarStore.updateRoomPrivacy(conversation.id, {
      isScreenProtected,
      isForwardingProtected,
      isViewOnceDefault,
      disappearingSeconds,
      scheduledWipeMode,
      scheduledWipeTime,
      isBurnOnExit,
      burnOnExitScope
    });
    showToast('✓ Pengaturan privasi obrolan berhasil diterapkan!');
    setTimeout(() => {
      onclose();
    }, 500);
  }

  function handleCopyFingerprint(text: string) {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(text);
      showToast('Sidik jari E2EE disalin ke papan klip');
    }
  }
</script>

<BottomSheet {open} title="Privasi Obrolan Ekstrem" {onclose}>
  {#if conversation}
    <div class="privacy-sheet-container">
      <!-- Toast Feedback Floating -->
      {#if toastMessage}
        <div class="feedback-toast-pill">
          {toastMessage}
        </div>
      {/if}

      <!-- 1. Mutual Shield Notice Card -->
      <div class="mutual-shield-card">
        <div class="shield-badge-icon" style="display: flex; align-items: center; justify-content: center;">
          <ShieldCheck size={28} weight="bold" color="var(--brand-teal, #0D9488)" />
        </div>
        <div class="shield-desc-col">
          <span class="shield-title">Proteksi 2 Arah (Mutual Shield)</span>
          <p class="shield-text">
            Kebijakan privasi di ruang obrolan <strong>{conversation.title}</strong> berlaku untuk <em>kedua belah pihak</em>. Jika salah satu pihak mengaktifkan proteksi layar atau konten, kedua perangkat otomatis terlindungi serentak.
          </p>
        </div>
      </div>

      <!-- 2. Section: Proteksi Layar & Konten (2 Arah) -->
      <div class="setting-group-box">
        <div class="group-header-row">
          <span class="micro-title">PROTEKSI LAYAR & KONTEN (2 ARAH)</span>
          <span class="active-badge-tag">Mutual Shield</span>
        </div>

        <!-- Toggle: Anti-Screenshot (2 Arah) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="setting-interactive-row"
          onclick={() => (isScreenProtected = !isScreenProtected)}
        >
          <div class="row-icon-cell icon-teal" style="display: flex; align-items: center; justify-content: center;">
            <DeviceMobileSlash size={22} weight="bold" color="var(--brand-teal, #0D9488)" />
          </div>
          <div class="row-text-cell">
            <span class="row-title">Blokir Screenshot & Tangkapan Layar</span>
            <span class="row-desc">
              Menolak perekaman layar OS, tangkapan layar (Win32 affinity / Android FLAG_SECURE) di perangkat kedua belah pihak.
            </span>
          </div>
          <div class="switch-ui {isScreenProtected ? 'switch-on' : ''}">
            <span class="switch-handle"></span>
          </div>
        </div>

        <div class="row-divider"></div>

        <!-- Toggle: Larang Teruskan & Salin Pesan (2 Arah) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="setting-interactive-row"
          onclick={() => (isForwardingProtected = !isForwardingProtected)}
        >
          <div class="row-icon-cell icon-amber" style="display: flex; align-items: center; justify-content: center;">
            <Prohibit size={22} weight="bold" color="#D97706" />
          </div>
          <div class="row-text-cell">
            <span class="row-title">Larang Teruskan & Salin Pesan</span>
            <span class="row-desc">
              Penerima tidak dapat meneruskan (*forward*) teks/media ke ruang obrolan lain atau menyalinnya ke papan klip.
            </span>
          </div>
          <div class="switch-ui {isForwardingProtected ? 'switch-on' : ''}">
            <span class="switch-handle"></span>
          </div>
        </div>

        <div class="row-divider"></div>

        <!-- Toggle: Media Sekali Lihat Default (View-Once) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="setting-interactive-row"
          onclick={() => (isViewOnceDefault = !isViewOnceDefault)}
        >
          <div class="row-icon-cell icon-purple" style="display: flex; align-items: center; justify-content: center;">
            <EyeSlash size={22} weight="bold" color="#8B5CF6" />
          </div>
          <div class="row-text-cell">
            <span class="row-title">Mode Sekali Lihat (View-Once) Default</span>
            <span class="row-desc">
              Setiap foto dan video yang dikirimkan dalam ruangan ini otomatis berstatus sekali lihat dan musnah setelah ditutup.
            </span>
          </div>
          <div class="switch-ui {isViewOnceDefault ? 'switch-on' : ''}">
            <span class="switch-handle"></span>
          </div>
        </div>
      </div>

      <!-- 3. Section: Penghapusan Pesan Otomatis (2 Arah) -->
      <div class="setting-group-box">
        <div class="group-header-row">
          <span class="micro-title">SIKLUS HIDUP & PENGHAPUSAN OTOMATIS</span>
          <span class="active-badge-tag tag-rose">Wipe 2 Arah</span>
        </div>

        <!-- Timer Pesan Lenyap (Countdown Wipe - 2 Arah) -->
        <div class="setting-block-row">
          <div class="block-top-row">
            <div class="row-icon-cell icon-rose" style="display: flex; align-items: center; justify-content: center;">
              <Hourglass size={22} weight="bold" color="#E11D48" />
            </div>
            <div class="row-text-cell">
              <span class="row-title">Pesan Lenyap Otomatis (Countdown Wipe)</span>
              <span class="row-desc">
                Menghapus seluruh pesan (pesan sendiri + pesan rekan) di kedua perangkat secara permanen.
              </span>
            </div>
          </div>

          <div class="pills-selector-wrap">
            {#each disappearingOptions as opt}
              <button
                type="button"
                class="timer-pill-btn {disappearingSeconds === opt.sec ? 'pill-active' : ''}"
                onclick={() => (disappearingSeconds = opt.sec)}
              >
                {opt.label}
              </button>
            {/each}
          </div>
        </div>

        <div class="row-divider"></div>

        <!-- Pembersihan Terjadwal Ruangan (Scheduled Wipe) -->
        <div class="setting-block-row">
          <div class="block-top-row">
            <div class="row-icon-cell icon-blue" style="display: flex; align-items: center; justify-content: center;">
              <Alarm size={22} weight="bold" color="var(--brand-blue, #0284C7)" />
            </div>
            <div class="row-text-cell">
              <span class="row-title">Pembersihan Terjadwal Ruangan</span>
              <span class="row-desc">
                Wipe zeroing seluruh percakapan pada jam tertentu (pesan sendiri & rekan).
              </span>
            </div>
          </div>

          <div class="scheduled-controls-row">
            <div class="segmented-control">
              <button
                type="button"
                class="segment-btn {scheduledWipeMode === 'off' ? 'segment-active' : ''}"
                onclick={() => (scheduledWipeMode = 'off')}
              >
                Nonaktif
              </button>
              <button
                type="button"
                class="segment-btn {scheduledWipeMode === 'one_shot' ? 'segment-active' : ''}"
                onclick={() => (scheduledWipeMode = 'one_shot')}
              >
                1 Kali
              </button>
              <button
                type="button"
                class="segment-btn {scheduledWipeMode === 'daily' ? 'segment-active' : ''}"
                onclick={() => (scheduledWipeMode = 'daily')}
              >
                Harian
              </button>
            </div>

            {#if scheduledWipeMode !== 'off'}
              <div class="time-picker-box">
                <span style="font-size: 0.8rem; font-weight: 600; color: var(--text-secondary);">Pukul:</span>
                <input
                  type="time"
                  class="time-input-field"
                  bind:value={scheduledWipeTime}
                />
              </div>
            {/if}
          </div>
        </div>

        <div class="row-divider"></div>

        <!-- Hapus Saat Keluar (Burn on Exit) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="setting-interactive-row"
          onclick={() => (isBurnOnExit = !isBurnOnExit)}
        >
          <div class="row-icon-cell icon-red" style="display: flex; align-items: center; justify-content: center;">
            <Fire size={22} weight="bold" color="#EF4444" />
          </div>
          <div class="row-text-cell">
            <span class="row-title" style="color: #E11D48;">Hapus Saat Keluar (Burn on Exit)</span>
            <span class="row-desc">
              Musnahkan seluruh pesan seketika saat Anda menutup atau keluar dari ruang obrolan ini.
            </span>
          </div>
          <div class="switch-ui switch-red {isBurnOnExit ? 'switch-on' : ''}">
            <span class="switch-handle"></span>
          </div>
        </div>

        <!-- Cakupan Burn on Exit (Hanya jika aktif) -->
        {#if isBurnOnExit}
          <div class="burn-scope-card">
            <span class="scope-title">Pilih Cakupan Pemusnahan (Scope):</span>
            <div class="scope-options-row">
              <button
                type="button"
                class="scope-btn {burnOnExitScope === 'local' ? 'scope-active' : ''}"
                onclick={() => (burnOnExitScope = 'local')}
              >
                <div class="scope-btn-header">
                  <span class="scope-radio-dot">{burnOnExitScope === 'local' ? '●' : '○'}</span>
                  <strong>Hanya Perangkat Saya (1 Arah)</strong>
                </div>
                <span class="scope-btn-desc">Hanya membersihkan cache lokal pada perangkat Anda saat keluar.</span>
              </button>

              <button
                type="button"
                class="scope-btn scope-mutual {burnOnExitScope === 'mutual' ? 'scope-active-mutual' : ''}"
                onclick={() => (burnOnExitScope = 'mutual')}
              >
                <div class="scope-btn-header">
                  <span class="scope-radio-dot">{burnOnExitScope === 'mutual' ? '●' : '○'}</span>
                  <strong>Kedua Belah Pihak (2 Arah)</strong>
                </div>
                <span class="scope-btn-desc">Kirim perintah pemusnahan instan untuk menghapus bersih pesan di kedua perangkat.</span>
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- 4. Section: Identitas Kriptografi E2EE & SAS -->
      <div class="setting-group-box">
        <div class="group-header-row">
          <span class="micro-title">KEAMANAN & VERIFIKASI ENKRIPSI</span>
          <span class="active-badge-tag tag-blue">Aegis Protocol Shield</span>
        </div>

        <div class="crypto-info-card">
          <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 6px;">
            <span style="font-size: 0.82rem; font-weight: 600; color: var(--text-main);">Kode Sidik Jari Obrolan</span>
            <button
              type="button"
              class="copy-micro-btn"
              onclick={() => handleCopyFingerprint('ED25519: 9A74-E122-9A9B-718E-4F10')}
              style="display: flex; align-items: center; gap: 4px;"
            >
              <Copy size={13} weight="bold" />
              <span>Salin</span>
            </button>
          </div>
          <code class="fingerprint-display">ED25519: 9A74-E122-9A9B-718E-4F10</code>
          
          {#if onopenSas}
            <button
              type="button"
              class="verify-sas-action-btn"
              onclick={() => {
                onclose();
                onopenSas();
              }}
              style="display: flex; align-items: center; justify-content: center; gap: 6px;"
            >
              <ShieldCheck size={16} weight="bold" />
              <span>Bandingkan Kode SAS (Aegis Shield)</span>
            </button>
          {/if}
        </div>
      </div>

      <!-- 5. Footer Actions -->
      <div class="sheet-action-footer">
        <button
          type="button"
          class="btn-cancel"
          onclick={onclose}
        >
          Batal
        </button>
        <button
          type="button"
          class="btn-apply-privacy"
          onclick={handleSave}
        >
          ✓ Terapkan Pengaturan Privasi
        </button>
      </div>
    </div>
  {/if}
</BottomSheet>

<style>
  .privacy-sheet-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 12px;
    position: relative;
  }

  .feedback-toast-pill {
    position: absolute;
    top: -8px;
    left: 50%;
    transform: translateX(-50%);
    background: #0F172A;
    color: #38BDF8;
    font-size: 0.82rem;
    font-weight: 600;
    padding: 6px 14px;
    border-radius: 999px;
    border: 1.5px solid #38BDF8;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    z-index: 100;
    pointer-events: none;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translate(-50%, -8px); }
    to { opacity: 1; transform: translate(-50%, 0); }
  }

  /* Mutual Shield Alert Banner */
  .mutual-shield-card {
    display: flex;
    gap: 12px;
    padding: 12px 14px;
    background: rgba(13, 148, 136, 0.08);
    border: 1.5px solid var(--brand-teal);
    border-radius: var(--radius-md, 12px);
    box-shadow: 2px 2px 0px rgba(13, 148, 136, 0.25);
  }

  .shield-badge-icon {
    font-size: 1.8rem;
    flex-shrink: 0;
  }

  .shield-desc-col {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .shield-title {
    font-size: 0.92rem;
    font-weight: 700;
    color: var(--brand-teal);
  }

  .shield-text {
    font-size: 0.8rem;
    line-height: 1.45;
    color: var(--text-secondary);
    margin: 0;
  }

  /* Setting Group Box */
  .setting-group-box {
    background: var(--bg-card);
    border: 1.5px solid var(--border-sketch-color, #E2E8F0);
    border-radius: var(--radius-md, 12px);
    padding: 14px;
    box-shadow: 2px 2px 0px rgba(0, 0, 0, 0.04);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .group-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 2px;
  }

  .micro-title {
    font-size: 0.72rem;
    font-weight: 800;
    letter-spacing: 0.06em;
    color: var(--text-muted);
  }

  .active-badge-tag {
    font-size: 0.7rem;
    font-weight: 700;
    padding: 2px 8px;
    background: rgba(13, 148, 136, 0.12);
    color: var(--brand-teal);
    border-radius: 999px;
    border: 1px solid var(--brand-teal);
  }

  .tag-rose {
    background: rgba(225, 29, 72, 0.1);
    color: #E11D48;
    border-color: #E11D48;
  }

  .tag-blue {
    background: rgba(14, 116, 144, 0.1);
    color: var(--brand-blue);
    border-color: var(--brand-blue);
  }

  /* Setting Interactive Rows */
  .setting-interactive-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 2px;
    cursor: pointer;
    user-select: none;
  }

  .row-icon-cell {
    width: 38px;
    height: 38px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    flex-shrink: 0;
    border: 1.5px solid rgba(0,0,0,0.06);
  }

  .icon-teal { background: #CCFBF1; }
  .icon-amber { background: #FEF3C7; }
  .icon-purple { background: #EDE9FE; }
  .icon-rose { background: #FFE4E6; }
  .icon-blue { background: #E0F2FE; }
  .icon-red { background: #FEE2E2; }

  .row-text-cell {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .row-title {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--text-main);
  }

  .row-desc {
    font-size: 0.76rem;
    color: var(--text-secondary);
    line-height: 1.35;
  }

  .row-divider {
    height: 1px;
    background: var(--border-light, #F1F5F9);
    margin: 2px 0;
  }

  /* Custom Switch */
  .switch-ui {
    width: 44px;
    height: 24px;
    background: var(--bg-secondary, #E2E8F0);
    border-radius: 999px;
    position: relative;
    transition: background 0.2s ease;
    flex-shrink: 0;
    border: 1.5px solid var(--sketch-border-color, #CBD5E1);
  }

  .switch-handle {
    width: 18px;
    height: 18px;
    background: #FFFFFF;
    border-radius: 50%;
    position: absolute;
    top: 1.5px;
    left: 2px;
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .switch-on {
    background: var(--brand-teal, #0D9488);
    border-color: var(--brand-teal, #0D9488);
  }

  .switch-red.switch-on {
    background: #E11D48;
    border-color: #E11D48;
  }

  .switch-on .switch-handle {
    transform: translateX(18px);
  }

  /* Setting Block Row (Pills & Controls) */
  .setting-block-row {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 6px 2px;
  }

  .block-top-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .pills-selector-wrap {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-left: 50px;
  }

  .timer-pill-btn {
    font-size: 0.78rem;
    font-weight: 600;
    padding: 5px 12px;
    border-radius: 999px;
    border: 1.5px solid var(--border-sketch-color, #CBD5E1);
    background: var(--bg-main, #FFFFFF);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .timer-pill-btn:hover {
    border-color: var(--brand-blue);
    color: var(--brand-blue);
  }

  .timer-pill-btn.pill-active {
    background: #FFE4E6;
    color: #E11D48;
    border-color: #E11D48;
    font-weight: 700;
    box-shadow: 1px 1px 0px #E11D48;
  }

  /* Scheduled Controls Row */
  .scheduled-controls-row {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-left: 50px;
    flex-wrap: wrap;
  }

  .segmented-control {
    display: flex;
    background: var(--bg-secondary, #F1F5F9);
    border: 1.5px solid var(--sketch-border-color, #CBD5E1);
    border-radius: 8px;
    padding: 2px;
  }

  .segment-btn {
    font-size: 0.78rem;
    font-weight: 600;
    padding: 5px 10px;
    border: none;
    background: transparent;
    border-radius: 6px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .segment-btn.segment-active {
    background: var(--bg-main, #FFFFFF);
    color: var(--brand-blue);
    font-weight: 700;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .time-picker-box {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .time-input-field {
    font-family: inherit;
    font-size: 0.84rem;
    font-weight: 600;
    padding: 4px 8px;
    border-radius: 6px;
    border: 1.5px solid var(--sketch-border-color, #CBD5E1);
    background: var(--bg-main, #FFFFFF);
    color: var(--text-main);
  }

  /* Burn on Exit Scope Options */
  .burn-scope-card {
    margin-left: 50px;
    padding: 10px 12px;
    background: #FFF1F2;
    border: 1.5px dashed #FDA4AF;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .scope-title {
    font-size: 0.76rem;
    font-weight: 700;
    color: #BE123C;
  }

  .scope-options-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .scope-btn {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 8px 10px;
    background: #FFFFFF;
    border: 1.5px solid #FECDD3;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease;
  }

  .scope-btn-header {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.82rem;
    color: var(--text-main);
  }

  .scope-radio-dot {
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .scope-btn-desc {
    font-size: 0.72rem;
    color: var(--text-secondary);
    margin-left: 16px;
    line-height: 1.3;
  }

  .scope-btn.scope-active {
    border-color: #E11D48;
    background: #FFF1F2;
  }

  .scope-btn.scope-active .scope-radio-dot {
    color: #E11D48;
  }

  .scope-btn.scope-active-mutual {
    border-color: #BE123C;
    background: #FFE4E6;
    box-shadow: 1px 1px 0px #BE123C;
  }

  .scope-btn.scope-active-mutual .scope-radio-dot {
    color: #BE123C;
  }

  /* Crypto Info Card */
  .crypto-info-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: var(--bg-main, #FFFFFF);
    padding: 10px 12px;
    border-radius: 8px;
    border: 1px solid var(--border-light, #E2E8F0);
  }

  .copy-micro-btn {
    font-size: 0.74rem;
    font-weight: 600;
    color: var(--brand-blue);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 2px 4px;
  }

  .fingerprint-display {
    font-family: monospace;
    font-size: 0.8rem;
    background: var(--bg-secondary, #F1F5F9);
    padding: 6px 8px;
    border-radius: 6px;
    color: var(--text-main);
    word-break: break-all;
    border: 1px solid var(--border-light, #CBD5E1);
  }

  .verify-sas-action-btn {
    margin-top: 4px;
    padding: 8px 12px;
    font-size: 0.82rem;
    font-weight: 700;
    color: var(--brand-blue);
    background: rgba(14, 116, 144, 0.08);
    border: 1.5px solid var(--brand-blue);
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    transition: all 0.15s ease;
  }

  .verify-sas-action-btn:hover {
    background: rgba(14, 116, 144, 0.15);
  }

  /* Sheet Action Footer */
  .sheet-action-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 8px;
  }

  .btn-cancel {
    padding: 9px 16px;
    font-size: 0.86rem;
    font-weight: 600;
    color: var(--text-secondary);
    background: transparent;
    border: 1.5px solid var(--sketch-border-color, #CBD5E1);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-cancel:hover {
    background: var(--bg-secondary);
  }

  .btn-apply-privacy {
    padding: 9px 18px;
    font-size: 0.86rem;
    font-weight: 700;
    color: #FFFFFF;
    background: var(--brand-teal, #0D9488);
    border: 1.5px solid #0F766E;
    border-radius: 8px;
    cursor: pointer;
    box-shadow: 2px 2px 0px #0F766E;
    transition: all 0.15s ease;
  }

  .btn-apply-privacy:hover {
    transform: translate(-1px, -1px);
    box-shadow: 3px 3px 0px #0F766E;
  }

  .btn-apply-privacy:active {
    transform: translate(1px, 1px);
    box-shadow: 1px 1px 0px #0F766E;
  }
</style>
