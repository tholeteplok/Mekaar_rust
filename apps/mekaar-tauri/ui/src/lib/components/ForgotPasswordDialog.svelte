<script lang="ts">
  import { mekaarStore } from '../stores';
  import Dialog from './Dialog.svelte';
  import Button from './Button.svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open = false, onclose }: Props = $props();

  let input = $state('');
  let otp = $state('');
  let newPassword = $state('');
  let confirmPassword = $state('');
  let isSubmitting = $state(false);
  let resolvedEmail = $state('');
  let showOtpStage = $state(false);
  let statusMessage = $state('');
  let errorMessage = $state('');
  let obscureNewPassword = $state(true);

  function resetForm() {
    input = '';
    otp = '';
    newPassword = '';
    confirmPassword = '';
    isSubmitting = false;
    resolvedEmail = '';
    showOtpStage = false;
    statusMessage = '';
    errorMessage = '';
  }

  async function handleSendResetRequest() {
    errorMessage = '';
    statusMessage = '';
    if (!input.trim()) {
      errorMessage = 'Email atau username tidak boleh kosong.';
      return;
    }

    isSubmitting = true;
    try {
      const res = await mekaarStore.sendPasswordResetEmail(input);
      resolvedEmail = res.email;
      statusMessage = res.message || `Kode OTP dikirim ke ${res.email}`;
      showOtpStage = true;
    } catch (err: any) {
      errorMessage = err?.message || 'Gagal mengirim email reset password.';
    } finally {
      isSubmitting = false;
    }
  }

  async function handleConfirmReset() {
    errorMessage = '';
    statusMessage = '';

    if (!otp.trim()) {
      errorMessage = 'Masukkan kode OTP 6-digit.';
      return;
    }
    if (newPassword.length < 6) {
      errorMessage = 'Password minimal harus 6 karakter.';
      return;
    }
    if (newPassword !== confirmPassword) {
      errorMessage = 'Konfirmasi password baru tidak cocok.';
      return;
    }

    isSubmitting = true;
    try {
      const res = await mekaarStore.confirmPasswordReset({
        email: resolvedEmail || input.trim(),
        token: otp.trim(),
        newPassword
      });

      if (res.success) {
        statusMessage = 'Password berhasil diperbarui! Silakan masuk dengan kata sandi baru Anda.';
        setTimeout(() => {
          resetForm();
          onclose();
        }, 1500);
      } else {
        errorMessage = res.message || 'Verifikasi gagal. Periksa kembali kode OTP Anda.';
      }
    } catch (err: any) {
      errorMessage = err?.message || 'Terjadi kesalahan saat mereset password.';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<Dialog {open} title={showOtpStage ? 'Reset Kata Sandi' : 'Lupa Password?'} onclose={() => { resetForm(); onclose(); }}>
  <div class="recovery-flow">
    {#if !showOtpStage}
      <p class="desc-text">
        Masukkan alamat email terdaftar atau username Anda. Kami akan mengirimkan instruksi dan kode OTP 6-digit untuk pemulihan akun.
      </p>

      <div class="field-block">
        <label for="recovery-input" class="field-label">Email atau Username</label>
        <input
          id="recovery-input"
          type="text"
          class="field-input"
          placeholder="nama@email.com atau @username"
          bind:value={input}
          disabled={isSubmitting}
        />
      </div>

      {#if errorMessage}
        <div class="banner error-banner">⚠️ {errorMessage}</div>
      {/if}

      <div class="action-row">
        <Button
          variant="primary"
          fullWidth
          disabled={isSubmitting}
          onclick={handleSendResetRequest}
        >
          {isSubmitting ? 'Mengirim...' : 'Kirim Kode Pemulihan ➔'}
        </Button>
      </div>

      <div class="otp-switch">
        <button
          type="button"
          class="link-btn"
          onclick={() => { showOtpStage = true; }}
        >
          Sudah punya kode OTP 6-digit? Masukkan langsung
        </button>
      </div>
    {:else}
      <p class="desc-text">
        Masukkan kode OTP 6-digit yang diterima di <strong>{resolvedEmail || 'email Anda'}</strong> beserta kata sandi baru.
      </p>

      <div class="field-block">
        <label for="recovery-otp" class="field-label">Kode OTP 6-Digit</label>
        <input
          id="recovery-otp"
          type="text"
          maxlength={6}
          class="field-input otp-input"
          placeholder="000000"
          bind:value={otp}
          disabled={isSubmitting}
        />
      </div>

      <div class="field-block">
        <label for="recovery-new-pwd" class="field-label">Password Baru</label>
        <div class="pwd-wrapper">
          <input
            id="recovery-new-pwd"
            type={obscureNewPassword ? 'password' : 'text'}
            class="field-input"
            placeholder="Minimal 6 karakter"
            bind:value={newPassword}
            disabled={isSubmitting}
          />
          <button
            type="button"
            class="eye-btn"
            onclick={() => (obscureNewPassword = !obscureNewPassword)}
            aria-label="Toggle password visibility"
          >
            {obscureNewPassword ? '👁️' : '🙈'}
          </button>
        </div>
      </div>

      <div class="field-block">
        <label for="recovery-confirm-pwd" class="field-label">Konfirmasi Password Baru</label>
        <input
          id="recovery-confirm-pwd"
          type="password"
          class="field-input"
          placeholder="Ulangi password baru"
          bind:value={confirmPassword}
          disabled={isSubmitting}
        />
      </div>

      {#if statusMessage}
        <div class="banner success-banner">✓ {statusMessage}</div>
      {/if}

      {#if errorMessage}
        <div class="banner error-banner">⚠️ {errorMessage}</div>
      {/if}

      <div class="action-row">
        <Button
          variant="primary"
          fullWidth
          disabled={isSubmitting}
          onclick={handleConfirmReset}
        >
          {isSubmitting ? 'Memproses...' : 'Perbarui Password Sekarang'}
        </Button>
      </div>

      <div class="otp-switch">
        <button
          type="button"
          class="link-btn"
          onclick={() => { showOtpStage = false; }}
        >
          ← Kembali ke kirim email
        </button>
      </div>
    {/if}
  </div>
</Dialog>

<style>
  .recovery-flow {
    display: flex;
    flex-direction: column;
    gap: 16px;
    user-select: none;
  }

  .desc-text {
    font-size: 0.88rem;
    color: var(--text-secondary);
    line-height: 1.45;
    margin: 0;
  }

  .field-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label {
    font-size: 0.82rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .field-input {
    width: 100%;
    height: 44px;
    padding: 0 14px;
    border-radius: 12px;
    border: 1px solid var(--border-subtle);
    background-color: var(--bg-secondary);
    font-size: 0.92rem;
    color: var(--text-primary);
    box-sizing: border-box;
    outline: none;
    transition: border-color 0.15s ease, background-color 0.15s ease;
  }

  .field-input:focus {
    border-color: var(--brand-blue);
    background-color: var(--bg-surface);
  }

  .otp-input {
    text-align: center;
    font-family: var(--font-mono);
    font-size: 1.3rem;
    letter-spacing: 6px;
    font-weight: 700;
  }

  .pwd-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .eye-btn {
    position: absolute;
    right: 12px;
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 1rem;
    color: var(--text-secondary);
  }

  .banner {
    padding: 10px 12px;
    border-radius: 10px;
    font-size: 0.84rem;
    font-weight: 600;
    line-height: 1.35;
  }

  .error-banner {
    background-color: #FFF1F2;
    border: 1px solid #FECDD3;
    color: #BE123C;
  }

  .success-banner {
    background-color: #ECFDF5;
    border: 1px solid #A7F3D0;
    color: #065F46;
  }

  .action-row {
    margin-top: 4px;
  }

  .otp-switch {
    display: flex;
    justify-content: center;
    margin-top: 4px;
  }

  .link-btn {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 6px;
  }

  .link-btn:hover {
    text-decoration: underline;
  }
</style>
