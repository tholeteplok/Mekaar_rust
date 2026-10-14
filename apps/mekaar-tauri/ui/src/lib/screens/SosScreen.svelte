<script lang="ts">
  import { onMount } from 'svelte';
  import { sosStore } from '../stores';
  import { AppBar, Card, CardExpanded, Button, HoldToConfirmButton, ForensicExportDialog } from '../components';

  interface Props {
    onback: () => void;
  }

  let { onback }: Props = $props();

  let showForensicDialog = $state(false);

  onMount(() => {
    sosStore.loadGuardians();
  });

  async function handlePanicConfirm() {
    await sosStore.triggerPanic();
  }

  function handleCancelEmergency() {
    sosStore.resolveEmergency();
  }
</script>

<div class="sos-screen" data-register="sos">
  <AppBar
    title="FOCUSED SOS"
    subtitle="Pusat Sinyal Darurat & Jaringan Guardian"
    showBack
    register="sos"
    {onback}
  />

  <div class="sos-content">
    <!-- Active Emergency Alert Box -->
    {#if sosStore.isEmergencyActive}
      <div class="emergency-active-banner">
        <div class="banner-icon">🚨</div>
        <div class="banner-text">
          <h3>SINYAL DARURAT AKTIF</h3>
          <p>
            Peringatan telah disiarkan ke {sosStore.lastAlert?.dispatchedTo.length ?? 0} Guardian terdaftar bersama stempel waktu forensik.
          </p>
        </div>
        <Button variant="outline" size="sm" onclick={handleCancelEmergency}>
          Matikan Sinyal
        </Button>
      </div>
    {/if}

    <!-- Panic Button Hero Section -->
    <section class="panic-hero-card">
      <h2 class="hero-title">Tombol Panik Darurat</h2>
      <p class="hero-desc">
        Tekan dan tahan tombol di bawah ini selama 2 detik untuk mengirimkan sinyal bahaya instan dan mengunci ruang rahasia.
      </p>

      <div class="panic-trigger-wrapper">
        <HoldToConfirmButton
          durationMs={2000}
          size={84}
          label="SOS"
          sublabel="Tahan 2 Detik"
          onconfirm={handlePanicConfirm}
        />
      </div>
    </section>

    <!-- Trusted Guardians Section -->
    <section class="guardians-section">
      <div class="section-header-row">
        <h3 class="section-title">Guardian Terpercaya</h3>
        <span class="active-count">{sosStore.guardians.length} Terdaftar</span>
      </div>

      <div class="guardians-list">
        {#each sosStore.guardians as guard (guard.id)}
          <Card variant="default" padding="md">
            <div class="guardian-row">
              <div class="guard-avatar">
                🛡️
              </div>
              <div class="guard-info">
                <div class="guard-name-row">
                  <span class="guard-name">{guard.name}</span>
                  <span class="role-badge">{guard.role}</span>
                </div>
                <span class="guard-key">{guard.publicKey}</span>
              </div>
              <div class="guard-status">
                <span class="status-indicator" class:online={guard.isOnline}></span>
                <span class="status-text">{guard.isOnline ? 'Online' : 'Offline'}</span>
              </div>
            </div>
          </Card>
        {/each}
      </div>
    </section>

    <!-- Forensic Watchdog Details Accordion -->
    <section class="forensic-section">
      <CardExpanded
        title="Protokol Bukti Forensik & Watchdog"
        subtitle="Integritas Rantai Hash SHA-256 • Dead Man's Switch"
        badge="Aktif"
        badgeVariant="coral"
      >
        <div class="forensic-details">
          <p>
            <strong>Status Watchdog:</strong> Heartbeat aktif setiap 30 detik ke server relay.
          </p>
          <p>
            <strong>Protokol Duress:</strong> Jika dipaksa membuka kunci, gunakan PIN alternatif (9999) untuk membuka brankas umpan tanpa membahayakan data rahasia Anda.
          </p>
          <p>
            <strong>Stempel Forensik Terakhir:</strong>
            <code>{sosStore.lastAlert ? `ALERT-ID: ${sosStore.lastAlert.alertId}` : 'IDLE - Tidak ada sinyal darurat tertunda'}</code>
          </p>

          <div style="margin-top: var(--space-3);">
            <Button variant="secondary" size="sm" onclick={() => showForensicDialog = true}>
              📋 Ekspor Bukti Forensik Kriptografis
            </Button>
          </div>
        </div>
      </CardExpanded>
    </section>
  </div>

  <ForensicExportDialog bind:open={showForensicDialog} />
</div>

<style>
  .sos-screen {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background-color: var(--surface-canvas);
    color: var(--text-primary);
  }

  .sos-content {
    max-width: 800px;
    width: 100%;
    margin: 0 auto;
    padding: var(--space-4) var(--space-4) var(--space-12);
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .emergency-active-banner {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    background-color: rgba(var(--sos-coral-rgb), 0.2);
    border: 1.5px solid var(--sos-coral);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    animation: bannerPulse 1s infinite alternate;
  }

  .banner-icon {
    font-size: 32px;
  }

  .banner-text {
    flex: 1;
  }

  .banner-text h3 {
    font-size: var(--font-h2-size);
    font-weight: 800;
    color: var(--sos-coral);
    margin-bottom: 2px;
  }

  .banner-text p {
    font-size: var(--font-label-size);
    color: var(--text-primary);
  }

  .panic-hero-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-8) var(--space-4);
    background-color: var(--surface-card);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-subtle);
    box-shadow: var(--shadow-elevated);
    gap: var(--space-2);
  }

  .hero-title {
    font-size: var(--font-h1-size);
    font-weight: 800;
    color: var(--text-primary);
  }

  .hero-desc {
    max-width: 500px;
    font-size: var(--font-body-size);
    color: var(--text-secondary);
    line-height: var(--font-body-line);
  }

  .panic-trigger-wrapper {
    margin-top: var(--space-4);
  }

  .guardians-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .section-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .section-title {
    font-size: var(--font-h2-size);
    font-weight: 700;
    color: var(--text-primary);
  }

  .active-count {
    font-size: var(--font-caption-size);
    font-weight: 600;
    color: var(--text-secondary);
  }

  .guardians-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .guardian-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .guard-avatar {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-pill);
    background-color: var(--surface-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    flex-shrink: 0;
  }

  .guard-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .guard-name-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .guard-name {
    font-size: var(--font-body-size);
    font-weight: 700;
    color: var(--text-primary);
  }

  .role-badge {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: var(--radius-pill);
    background-color: var(--brand-light-blue);
    color: var(--brand-blue);
    text-transform: uppercase;
  }

  .guard-key {
    font-size: var(--font-caption-size);
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .guard-status {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .status-indicator {
    width: 10px;
    height: 10px;
    border-radius: var(--radius-pill);
    background-color: var(--text-secondary);
  }

  .status-indicator.online {
    background-color: var(--safe-teal);
    box-shadow: 0 0 8px rgba(var(--safe-teal-rgb), 0.6);
  }

  .status-text {
    font-size: var(--font-caption-size);
    font-weight: 600;
    color: var(--text-secondary);
  }

  .forensic-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    font-size: var(--font-label-size);
    line-height: var(--font-label-line);
  }

  .forensic-details code {
    background-color: var(--surface-subtle);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 11px;
    word-break: break-all;
  }

  @keyframes bannerPulse {
    0% {
      box-shadow: 0 0 0 0 rgba(var(--sos-coral-rgb), 0.4);
    }
    100% {
      box-shadow: 0 0 0 12px rgba(var(--sos-coral-rgb), 0);
    }
  }
</style>
