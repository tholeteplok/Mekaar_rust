<script lang="ts">
  import { mekaarStore } from '../stores';
  import BottomSheet from './BottomSheet.svelte';

  let {
    open = false,
    onclose
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  let selectedMode = $state<'off' | 'one_shot' | 'daily'>('off');
  let selectedTime = $state('02:00');
  let isSaved = $state(false);

  $effect(() => {
    if (open) {
      selectedMode = mekaarStore.scheduledWipe.mode;
      selectedTime = mekaarStore.scheduledWipe.time || '02:00';
      isSaved = false;
    }
  });

  const targetDescription = $derived.by(() => {
    if (selectedMode === 'off') {
      return 'Pembersihan terjadwal dinonaktifkan. Riwayat obrolan disimpan sampai dihapus manual.';
    }
    const [h, m] = selectedTime.split(':').map(Number);
    const now = new Date();
    const target = new Date();
    target.setHours(h, m, 0, 0);

    const isTomorrow = target.getTime() <= now.getTime();
    const dayLabel = isTomorrow ? 'Besok' : 'Hari ini';

    if (selectedMode === 'one_shot') {
      return `Pembersihan sekali jalan akan dieksekusi pada ${dayLabel} pukul ${selectedTime} WIB. Setelah itu mode kembali nonaktif.`;
    } else {
      return `Pembersihan berulang harian otomatis dieksekusi setiap malam pukul ${selectedTime} WIB secara berkala.`;
    }
  });

  async function handleSave() {
    await mekaarStore.setScheduledWipe(selectedMode, selectedTime);
    isSaved = true;
    setTimeout(() => {
      onclose();
    }, 700);
  }
</script>

<BottomSheet
  {open}
  title="Pembersihan Pesan Terjadwal"
  subtitle="Pemusnahan kriptografi otomatis riwayat obrolan berbasis jam"
  {onclose}
  maxWidth="520px"
>
  <div class="sheet-body">
    <div class="mode-options-grid">
      <!-- Option: OFF -->
      <label class="mode-card {selectedMode === 'off' ? 'active' : ''}">
        <input
          type="radio"
          name="wipe-mode"
          value="off"
          checked={selectedMode === 'off'}
          onchange={() => (selectedMode = 'off')}
        />
        <div class="mode-info">
          <div class="mode-label">🚫 Nonaktif</div>
          <div class="mode-desc">Riwayat obrolan disimpan selamanya tanpa pembersihan otomatis.</div>
        </div>
      </label>

      <!-- Option: ONE-SHOT -->
      <label class="mode-card {selectedMode === 'one_shot' ? 'active' : ''}">
        <input
          type="radio"
          name="wipe-mode"
          value="one_shot"
          checked={selectedMode === 'one_shot'}
          onchange={() => (selectedMode = 'one_shot')}
        />
        <div class="mode-info">
          <div class="mode-label">⏱️ Sekali Eksekusi (One-Shot)</div>
          <div class="mode-desc">Hapus seluruh riwayat pesan sekali saja pada jam target mendatang.</div>
        </div>
      </label>

      <!-- Option: DAILY -->
      <label class="mode-card {selectedMode === 'daily' ? 'active' : ''}">
        <input
          type="radio"
          name="wipe-mode"
          value="daily"
          checked={selectedMode === 'daily'}
          onchange={() => (selectedMode = 'daily')}
        />
        <div class="mode-info">
          <div class="mode-label">🔄 Berulang Setiap Hari (Daily)</div>
          <div class="mode-desc">Rutin bersihkan seluruh obrolan setiap malam pada jam yang ditentukan.</div>
        </div>
      </label>
    </div>

    {#if selectedMode !== 'off'}
      <div class="time-picker-section">
        <span class="time-label">PILIH JAM TARGET EKSEKUSI:</span>
        <div class="time-input-row">
          <input
            type="time"
            class="time-picker-field"
            bind:value={selectedTime}
          />
          <span class="tz-badge">WIB (Lokal)</span>
        </div>
      </div>
    {/if}

    <div class="target-callout-box">
      <div style="font-weight: 700; font-size: 0.8rem; color: var(--text-secondary); margin-bottom: 4px;">
        RENCANA TINDAKAN:
      </div>
      <div style="font-size: 0.85rem; color: var(--text-primary); line-height: 1.45;">
        {targetDescription}
      </div>
    </div>

    {#if isSaved}
      <div class="success-banner">
        ✓ Konfigurasi pembersihan terjadwal berhasil disimpan!
      </div>
    {/if}

    <div class="dialog-actions-row">
      <button type="button" class="btn-cancel" onclick={onclose}>Batal</button>
      <button type="button" class="btn-save" onclick={handleSave}>
        Simpan Pengaturan
      </button>
    </div>
  </div>
</BottomSheet>

<style>
  .sheet-body {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .mode-options-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .mode-card {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px 14px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-light);
    background-color: var(--bg-card);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .mode-card:hover {
    border-color: var(--brand-blue-light);
  }

  .mode-card.active {
    border-color: var(--brand-blue);
    background-color: var(--brand-blue-subtle);
  }

  .mode-card input[type='radio'] {
    margin-top: 3px;
    accent-color: var(--brand-blue);
  }

  .mode-info {
    display: flex;
    flex-direction: column;
  }

  .mode-label {
    font-size: 0.88rem;
    font-weight: 700;
    font-family: var(--font-active);
    color: var(--text-primary);
  }

  .mode-desc {
    font-size: 0.76rem;
    color: var(--text-secondary);
    margin-top: 2px;
    line-height: 1.4;
  }

  .time-picker-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px 14px;
    background-color: var(--bg-primary);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-light);
  }

  .time-label {
    font-size: 0.76rem;
    font-weight: 700;
    color: var(--brand-blue);
    letter-spacing: 0.5px;
  }

  .time-input-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .time-picker-field {
    min-height: var(--touch-target-min, 40px);
    padding: 8px 14px;
    border-radius: var(--radius-pill);
    border: 1px solid var(--border-light);
    font-size: 1.15rem;
    font-weight: 700;
    background: var(--bg-card);
    color: var(--text-primary);
    font-family: inherit;
    outline: none;
  }

  .tz-badge {
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .target-callout-box {
    padding: 12px 14px;
    background-color: var(--bg-primary);
    border-radius: var(--radius-lg);
    border-left: 3px solid var(--brand-blue);
  }

  .success-banner {
    padding: 10px 14px;
    background-color: #ECFDF5;
    border: 1px solid #A7F3D0;
    border-radius: var(--radius-md);
    color: #065F46;
    font-size: 0.82rem;
    font-weight: 700;
  }

  .dialog-actions-row {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 6px;
  }

  .btn-cancel {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 18px;
    border-radius: var(--radius-pill);
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 600;
    font-family: var(--font-active);
    cursor: pointer;
  }

  .btn-save {
    min-height: var(--touch-target-min, 40px);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 22px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-blue);
    border: none;
    color: #FFFFFF;
    font-size: 0.85rem;
    font-weight: 700;
    font-family: var(--font-active);
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(37, 99, 235, 0.25);
  }

  .btn-save:hover {
    background-color: #1D4ED8;
  }
</style>
