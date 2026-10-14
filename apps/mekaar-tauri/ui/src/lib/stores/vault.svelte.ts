import type { VaultSession } from '../types';

class VaultStore {
  isLocked = $state<boolean>(true);
  session = $state<VaultSession | null>(null);
  pinInput = $state<string>('');
  errorMessage = $state<string | null>(null);
  failedAttempts = $state<number>(0);
  isSubmitting = $state<boolean>(false);

  get isDuress(): boolean {
    return this.session?.isDuress ?? false;
  }

  get identityFingerprint(): string {
    return this.session?.identityFingerprint ?? 'MKR-UNREGISTERED';
  }

  async enterDigit(d: string) {
    if (this.pinInput.length < 6) {
      this.pinInput += d;
      this.errorMessage = null;

      // Auto submit saat 6 digit lengkap, atau saat 4 digit quick/decoy PIN
      if (this.pinInput.length === 6 || (this.pinInput.length === 4 && (this.pinInput === '1234' || this.pinInput === '9999'))) {
        await this.submitPin();
      }
    }
  }

  deleteDigit() {
    if (this.pinInput.length > 0) {
      this.pinInput = this.pinInput.slice(0, -1);
      this.errorMessage = null;
    }
  }

  clearPin() {
    this.pinInput = '';
    this.errorMessage = null;
  }

  async submitPin(): Promise<boolean> {
    if (this.pinInput.length < 4) {
      this.errorMessage = 'PIN minimal 4 digit';
      return false;
    }

    this.isSubmitting = true;
    this.errorMessage = null;
    const pin = this.pinInput;

    try {
      let result: VaultSession;

      // Try invoking Tauri backend command
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const raw = await invoke<{
          unlocked: boolean;
          is_duress: boolean;
          identity_fingerprint: string;
          session_token: string;
        }>('unlock_vault', { pin });

        result = {
          unlocked: raw.unlocked,
          isDuress: raw.is_duress,
          identityFingerprint: raw.identity_fingerprint,
          sessionToken: raw.session_token,
        };
      } catch (invokeErr: any) {
        const isTauri = typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
        if (isTauri && invokeErr?.message && !invokeErr.message.includes('not found')) {
          throw invokeErr;
        }

        // Web fallback simulation
        if (pin === '9999') {
          result = {
            unlocked: true,
            isDuress: true,
            identityFingerprint: 'MKR-DU-8841-B720-DCOY',
            sessionToken: 'tok_decoy_web_9999',
          };
        } else if (pin.length >= 4) {
          result = {
            unlocked: true,
            isDuress: false,
            identityFingerprint: 'MKR-7A-3F91-C82E-8902',
            sessionToken: 'tok_secure_web_active',
          };
        } else {
          throw new Error('PIN minimal 4 digit');
        }
      }

      this.session = result;
      this.isLocked = false;
      this.pinInput = '';
      this.failedAttempts = 0;
      this.isSubmitting = false;
      return true;
    } catch (err: unknown) {
      this.failedAttempts += 1;
      this.errorMessage = err instanceof Error ? err.message : 'PIN salah atau tidak valid';
      this.pinInput = '';
      this.isSubmitting = false;
      return false;
    }
  }

  lock() {
    this.isLocked = true;
    this.session = null;
    this.pinInput = '';
    this.errorMessage = null;
  }
}

export const vaultStore = new VaultStore();
