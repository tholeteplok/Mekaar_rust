<script lang="ts">
  import { mekaarStore } from '../stores';
  import BrandIcon from '../components/BrandIcon.svelte';
  import Button from '../components/Button.svelte';
  import ForgotPasswordDialog from '../components/ForgotPasswordDialog.svelte';

  let isLogin = $state(true);

  // Form states
  let identifier = $state(''); // email or username
  let password = $state('');
  let displayName = $state('');
  let username = $state('');
  let email = $state('');
  let confirmPassword = $state('');

  let obscurePassword = $state(true);
  let isLoading = $state(false);
  let errorMessage = $state('');
  let showForgotModal = $state(false);

  const cleanUsername = $derived(
    username.replace(/^@/, '').trim().toLowerCase()
  );

  async function handleSubmit() {
    errorMessage = '';

    if (isLogin) {
      // ── Validasi Login ──
      if (!identifier.trim()) {
        errorMessage = 'Masukkan email atau username Anda.';
        return;
      }
      if (!password) {
        errorMessage = 'Masukkan password Anda.';
        return;
      }

      isLoading = true;
      try {
        const res = await mekaarStore.loginUser({
          identifier: identifier.trim(),
          password
        });

        if (res.success) {
          // Lanjut ke pembuatan atau buka kunci PIN
          const storedPin = localStorage.getItem('mekaar_pin');
          if (storedPin) {
            mekaarStore.advanceFtue('pin_locked');
          } else {
            mekaarStore.advanceFtue('pin_setup');
          }
        } else {
          errorMessage = res.message || 'Login gagal. Periksa kembali kredensial Anda.';
        }
      } catch (err: any) {
        errorMessage = err?.message || 'Terjadi kesalahan sistem saat masuk.';
      } finally {
        isLoading = false;
      }
    } else {
      // ── Validasi Registrasi ──
      if (!displayName.trim()) {
        errorMessage = 'Nama tampilan tidak boleh kosong.';
        return;
      }
      if (cleanUsername.length < 3) {
        errorMessage = 'Username minimal 3 karakter alfanumerik.';
        return;
      }
      if (!/^[a-z0-9_]+$/.test(cleanUsername)) {
        errorMessage = 'Username hanya boleh berisi huruf kecil, angka, dan garis bawah (_).';
        return;
      }
      if (!email.trim() || !email.includes('@')) {
        errorMessage = 'Alamat email pemulihan tidak valid.';
        return;
      }
      if (password.length < 6) {
        errorMessage = 'Password minimal harus 6 karakter.';
        return;
      }
      if (password !== confirmPassword) {
        errorMessage = 'Konfirmasi password tidak cocok.';
        return;
      }

      isLoading = true;
      try {
        const res = await mekaarStore.registerUser({
          displayName: displayName.trim(),
          username: cleanUsername,
          email: email.trim(),
          password
        });

        if (res.success) {
          // Setelah akun dan identitas kriptografi terdaftar, buat PIN keamanan lokal
          mekaarStore.advanceFtue('pin_setup');
        } else {
          errorMessage = res.message || 'Gagal mendaftarkan akun baru.';
        }
      } catch (err: any) {
        errorMessage = err?.message || 'Terjadi kesalahan saat pendaftaran.';
      } finally {
        isLoading = false;
      }
    }
  }
</script>

<div class="auth-viewport">
  <div class="auth-card clean-card with-ticks">
    <!-- Header Brand Section (Mekaar_chat Parity) -->
    <div class="brand-header">
      <BrandIcon size={38} variant="raw" alt="Logo MEKAAR" />
      <span class="wordmark-label">
        <span class="wordmark-mek">Mek</span><span class="wordmark-aar">aar</span>
      </span>
    </div>

    <div class="heading-block">
      <h1 class="main-title">
        {isLogin ? 'Selamat Datang\nKembali' : 'Buat Akun\nBaru Anda'}
      </h1>
      <p class="subtitle">
        {isLogin
          ? 'Masukkan email atau username dan password untuk melanjutkan.'
          : 'Mulai dengan membuat profil chat terenkripsi dan amankan identitas Anda.'}
      </p>
    </div>

    <!-- Toggle Segment: Masuk / Daftar -->
    <div class="segment-bar">
      <button
        type="button"
        class="segment-item {isLogin ? 'active' : ''}"
        onclick={() => { isLogin = true; errorMessage = ''; }}
      >
        Masuk
      </button>
      <button
        type="button"
        class="segment-item {!isLogin ? 'active' : ''}"
        onclick={() => { isLogin = false; errorMessage = ''; }}
      >
        Daftar Baru
      </button>
    </div>

    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="form-body">
      {#if isLogin}
        <!-- Login Form Fields -->
        <div class="input-block">
          <label for="login-identifier" class="input-label">Email atau Username</label>
          <div class="input-field-wrap">
            <span class="icon-adornment">👤</span>
            <input
              id="login-identifier"
              type="text"
              class="text-input"
              placeholder="nama@email.com atau @username"
              bind:value={identifier}
              disabled={isLoading}
            />
          </div>
        </div>

        <div class="input-block">
          <div class="label-row">
            <label for="login-password" class="input-label">Password</label>
            <button
              type="button"
              class="forgot-link"
              onclick={() => (showForgotModal = true)}
            >
              Lupa Password?
            </button>
          </div>
          <div class="input-field-wrap">
            <span class="icon-adornment">🔒</span>
            <input
              id="login-password"
              type={obscurePassword ? 'password' : 'text'}
              class="text-input"
              placeholder="Masukkan password"
              bind:value={password}
              disabled={isLoading}
            />
            <button
              type="button"
              class="eye-btn"
              onclick={() => (obscurePassword = !obscurePassword)}
              aria-label="Toggle password"
            >
              {obscurePassword ? '👁️' : '🙈'}
            </button>
          </div>
        </div>
      {:else}
        <!-- Register Form Fields -->
        <div class="input-block">
          <label for="reg-name" class="input-label">Nama Tampilan</label>
          <input
            id="reg-name"
            type="text"
            class="text-input plain-input"
            placeholder="Contoh: Budi Santoso"
            bind:value={displayName}
            disabled={isLoading}
          />
        </div>

        <div class="input-block">
          <label for="reg-username" class="input-label">Username Unik</label>
          <div class="input-field-wrap">
            <span class="prefix-tag">@</span>
            <input
              id="reg-username"
              type="text"
              class="text-input with-prefix"
              placeholder="username"
              bind:value={username}
              disabled={isLoading}
            />
          </div>
          <span class="field-hint">Digunakan rekan kontak untuk menemukan Anda.</span>
        </div>

        <div class="input-block">
          <label for="reg-email" class="input-label">Email Pemulihan Akun</label>
          <div class="input-field-wrap">
            <span class="icon-adornment">✉️</span>
            <input
              id="reg-email"
              type="email"
              class="text-input"
              placeholder="nama@email.com"
              bind:value={email}
              disabled={isLoading}
            />
          </div>
          <span class="field-hint">Penting untuk memulihkan akses saat lupa kata sandi.</span>
        </div>

        <div class="input-block">
          <label for="reg-password" class="input-label">Password</label>
          <div class="input-field-wrap">
            <span class="icon-adornment">🔒</span>
            <input
              id="reg-password"
              type={obscurePassword ? 'password' : 'text'}
              class="text-input"
              placeholder="Minimal 6 karakter"
              bind:value={password}
              disabled={isLoading}
            />
            <button
              type="button"
              class="eye-btn"
              onclick={() => (obscurePassword = !obscurePassword)}
              aria-label="Toggle password"
            >
              {obscurePassword ? '👁️' : '🙈'}
            </button>
          </div>
        </div>

        <div class="input-block">
          <label for="reg-confirm-password" class="input-label">Konfirmasi Password</label>
          <div class="input-field-wrap">
            <span class="icon-adornment">🛡️</span>
            <input
              id="reg-confirm-password"
              type="password"
              class="text-input"
              placeholder="Ulangi password"
              bind:value={confirmPassword}
              disabled={isLoading}
            />
          </div>
        </div>
      {/if}

      {#if errorMessage}
        <div class="error-banner">⚠️ {errorMessage}</div>
      {/if}

      <div class="submit-action-wrap">
        <Button
          variant="primary"
          fullWidth
          size="lg"
          disabled={isLoading}
          onclick={handleSubmit}
        >
          {isLoading
            ? 'Memproses...'
            : isLogin
              ? 'Masuk ke Aplikasi ➔'
              : 'Daftar Sekarang ➔'}
        </Button>
      </div>
    </form>

    <!-- Footer switcher -->
    <div class="auth-footer">
      <span class="footer-prompt">
        {isLogin ? 'Belum punya akun?' : 'Sudah punya akun?'}
      </span>
      <button
        type="button"
        class="switch-link-btn"
        onclick={() => { isLogin = !isLogin; errorMessage = ''; }}
      >
        {isLogin ? 'Daftar Sekarang' : 'Masuk di sini'}
      </button>
    </div>
  </div>
</div>

<ForgotPasswordDialog
  open={showForgotModal}
  onclose={() => (showForgotModal = false)}
/>

<style>
  .auth-viewport {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: calc(24px + env(safe-area-inset-top, 0px)) 16px calc(24px + env(safe-area-inset-bottom, 0px));
    background-color: var(--bg-primary);
    box-sizing: border-box;
    user-select: none;
    -webkit-user-select: none;
  }

  .auth-card {
    width: min(440px, 100%);
    padding: 32px 28px;
    background: var(--bg-surface);
    border-radius: var(--radius-card);
    border: 0.75px solid var(--border-subtle);
    box-shadow: var(--shadow-card);
    display: flex;
    flex-direction: column;
    gap: 20px;
    box-sizing: border-box;
  }

  .brand-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .wordmark-label {
    font-size: 1.55rem;
    font-weight: 800;
    letter-spacing: -0.4px;
    font-family: var(--font-sans);
  }

  .wordmark-mek {
    color: #F59E0B;
  }

  .wordmark-aar {
    color: #0EA5E9;
  }

  .heading-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .main-title {
    font-size: 1.7rem;
    font-weight: 800;
    color: var(--text-primary);
    line-height: 1.25;
    letter-spacing: -0.3px;
    white-space: pre-line;
    margin: 0;
  }

  .subtitle {
    font-size: 0.9rem;
    color: var(--text-secondary);
    line-height: 1.45;
    margin: 0;
  }

  .form-body {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .input-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .input-label {
    font-size: 0.84rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .forgot-link {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
    padding: 0;
  }

  .forgot-link:hover {
    text-decoration: underline;
  }

  .input-field-wrap {
    position: relative;
    display: flex;
    align-items: center;
  }

  .icon-adornment {
    position: absolute;
    left: 14px;
    font-size: 0.95rem;
    pointer-events: none;
    color: var(--text-secondary);
  }

  .prefix-tag {
    position: absolute;
    left: 14px;
    font-weight: 700;
    color: var(--brand-blue);
    pointer-events: none;
  }

  .text-input {
    width: 100%;
    height: 48px;
    padding: 0 42px 0 42px;
    border-radius: 14px;
    border: 1px solid var(--border-subtle);
    background-color: var(--bg-secondary);
    font-size: 0.94rem;
    color: var(--text-primary);
    box-sizing: border-box;
    outline: none;
    transition: border-color 0.15s ease, background-color 0.15s ease;
  }

  .text-input.plain-input {
    padding-left: 14px;
    padding-right: 14px;
  }

  .text-input.with-prefix {
    padding-left: 32px;
  }

  .text-input:focus {
    border-color: var(--brand-blue);
    background-color: var(--bg-surface);
  }

  .eye-btn {
    position: absolute;
    right: 12px;
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 1rem;
    color: var(--text-secondary);
    padding: 4px;
  }

  .field-hint {
    font-size: 0.74rem;
    color: var(--text-secondary);
    line-height: 1.3;
  }

  .error-banner {
    padding: 10px 12px;
    border-radius: 10px;
    background-color: #FFF1F2;
    border: 1px solid #FECDD3;
    color: #BE123C;
    font-size: 0.84rem;
    font-weight: 600;
  }

  .submit-action-wrap {
    margin-top: 6px;
  }

  .auth-footer {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    margin-top: 4px;
  }

  .footer-prompt {
    font-size: 0.88rem;
    color: var(--text-secondary);
  }

  .switch-link-btn {
    background: transparent;
    border: none;
    color: var(--brand-blue);
    font-size: 0.88rem;
    font-weight: 700;
    cursor: pointer;
    padding: 0;
  }

  .switch-link-btn:hover {
    text-decoration: underline;
  }
</style>
