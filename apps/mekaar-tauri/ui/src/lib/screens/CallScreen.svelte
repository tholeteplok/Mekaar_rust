<script lang="ts">
  import { mekaarStore } from '../stores';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
  import CallFloatingPip from '../components/CallFloatingPip.svelte';

  function formatDuration(sec: number): string {
    const m = Math.floor(sec / 60);
    const s = sec % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }
</script>

{#if mekaarStore.activeCall}
  {@const call = mekaarStore.activeCall}

  {#if call.isMinimized}
    <!-- Floating Picture-in-Picture Widget -->
    <CallFloatingPip />
  {:else}
    <!-- Full Call Overlay Screen -->
    <div class="call-overlay">
      <div class="call-card clean-card">
        <!-- Header Bar: Minimize Button & Call Type -->
        <div class="call-card-top-bar">
          <button
            type="button"
            class="minimize-header-btn"
            onclick={() => mekaarStore.toggleCallMinimize()}
            title="Ciutkan Layar Panggilan"
          >
            ▼ Perkecil (PiP)
          </button>
          <span class="status-badge {call.state}">
            {call.state === 'dialing'
              ? 'Memanggil...'
              : call.state === 'ringing'
              ? 'Panggilan Masuk...'
              : `🟢 Terkoneksi (${formatDuration(call.durationSeconds)})`}
          </span>
        </div>

        <!-- Real-Time Network & Security Telemetry Strip -->
        {#if call.state === 'connected'}
          <div class="network-telemetry-strip">
            <span class="telemetry-pill" title="Kualitas Jaringan">
              📶 Sinyal: <strong>{call.networkQuality === 'excellent' ? 'Sangat Baik' : 'Stabil'}</strong>
            </span>
            <span class="telemetry-pill" title="Latensi Sambungan Langsung">
              ⚡ Latensi: <strong>{call.latencyMs ?? 38}ms</strong>
            </span>
            <span class="telemetry-pill" title="Kejernihan Audio/Video">
              🎵 Bitrate: <strong>{call.bitrateKbps ?? 128} kbps</strong>
            </span>
            <span class="telemetry-pill srtp-tag" title="Enkripsi Panggilan Langsung">
              🔒 E2EE Call
            </span>
          </div>
        {/if}

        <!-- Avatar & Nama Lawan Bicara -->
        <div class="avatar-center-wrap">
          <SketchAvatar name={call.peerName} color={call.avatarColor} size={84} online={true} />
        </div>

        <h2 class="call-peer-title">{call.peerName}</h2>
        <p class="call-status-sub">
          {#if call.state === 'ringing'}
            🔔 Panggilan {call.isVideo ? 'Video' : 'Suara'} Masuk...
          {:else if call.state === 'dialing'}
            📡 Menghubungkan Sambungan Aman...
          {:else}
            🔒 Panggilan Terenkripsi E2EE • Sejak {call.startedAt}
          {/if}
        </p>

        <!-- SAS (Short Authentication String) Verification Box Anti-MITM -->
        <div class="sas-verify-box">
          <div class="sas-header-row">
            <span class="sas-title-label">
              KODE ANTI-SADAP (SAS)
            </span>
            <span class="sas-emojis-tag">{call.sasEmojis ?? '🛡️ ⚡'}</span>
          </div>
          <div class="sas-code-display">
            {call.encryptionSAS}
          </div>
          <div class="sas-explanation-text">
            Cocokkan kode 4 digit & emoji ini dengan lawan bicara untuk memastikan tidak ada pihak ketiga yang menyadap panggilan.
          </div>
        </div>

        <!-- Video Stage HD (Jika Mode Video Aktif) -->
        {#if call.isVideo && call.state === 'connected'}
          <div class="video-stage">
            {#if call.mutedVideo}
              <div class="video-placeholder-text">🎥 Kamera Ditutup</div>
            {:else}
              <div class="video-active-stream">
                <span class="stream-badge">🎥 720p HD P2P ({call.peerName})</span>
                <span class="stream-cam-facing">
                  {call.cameraFacing === 'environment' ? '📷 Kamera Belakang' : '🤳 Kamera Depan'}
                </span>
              </div>
            {/if}
            <div class="pip-self">
              {call.mutedVideo ? 'Kamera Mati' : 'Kamera Anda'}
            </div>
          </div>
        {/if}

        <!-- Tombol Aksi Kontrol Panggilan -->
        <div class="call-actions-row">
          {#if call.state === 'ringing'}
            <Button variant="primary" onclick={() => mekaarStore.acceptIncomingCall()}>
              📞 Terima Panggilan
            </Button>
            <Button variant="danger" onclick={() => mekaarStore.rejectIncomingCall()}>
              ✕ Tolak
            </Button>
          {:else}
            <!-- Toggle Mute Audio -->
            <Button
              variant={call.mutedAudio ? 'danger' : 'secondary'}
              onclick={() => mekaarStore.toggleCallMute()}
            >
              {call.mutedAudio ? '🔇 Buka Mic' : '🎙️ Mic Aktif'}
            </Button>

            <!-- Video Controls: Toggle / Upgrade to Video -->
            {#if call.isVideo}
              <Button
                variant={call.mutedVideo ? 'secondary' : 'primary'}
                onclick={() => mekaarStore.toggleCallVideo()}
              >
                {call.mutedVideo ? '📷 Buka Kamera' : '🎥 Tutup Kamera'}
              </Button>
              <Button
                variant="secondary"
                onclick={() => mekaarStore.flipCamera()}
                title="Beralih antara Kamera Depan & Belakang"
              >
                🔄 Balik Kamera
              </Button>
            {:else}
              <Button
                variant="secondary"
                onclick={() => mekaarStore.upgradeToVideoCall()}
                title="Tingkatkan panggilan suara menjadi panggilan video"
              >
                🎥 Beralih ke Video
              </Button>
            {/if}

            <!-- Speaker Toggle -->
            <Button
              variant={call.speakerOn ? 'primary' : 'secondary'}
              onclick={() => mekaarStore.toggleCallSpeaker()}
            >
              🔊 {call.speakerOn ? 'Speaker' : 'Earpiece'}
            </Button>

            <!-- End Call -->
            <Button variant="danger" onclick={() => mekaarStore.endCall()}>
              ☎️ Akhiri
            </Button>
          {/if}
        </div>
      </div>
    </div>
  {/if}
{/if}

<style>
  .call-overlay {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.85);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 1060;
  }

  .call-card {
    width: min(440px, 100%);
    max-height: 92vh;
    overflow-y: auto;
    padding: 24px;
    text-align: center;
    background: var(--surface-card, #FFFFFF);
    border: 1px solid var(--border-subtle, #E2E8F0);
    border-radius: var(--radius-xl, 24px);
    box-shadow: 0 20px 48px -10px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .call-card-top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .minimize-header-btn {
    background: var(--bg-surface, #F1F5F9);
    border: 1px solid var(--border-subtle, #CBD5E1);
    border-radius: var(--radius-pill, 999px);
    font-size: 0.8rem;
    font-weight: 700;
    font-family: var(--font-active);
    min-height: var(--touch-target-min, 40px);
    padding: 0 16px;
    color: var(--text-secondary, #475569);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .minimize-header-btn:hover {
    background: var(--border-light, #E2E8F0);
    color: var(--text-primary, #0F172A);
  }

  .status-badge {
    font-size: 0.78rem;
    font-weight: 700;
    padding: 6px 14px;
    min-height: 32px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-pill, 999px);
  }

  .status-badge.connected {
    background: rgba(16, 185, 129, 0.15);
    color: #10B981;
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .status-badge.dialing, .status-badge.ringing {
    background: rgba(245, 158, 11, 0.15);
    color: #F59E0B;
    border: 1px solid rgba(245, 158, 11, 0.3);
  }

  /* Telemetry Strip */
  .network-telemetry-strip {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    flex-wrap: wrap;
    background: rgba(2, 132, 199, 0.06);
    border: 1px solid rgba(2, 132, 199, 0.18);
    border-radius: 10px;
    padding: 6px 10px;
  }

  .telemetry-pill {
    font-size: 0.72rem;
    color: var(--text-secondary, #64748B);
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .telemetry-pill strong {
    color: var(--text-primary, #0F172A);
  }

  .srtp-tag {
    color: var(--brand-blue, #0284C7);
    font-weight: 700;
  }

  .avatar-center-wrap {
    margin: 4px 0;
  }

  .call-peer-title {
    font-size: 1.35rem;
    font-weight: 800;
    font-family: var(--font-active);
    color: var(--text-primary, #0F172A);
    margin: 0;
  }

  .call-status-sub {
    font-size: 0.8rem;
    color: var(--text-secondary, #64748B);
    margin: 0;
  }

  /* SAS Box */
  .sas-verify-box {
    background: var(--brand-blue-subtle, #EFF6FF);
    border: 1.5px solid var(--brand-blue, #93C5FD);
    border-radius: 14px;
    padding: 12px;
    text-align: center;
  }

  .sas-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 4px;
  }

  .sas-title-label {
    font-size: 0.76rem;
    font-weight: 800;
    color: var(--brand-blue, #1E3A8A);
    letter-spacing: 0.5px;
  }

  .sas-emojis-tag {
    font-size: 1rem;
  }

  .sas-code-display {
    font-family: monospace;
    font-size: 1.35rem;
    font-weight: 800;
    color: var(--brand-blue, #1D4ED8);
    letter-spacing: 3px;
  }

  .sas-explanation-text {
    font-size: 0.74rem;
    color: var(--text-secondary, #475569);
    margin-top: 4px;
    line-height: 1.3;
  }

  /* Video Stage */
  .video-stage {
    height: 180px;
    background: #0B132B;
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
  }

  .video-placeholder-text {
    font-size: 0.84rem;
    color: #94A3B8;
  }

  .video-active-stream {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
  }

  .stream-badge {
    font-size: 0.84rem;
    font-weight: 700;
    color: #E2E8F0;
  }

  .stream-cam-facing {
    font-size: 0.72rem;
    color: #38BDF8;
    background: rgba(0, 0, 0, 0.4);
    padding: 2px 8px;
    border-radius: 6px;
  }

  .pip-self {
    position: absolute;
    bottom: 8px;
    right: 8px;
    padding: 4px 8px;
    font-size: 0.7rem;
    font-weight: 700;
    background: rgba(15, 23, 42, 0.85);
    color: #FFFFFF;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .call-actions-row {
    display: flex;
    justify-content: center;
    gap: 8px;
    flex-wrap: wrap;
    margin-top: 6px;
  }
</style>
