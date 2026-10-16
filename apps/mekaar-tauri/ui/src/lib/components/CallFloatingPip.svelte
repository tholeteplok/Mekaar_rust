<script lang="ts">
  import { mekaarStore } from '../stores';
  import SketchAvatar from './SketchAvatar.svelte';

  function formatDuration(sec: number): string {
    const m = Math.floor(sec / 60);
    const s = sec % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }
</script>

{#if mekaarStore.activeCall && mekaarStore.activeCall.isMinimized}
  {@const call = mekaarStore.activeCall}

  <!-- Floating Picture-in-Picture Widget Container -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="call-pip-card"
    onclick={() => mekaarStore.toggleCallMinimize()}
    role="region"
    aria-label="Jendela Panggilan Mengambang (PiP)"
  >
    <!-- Header: Status, Duration, Network & Maximize Hint -->
    <div class="pip-header-row">
      <div class="pip-status-left">
        <span class="pip-pulse-green"></span>
        <span class="pip-duration-text">{formatDuration(call.durationSeconds)}</span>
      </div>

      <div class="pip-header-right">
        <span class="pip-network-badge" title="Kualitas Sambungan Panggilan">
          📶 {call.latencyMs ?? 38}ms
        </span>
        <button
          type="button"
          class="pip-maximize-icon-btn"
          onclick={(e) => {
            e.stopPropagation();
            mekaarStore.toggleCallMinimize();
          }}
          title="Perbesar Layar Panggilan"
          aria-label="Perbesar"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 3 21 3 21 9"></polyline>
            <polyline points="9 21 3 21 3 15"></polyline>
            <line x1="21" y1="3" x2="14" y2="10"></line>
            <line x1="3" y1="21" x2="10" y2="14"></line>
          </svg>
        </button>
      </div>
    </div>

    <!-- Center Stage: Video Stream / Audio Waveform -->
    <div class="pip-media-stage">
      {#if call.isVideo && !call.mutedVideo}
        <!-- Simulated Realtime WebRTC Video Canvas -->
        <div class="pip-video-viewport">
          <div class="pip-video-placeholder">
            <span class="video-grid-indicator">🎥 720p HD P2P</span>
            <span class="video-peer-tag">{call.peerName}</span>
          </div>
          <!-- Camera facing indicator badge -->
          <div class="pip-camera-tag">
            {call.cameraFacing === 'environment' ? '📷 Belakang' : '🤳 Depan'}
          </div>
        </div>
      {:else}
        <!-- Audio Mode: Centered Avatar with Concentric Pulse -->
        <div class="pip-audio-viewport">
          <div class="audio-pulse-ring anim-pulse-wave"></div>
          <div class="pip-avatar-wrap">
            <SketchAvatar name={call.peerName} color={call.avatarColor} size={44} online={true} />
          </div>
          <div class="pip-peer-info">
            <span class="pip-peer-name">{call.peerName}</span>
            <span class="pip-sas-tag" title="Kode Verifikasi Anti-Sadap SAS">
              🛡️ {call.encryptionSAS}
            </span>
          </div>
        </div>
      {/if}
    </div>

    <!-- Quick Control Bar -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="pip-controls-dock" onclick={(e) => e.stopPropagation()}>
      <!-- Mute Mic Toggle -->
      <button
        type="button"
        class="pip-ctl-btn {call.mutedAudio ? 'btn-danger' : 'btn-normal'}"
        onclick={() => mekaarStore.toggleCallMute()}
        title={call.mutedAudio ? 'Nyalakan Mikrofon' : 'Bisukan Mikrofon'}
        aria-label="Mikrofon"
      >
        {call.mutedAudio ? '🔇' : '🎙️'}
      </button>

      <!-- Video / Camera Toggle -->
      {#if call.isVideo}
        <button
          type="button"
          class="pip-ctl-btn {call.mutedVideo ? 'btn-warn' : 'btn-normal'}"
          onclick={() => mekaarStore.toggleCallVideo()}
          title={call.mutedVideo ? 'Nyalakan Kamera' : 'Matikan Kamera'}
          aria-label="Kamera"
        >
          {call.mutedVideo ? '📷' : '🎥'}
        </button>
      {:else}
        <button
          type="button"
          class="pip-ctl-btn btn-normal"
          onclick={() => mekaarStore.upgradeToVideoCall()}
          title="Beralih ke Panggilan Video"
          aria-label="Beralih ke Video"
        >
          🎥
        </button>
      {/if}

      <!-- Flip Camera (If video is active) -->
      {#if call.isVideo && !call.mutedVideo}
        <button
          type="button"
          class="pip-ctl-btn btn-normal"
          onclick={() => mekaarStore.flipCamera()}
          title="Balik Kamera (Depan / Belakang)"
          aria-label="Balik Kamera"
        >
          🔄
        </button>
      {/if}

      <!-- End Call Button -->
      <button
        type="button"
        class="pip-ctl-btn btn-end-call"
        onclick={() => mekaarStore.endCall()}
        title="Akhiri Panggilan"
        aria-label="Akhiri Panggilan"
      >
        ☎️
      </button>
    </div>
  </div>
{/if}

<style>
  .call-pip-card {
    position: fixed;
    bottom: 84px;
    right: 20px;
    width: min(280px, calc(100vw - 32px));
    background: linear-gradient(145deg, #0F172A, #1E293B);
    border: 1.5px solid rgba(56, 189, 248, 0.4);
    border-radius: var(--radius-xl, 20px);
    box-shadow: 0 16px 36px -8px rgba(0, 0, 0, 0.55), 0 0 20px rgba(2, 132, 199, 0.2);
    padding: 10px 12px;
    z-index: 1050;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    gap: 8px;
    animation: slideUpScale 0.24s cubic-bezier(0.16, 1, 0.3, 1);
    user-select: none;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
  }

  .call-pip-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 20px 42px -6px rgba(0, 0, 0, 0.65), 0 0 26px rgba(2, 132, 199, 0.3);
    border-color: rgba(56, 189, 248, 0.7);
  }

  /* Header */
  .pip-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .pip-status-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .pip-pulse-green {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #10B981;
    box-shadow: 0 0 8px #10B981;
    animation: pipPulse 1.6s infinite ease-in-out;
  }

  .pip-duration-text {
    font-size: 0.78rem;
    font-weight: 800;
    color: #F8FAFC;
    font-family: monospace;
    letter-spacing: 0.5px;
  }

  .pip-header-right {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .pip-network-badge {
    font-size: 0.68rem;
    color: #38BDF8;
    background: rgba(56, 189, 248, 0.12);
    padding: 2px 6px;
    border-radius: var(--radius-pill, 999px);
    border: 1px solid rgba(56, 189, 248, 0.25);
    font-weight: 700;
  }

  .pip-maximize-icon-btn {
    width: 34px;
    height: 34px;
    min-width: 34px;
    min-height: 34px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.12);
    border: none;
    color: #CBD5E1;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.15s ease, color 0.15s ease, transform 0.1s ease;
  }

  .pip-maximize-icon-btn:hover {
    background: var(--brand-blue, #0284C7);
    color: #FFFFFF;
  }

  .pip-maximize-icon-btn:active {
    transform: scale(0.92);
  }

  /* Media Stage */
  .pip-media-stage {
    position: relative;
    border-radius: 12px;
    overflow: hidden;
    background: #020617;
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  /* Video Viewport */
  .pip-video-viewport {
    height: 94px;
    position: relative;
    background: linear-gradient(135deg, #091E3A, #020617);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .pip-video-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
  }

  .video-grid-indicator {
    font-size: 0.72rem;
    font-weight: 800;
    color: #38BDF8;
    letter-spacing: 0.5px;
  }

  .video-peer-tag {
    font-size: 0.76rem;
    font-weight: 700;
    color: #F8FAFC;
  }

  .pip-camera-tag {
    position: absolute;
    bottom: 6px;
    left: 8px;
    font-size: 0.65rem;
    font-weight: 700;
    color: #94A3B8;
    background: rgba(0, 0, 0, 0.6);
    padding: 2px 6px;
    border-radius: 6px;
  }

  /* Audio Viewport */
  .pip-audio-viewport {
    height: 78px;
    display: flex;
    align-items: center;
    padding: 0 10px;
    gap: 10px;
    position: relative;
  }

  .pip-avatar-wrap {
    position: relative;
    z-index: 2;
  }

  .audio-pulse-ring {
    position: absolute;
    left: 20px;
    top: 50%;
    transform: translateY(-50%);
    width: 50px;
    height: 50px;
    border-radius: 50%;
    background: rgba(16, 185, 129, 0.2);
    z-index: 1;
  }

  .pip-peer-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
    z-index: 2;
  }

  .pip-peer-name {
    font-size: 0.84rem;
    font-weight: 800;
    color: #F8FAFC;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .pip-sas-tag {
    font-size: 0.72rem;
    font-weight: 800;
    color: #60A5FA;
    font-family: monospace;
    letter-spacing: 1px;
  }

  /* Controls Dock */
  .pip-controls-dock {
    display: flex;
    align-items: center;
    justify-content: space-around;
    padding-top: 2px;
  }

  .pip-ctl-btn {
    width: var(--touch-target-min, 40px);
    height: var(--touch-target-min, 40px);
    min-width: var(--touch-target-min, 40px);
    min-height: var(--touch-target-min, 40px);
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.12rem;
    cursor: pointer;
    transition: transform 0.15s ease, background-color 0.15s ease;
  }

  .pip-ctl-btn:active {
    transform: scale(0.92);
  }

  .btn-normal {
    background-color: rgba(255, 255, 255, 0.12);
    color: #FFFFFF;
  }

  .btn-normal:hover {
    background-color: rgba(255, 255, 255, 0.22);
  }

  .btn-danger {
    background-color: #EF4444;
    color: #FFFFFF;
  }

  .btn-warn {
    background-color: #F59E0B;
    color: #FFFFFF;
  }

  .btn-end-call {
    background-color: #DC2626;
    color: #FFFFFF;
  }

  .btn-end-call:hover {
    background-color: #B91C1C;
  }

  @keyframes pipPulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(1.2); }
  }

  @keyframes anim-pulse-wave {
    0% { transform: translateY(-50%) scale(0.9); opacity: 0.8; }
    50% { transform: translateY(-50%) scale(1.3); opacity: 0.25; }
    100% { transform: translateY(-50%) scale(0.9); opacity: 0.8; }
  }

  @keyframes slideUpScale {
    from {
      opacity: 0;
      transform: translateY(16px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
