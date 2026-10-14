import type { Guardian, SosAlert } from '../types';

class SosStore {
  isEmergencyActive = $state<boolean>(false);
  guardians = $state<Guardian[]>([]);
  lastAlert = $state<SosAlert | null>(null);
  watchdogActive = $state<boolean>(true);
  isLoading = $state<boolean>(false);

  async loadGuardians() {
    this.isLoading = true;
    try {
      const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
      if (isTauri) {
        const { invoke } = await import('@tauri-apps/api/core');
        const raw = await invoke<Array<{
          id: string;
          name: string;
          public_key: string;
          is_online: boolean;
          role: string;
        }>>('get_guardians');

        this.guardians = raw.map((g) => ({
          id: g.id,
          name: g.name,
          publicKey: g.public_key,
          isOnline: g.is_online,
          role: g.role,
        }));
      } else {
        // Fallback mockup
        this.guardians = [
          {
            id: 'guard_budi',
            name: 'Budi Santoso (Guardian Utama)',
            publicKey: 'x25519_pk_budi_92a83f...',
            isOnline: true,
            role: 'Primary',
          },
          {
            id: 'guard_citra',
            name: 'Citra Lestari (Guardian Sekunder)',
            publicKey: 'x25519_pk_citra_47e11c...',
            isOnline: true,
            role: 'Secondary',
          },
        ];
      }
    } finally {
      this.isLoading = false;
    }
  }

  async triggerPanic(guardianId?: string): Promise<boolean> {
    const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
    let alert: SosAlert;

    if (isTauri) {
      const { invoke } = await import('@tauri-apps/api/core');
      const raw = await invoke<{
        alert_id: string;
        status: string;
        timestamp: number;
        dispatched_to: string[];
      }>('trigger_sos_alert', { guardianId });

      alert = {
        alertId: raw.alert_id,
        status: raw.status,
        timestamp: raw.timestamp,
        dispatchedTo: raw.dispatched_to,
      };
    } else {
      alert = {
        alertId: `sos_${Date.now()}`,
        status: 'BROADCAST_ACTIVE',
        timestamp: Date.now(),
        dispatchedTo: guardianId ? [guardianId] : ['guard_budi', 'guard_citra'],
      };
    }

    this.lastAlert = alert;
    this.isEmergencyActive = true;
    return true;
  }

  resolveEmergency() {
    this.isEmergencyActive = false;
  }
}

export const sosStore = new SosStore();
