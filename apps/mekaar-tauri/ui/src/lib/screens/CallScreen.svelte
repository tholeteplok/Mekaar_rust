<script lang="ts">
  import { mekaarStore } from '../stores';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';
</script>

{#if mekaarStore.activeCall}
  {@const call = mekaarStore.activeCall}
  <div class="call-overlay">
    <div class="call-card sketch-card">
      <span class="sketch-badge" style="background: #DBEAFE; margin-bottom: 10px;">
        🔒 WebRTC E2EE SAS: {call.encryptionSAS}
      </span>

      <div style="margin: 14px 0;">
        <SketchAvatar name={call.peerName} color={call.avatarColor} size={84} online={true} />
      </div>

      <h2 style="font-family: var(--font-crayon); font-size: 1.35rem;">{call.peerName}</h2>
      <p style="font-size: 0.88rem; color: var(--text-secondary); margin-bottom: 18px;">
        {#if call.state === 'ringing'}
          🔔 Panggilan {call.isVideo ? 'Video' : 'Suara'} Masuk...
        {:else}
          🟢 Terhubung ({call.isVideo ? 'Video HD WebRTC' : 'Audio Opus E2EE'}) • Sejak {call.startedAt}
        {/if}
      </p>

      {#if call.isVideo && call.state === 'connected'}
        <div class="video-stage sketch-card">
          <div style="font-size: 0.82rem; color: #E5E7EB;">🎥 Aliran Video P2P ({call.peerName})</div>
          <div class="pip-self sketch-card">Kamera Anda</div>
        </div>
      {/if}

      <div style="display: flex; justify-content: center; gap: 10px; flex-wrap: wrap; margin-top: 14px;">
        {#if call.state === 'ringing'}
          <Button variant="primary" onclick={() => mekaarStore.acceptIncomingCall()}>📞 Terima Panggilan</Button>
          <Button variant="danger" onclick={() => mekaarStore.endCall()}>✕ Tolak</Button>
        {:else}
          <Button
            variant={call.mutedAudio ? 'danger' : 'secondary'}
            onclick={() => (call.mutedAudio = !call.mutedAudio)}
          >
            {call.mutedAudio ? '🔇 Mic Mati' : '🎙️ Mic Aktif'}
          </Button>
          <Button
            variant={call.mutedVideo ? 'secondary' : 'primary'}
            onclick={() => (call.mutedVideo = !call.mutedVideo)}
          >
            {call.mutedVideo ? '📷 Kamera Mati' : '🎥 Kamera Aktif'}
          </Button>
          <Button
            variant={call.speakerOn ? 'primary' : 'secondary'}
            onclick={() => (call.speakerOn = !call.speakerOn)}
          >
            🔊 Speaker
          </Button>
          <Button variant="danger" onclick={() => mekaarStore.endCall()}>☎️ Akhiri</Button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .call-overlay {
    position: fixed;
    inset: 0;
    background: rgba(15, 20, 28, 0.78);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 18px;
    z-index: 150;
  }
  .call-card {
    width: min(460px, 100%);
    padding: 24px;
    text-align: center;
  }
  .video-stage {
    height: 190px;
    background: #111827;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    margin-bottom: 12px;
  }
  .pip-self {
    position: absolute;
    bottom: 10px;
    right: 10px;
    padding: 6px 10px;
    font-size: 0.72rem;
    background: #1F2937;
    color: #FFFFFF;
  }
</style>
