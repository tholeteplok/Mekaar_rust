<script lang="ts">
  import { mekaarStore } from '../stores';
  import Button from '../components/Button.svelte';
  import SketchAvatar from '../components/SketchAvatar.svelte';

  function formatDuration(sec: number): string {
    const m = Math.floor(sec / 60);
    const s = sec % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }
</script>

{#if mekaarStore.activeCall}
  {@const call = mekaarStore.activeCall}

  {#if call.isMinimized}
    <!-- Floating Minimalist PIP Pill Widget -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="call-minimized-pill sketch-card"
      onclick={() => mekaarStore.toggleCallMinimize()}
    >
      <div style="display: flex; align-items: center; gap: 8px;">
        <span class="pulse-call-dot"></span>
        <div style="text-align: left;">
          <div style="font-weight: 700; font-size: 0.85rem; line-height: 1.1;">{call.peerName}</div>
          <div style="font-size: 0.72rem; color: var(--text-secondary);">
            {call.isVideo ? '🎥' : '📞'} {formatDuration(call.durationSeconds)} • SAS: {call.encryptionSAS}
          </div>
        </div>
      </div>
      <div style="display: flex; align-items: center; gap: 6px;" onclick={(e) => e.stopPropagation()}>
        <button
          type="button"
          class="mini-btn {call.mutedAudio ? 'btn-danger' : ''}"
          onclick={() => mekaarStore.toggleCallMute()}
          title={call.mutedAudio ? 'Unmute' : 'Mute'}
        >
          {call.mutedAudio ? '🔇' : '🎙️'}
        </button>
        <button
          type="button"
          class="mini-btn btn-danger"
          onclick={() => mekaarStore.endCall()}
          title="Akhiri Panggilan"
        >
          ☎️
        </button>
      </div>
    </div>
  {:else}
    <!-- Full Call Overlay Screen -->
    <div class="call-overlay">
      <div class="call-card sketch-card">
        <!-- Header Bar: Minimize Button & Call Type -->
        <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
          <button
            type="button"
            class="minimize-header-btn"
            onclick={() => mekaarStore.toggleCallMinimize()}
            title="Ciutkan Layar Panggilan"
          >
            ▼ Perkecil
          </button>
          <span class="sketch-badge" style="background: {call.state === 'connected' ? '#DCFCE7' : '#FEF3C7'}; font-weight: 700;">
            {call.state === 'dialing'
              ? 'Memanggil...'
              : call.state === 'ringing'
              ? 'Panggilan Masuk...'
              : `🟢 Terkoneksi (${formatDuration(call.durationSeconds)})`}
          </span>
        </div>

        <!-- Avatar & Nama Lawan Bicara -->
        <div style="margin: 14px 0 10px;">
          <SketchAvatar name={call.peerName} color={call.avatarColor} size={84} online={true} />
        </div>

        <h2 style="font-family: var(--font-crayon); font-size: 1.35rem; margin-bottom: 4px;">{call.peerName}</h2>
        <p style="font-size: 0.84rem; color: var(--text-secondary); margin-bottom: 14px;">
          {#if call.state === 'ringing'}
            🔔 Panggilan {call.isVideo ? 'Video' : 'Suara'} Masuk via Relay...
          {:else if call.state === 'dialing'}
            📡 Menghubungkan WebRTC P2P SRTP...
          {:else}
            🔒 Terenkripsi E2EE WebRTC SRTP • Sejak {call.startedAt}
          {/if}
        </p>

        <!-- SAS (Short Authentication String) Verification Box Anti-MITM -->
        <div class="sas-verify-box sketch-card">
          <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 4px;">
            <span style="font-size: 0.76rem; font-weight: 700; color: #1E3A8A; letter-spacing: 0.5px;">
              VERIFIKASI ANTI-SADAP (SAS)
            </span>
            <span style="font-size: 0.95rem;">{call.sasEmojis ?? '🛡️ ⚡'}</span>
          </div>
          <div style="font-family: var(--font-mono); font-size: 1.15rem; font-weight: 800; color: #1D4ED8; letter-spacing: 2px;">
            {call.encryptionSAS}
          </div>
          <div style="font-size: 0.72rem; color: var(--text-secondary); margin-top: 4px; line-height: 1.3;">
            Cocokkan kode 4 digit & emoji ini dengan lawan bicara untuk memastikan tidak ada pihak ketiga yang menyadap panggilan.
          </div>
        </div>

        <!-- Video Stage HD (Jika Mode Video Aktif) -->
        {#if call.isVideo && call.state === 'connected'}
          <div class="video-stage sketch-card">
            {#if call.mutedVideo}
              <div style="font-size: 0.84rem; color: #94A3B8;">🎥 Video Dinonaktifkan</div>
            {:else}
              <div style="font-size: 0.84rem; color: #E2E8F0;">🎥 Aliran Video Terenkripsi P2P ({call.peerName})</div>
            {/if}
            <div class="pip-self sketch-card">
              {call.mutedVideo ? 'Kamera Mati' : 'Kamera Anda'}
            </div>
          </div>
        {/if}

        <!-- Tombol Aksi Kontrol Panggilan -->
        <div style="display: flex; justify-content: center; gap: 10px; flex-wrap: wrap; margin-top: 16px;">
          {#if call.state === 'ringing'}
            <Button variant="primary" onclick={() => mekaarStore.acceptIncomingCall()}>
              📞 Terima Panggilan
            </Button>
            <Button variant="danger" onclick={() => mekaarStore.rejectIncomingCall()}>
              ✕ Tolak
            </Button>
          {:else}
            <Button
              variant={call.mutedAudio ? 'danger' : 'secondary'}
              onclick={() => mekaarStore.toggleCallMute()}
            >
              {call.mutedAudio ? '🔇 Mic Mati' : '🎙️ Mic Aktif'}
            </Button>

            <Button
              variant={call.mutedVideo ? 'secondary' : 'primary'}
              onclick={() => mekaarStore.toggleCallVideo()}
            >
              {call.mutedVideo ? '📷 Buka Kamera' : '🎥 Tutup Kamera'}
            </Button>

            <Button
              variant={call.speakerOn ? 'primary' : 'secondary'}
              onclick={() => mekaarStore.toggleCallSpeaker()}
            >
              🔊 {call.speakerOn ? 'Speaker' : 'Earpiece'}
            </Button>

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
    background: rgba(15, 20, 28, 0.82);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 200;
  }
  .call-card {
    width: min(440px, 100%);
    padding: 22px;
    text-align: center;
    background: #FFFFFF;
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.35);
  }
  .minimize-header-btn {
    background: var(--bg-secondary);
    border: 1.5px solid var(--sketch-border-color);
    border-radius: 8px;
    font-size: 0.78rem;
    font-weight: 700;
    padding: 4px 10px;
    cursor: pointer;
  }
  .sas-verify-box {
    background: #EFF6FF;
    border: 1.5px solid #93C5FD;
    padding: 10px 14px;
    margin-bottom: 14px;
  }
  .video-stage {
    height: 180px;
    background: #0F172A;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    margin-bottom: 12px;
    border-radius: var(--radius-md);
  }
  .pip-self {
    position: absolute;
    bottom: 8px;
    right: 8px;
    padding: 4px 8px;
    font-size: 0.72rem;
    background: #1E293B;
    color: #FFFFFF;
    border: 1px solid #475569;
  }
  /* Floating Minimized Pill Widget */
  .call-minimized-pill {
    position: fixed;
    bottom: 84px;
    right: 18px;
    background: #FFFFFF;
    padding: 8px 14px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    border: 2px solid var(--sketch-border-color);
    border-radius: 999px;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.22);
    z-index: 250;
    cursor: pointer;
  }
  .pulse-call-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #10B981;
    box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.35);
  }
  .mini-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--sketch-border-color);
    border-radius: 50%;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .mini-btn.btn-danger {
    background: #FEE2E2;
    border-color: #EF4444;
  }
</style>
