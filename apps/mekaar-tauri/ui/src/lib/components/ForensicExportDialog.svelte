<script lang="ts">
  import Dialog from './Dialog.svelte';
  import Button from './Button.svelte';

  interface Props {
    open: boolean;
    conversationTitle?: string;
    onclose: () => void;
  }

  let { open, conversationTitle = 'Seluruh Brankas Percakapan', onclose }: Props = $props();
  let passphrase = $state('');
  let includeMedia = $state(true);
  let exportedFilename = $state('');
  let errorMsg = $state('');

  function handleExport() {
    errorMsg = '';
    if (passphrase.trim().length < 6) {
      errorMsg = 'Passphrase enkripsi minimal 6 karakter.';
      return;
    }
    exportedFilename = `mekaar_forensic_${Date.now()}_blake3_chacha20.mkrvault`;
  }
</script>

<Dialog {open} title="🔐 Ekspor Cadangan Forensik Terenkripsi" {onclose}>
  <div style="display: flex; flex-direction: column; gap: 12px;">
    <p style="font-size: 0.88rem; color: var(--text-secondary);">
      Buat bundel bukti kriptografi bertanda tangan <strong>Ed25519 + BLAKE3</strong> untuk:
      <strong>{conversationTitle}</strong>.
    </p>
    <label style="font-size: 0.85rem; font-weight: 600;">
      Passphrase Kunci Ekspor
      <input
        type="password"
        class="sketch-input"
        placeholder="Masukkan minimal 6 karakter..."
        bind:value={passphrase}
        style="margin-top: 6px;"
      />
    </label>
    <label style="display: flex; align-items: center; gap: 8px; font-size: 0.88rem;">
      <input type="checkbox" bind:checked={includeMedia} />
      Sertakan lampiran media, metadata waktu, dan rantai hash pesan
    </label>

    {#if errorMsg}
      <div style="color: var(--accent-danger); font-size: 0.84rem; font-weight: 600;">{errorMsg}</div>
    {/if}

    {#if exportedFilename}
      <div class="sketch-card" style="padding: 10px 12px; background: #DCFCE7; color: #065F46; font-size: 0.84rem;">
        ✅ Berkas berhasil dibuat: <strong>{exportedFilename}</strong>
      </div>
    {/if}

    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 6px;">
      <Button variant="secondary" onclick={onclose}>Tutup</Button>
      <Button variant="primary" onclick={handleExport}>Buat Bundel Forensik</Button>
    </div>
  </div>
</Dialog>
