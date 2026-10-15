<script lang="ts">
  import type { AttachmentModel } from '../types';
  import {
    MapPin,
    Timer,
    BatteryCharging,
    LockKey,
    Compass,
    ShieldCheck,
    StopCircle,
    CornersOut
  } from 'phosphor-svelte';

  interface Props {
    attachment: AttachmentModel;
    isSelf?: boolean;
    onopenmap?: (lat: number, lng: number) => void;
    onrequestguardian?: (name: string, lat: number, lng: number) => void;
    onstoplive?: () => void;
  }

  let {
    attachment,
    isSelf = false,
    onopenmap,
    onrequestguardian,
    onstoplive
  }: Props = $props();

  const lat = $derived(attachment.latitude ?? -7.5583);
  const lng = $derived(attachment.longitude ?? 110.8566);
  const isLive = $derived(!!attachment.isLiveLocation);
  const isActive = $derived(attachment.isLiveActive !== false);
</script>

<div class="live-location-card {isSelf ? 'card-self' : 'card-peer'}">
  <!-- 1. Header Banner -->
  <div class="location-card-header">
    <div class="header-status-col">
      <div class="status-indicator-row">
        {#if isLive}
          {#if isActive}
            <span class="live-pulse-dot" title="Lokasi Langsung Aktif"></span>
            <span class="live-title-text">Lokasi Langsung (Live)</span>
          {:else}
            <span class="ended-dot">●</span>
            <span class="ended-title-text">Sesi Lokasi Berakhir</span>
          {/if}
        {:else}
          <span class="static-pin-icon" style="display: flex; align-items: center;">
            <MapPin size={16} weight="fill" color="#0284C7" />
          </span>
          <span class="static-title-text">Titik Lokasi Aman</span>
        {/if}
      </div>

      {#if isLive && isActive}
        <span class="live-timer-sub" style="display: inline-flex; align-items: center; gap: 4px;">
          <Timer size={12} weight="bold" /> Sisa {attachment.liveExpiresAt || `${attachment.liveDurationMinutes || 60}m lagi`}
        </span>
      {/if}
    </div>

    <!-- Battery & Privacy Badge -->
    <div class="telemetry-badges">
      {#if isLive && attachment.batteryLevel}
        <span class="battery-badge" title="Tingkat Daya Perangkat Pengirim" style="display: inline-flex; align-items: center; gap: 4px;">
          <BatteryCharging size={13} weight="bold" /> {attachment.batteryLevel}%
        </span>
      {/if}
      <span class="e2ee-tag" style="display: inline-flex; align-items: center; gap: 4px;">
        <LockKey size={11} weight="bold" /> GPS E2EE
      </span>
    </div>
  </div>

  <!-- 2. Mini-Map SVG Vector Viewport -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="mini-map-viewport"
    onclick={() => onopenmap?.(lat, lng)}
    title="Klik untuk membuka di Peta Radar"
  >
    <svg viewBox="0 0 400 180" class="map-vector-svg" preserveAspectRatio="none">
      <defs>
        <!-- Background Grid Pattern -->
        <pattern id="street-grid" width="30" height="30" patternUnits="userSpaceOnUse">
          <path d="M 30 0 L 0 0 0 30" fill="none" stroke="rgba(255, 255, 255, 0.08)" stroke-width="1"/>
        </pattern>
        <!-- Radar Pulse Radial Gradient -->
        <radialGradient id="radar-glow" cx="50%" cy="50%" r="50%">
          <stop offset="0%" stop-color="#10B981" stop-opacity="0.4"/>
          <stop offset="60%" stop-color="#0284C7" stop-opacity="0.15"/>
          <stop offset="100%" stop-color="#0284C7" stop-opacity="0"/>
        </radialGradient>
      </defs>

      <!-- Map Dark Canvas -->
      <rect width="400" height="180" fill="#0B132B"/>
      <rect width="400" height="180" fill="url(#street-grid)"/>

      <!-- Simulated Road Network -->
      <path d="M -10 50 Q 150 90 410 40" fill="none" stroke="#1E293B" stroke-width="12"/>
      <path d="M -10 50 Q 150 90 410 40" fill="none" stroke="#334155" stroke-width="6"/>
      <path d="M 200 -10 L 200 190" fill="none" stroke="#1E293B" stroke-width="14"/>
      <path d="M 200 -10 L 200 190" fill="none" stroke="#38BDF8" stroke-width="2" stroke-dasharray="6,4" opacity="0.6"/>
      <path d="M 80 190 L 320 -10" fill="none" stroke="#334155" stroke-width="5"/>

      <!-- Radar Pulse Rings (when Live & Active) -->
      {#if isLive && isActive}
        <circle cx="200" cy="90" r="60" fill="url(#radar-glow)" class="anim-radar-wave"/>
        <circle cx="200" cy="90" r="42" fill="none" stroke="#10B981" stroke-width="1.5" class="anim-ring-outer" opacity="0.7"/>
        <circle cx="200" cy="90" r="24" fill="none" stroke="#34D399" stroke-width="1.8" class="anim-ring-inner" opacity="0.9"/>
      {/if}

      <!-- Center Pin Marker -->
      <circle cx="200" cy="90" r="11" fill={isLive && isActive ? '#10B981' : '#0284C7'} stroke="#FFFFFF" stroke-width="3"/>
      <circle cx="200" cy="90" r="4" fill="#FFFFFF"/>
    </svg>

    <!-- Overlay Hint Pill -->
    <div class="map-hover-hint">
      <span style="display: inline-flex; align-items: center; gap: 4px;">
        <CornersOut size={13} weight="bold" /> Perbesar Peta
      </span>
    </div>
  </div>

  <!-- 3. Details Strip -->
  <div class="location-details-col">
    <div class="location-name-row">
      <span class="location-title">{attachment.name || 'Posko Pengamanan'}</span>
      <span class="accuracy-tag">±4m akurat</span>
    </div>

    <span class="coordinates-subtext">
      {lat.toFixed(4)}, {lng.toFixed(4)} • Surakarta
    </span>
  </div>

  <!-- 4. Interactive Action Buttons -->
  <div class="location-actions-grid">
    <button
      type="button"
      class="action-btn btn-open-map"
      onclick={() => onopenmap?.(lat, lng)}
    >
      <Compass size={16} weight="bold" />
      <span>Peta Radar</span>
    </button>

    <button
      type="button"
      class="action-btn btn-guard-escort"
      onclick={() => onrequestguardian?.(attachment.name || 'Posko Pengamanan', lat, lng)}
    >
      <ShieldCheck size={16} weight="bold" />
      <span>Kawal Rute</span>
    </button>

    {#if isSelf && isLive && isActive && onstoplive}
      <button
        type="button"
        class="action-btn btn-stop-live"
        onclick={onstoplive}
        title="Hentikan siaran lokasi langsung"
      >
        <StopCircle size={15} weight="bold" />
        <span>Hentikan</span>
      </button>
    {/if}
  </div>
</div>

<style>
  .live-location-card {
    display: flex;
    flex-direction: column;
    border-radius: 14px;
    overflow: hidden;
    margin-bottom: 6px;
    background-color: #FFFFFF;
    border: 1px solid var(--border-subtle, #E2E8F0);
    box-shadow: 0 2px 10px rgba(15, 23, 42, 0.08);
    width: 100%;
    max-width: 380px;
    color: var(--text-primary, #0F172A);
  }

  .card-self {
    border-color: rgba(255, 255, 255, 0.25);
  }

  .card-peer {
    border-color: var(--border-subtle, #E2E8F0);
  }

  /* Header */
  .location-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background-color: #F8FAFC;
    border-bottom: 1px solid var(--border-light, #E2E8F0);
  }

  .header-status-col {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .status-indicator-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .live-pulse-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #10B981;
    box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7);
    animation: livePulse 1.6s infinite cubic-bezier(0.66, 0, 0, 1);
  }

  @keyframes livePulse {
    to {
      box-shadow: 0 0 0 8px rgba(16, 185, 129, 0);
    }
  }

  .ended-dot {
    color: #94A3B8;
    font-size: 0.7rem;
  }

  .static-pin-icon {
    font-size: 0.9rem;
    line-height: 1;
  }

  .live-title-text {
    font-size: 0.8rem;
    font-weight: 800;
    color: #065F46;
  }

  .ended-title-text {
    font-size: 0.8rem;
    font-weight: 700;
    color: #64748B;
  }

  .static-title-text {
    font-size: 0.8rem;
    font-weight: 800;
    color: var(--brand-blue, #0284C7);
  }

  .live-timer-sub {
    font-size: 0.7rem;
    color: #047857;
    font-weight: 600;
  }

  .telemetry-badges {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .battery-badge {
    background-color: #F1F5F9;
    border: 1px solid #CBD5E1;
    font-size: 0.68rem;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 999px;
    color: #334155;
  }

  .e2ee-tag {
    background-color: rgba(2, 132, 199, 0.12);
    border: 1px solid rgba(2, 132, 199, 0.3);
    font-size: 0.68rem;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 999px;
    color: #0284C7;
  }

  /* Map Viewport */
  .mini-map-viewport {
    position: relative;
    width: 100%;
    height: 140px;
    background-color: #0B132B;
    cursor: pointer;
    overflow: hidden;
  }

  .map-vector-svg {
    width: 100%;
    height: 100%;
    display: block;
    transition: transform 0.25s ease;
  }

  .mini-map-viewport:hover .map-vector-svg {
    transform: scale(1.05);
  }

  .anim-radar-wave {
    animation: radarExpand 2s infinite ease-out;
    transform-origin: center;
  }

  .anim-ring-outer {
    animation: ringPulse 2s infinite ease-out;
    transform-origin: center;
  }

  .anim-ring-inner {
    animation: ringPulse 2s infinite ease-out 0.4s;
    transform-origin: center;
  }

  @keyframes ringPulse {
    0% {
      r: 10px;
      opacity: 1;
    }
    100% {
      r: 52px;
      opacity: 0;
    }
  }

  @keyframes radarExpand {
    0% {
      r: 20px;
      opacity: 0.6;
    }
    100% {
      r: 75px;
      opacity: 0;
    }
  }

  .map-hover-hint {
    position: absolute;
    bottom: 8px;
    right: 8px;
    background: rgba(15, 23, 42, 0.75);
    backdrop-filter: blur(4px);
    color: #FFFFFF;
    font-size: 0.68rem;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .mini-map-viewport:hover .map-hover-hint {
    opacity: 1;
  }

  /* Details */
  .location-details-col {
    padding: 8px 12px 6px 12px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .location-name-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .location-title {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--text-primary, #0F172A);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .accuracy-tag {
    font-size: 0.68rem;
    color: #10B981;
    font-weight: 700;
    white-space: nowrap;
  }

  .coordinates-subtext {
    font-size: 0.74rem;
    color: var(--text-secondary, #64748B);
    font-family: monospace;
  }

  /* Actions */
  .location-actions-grid {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px 10px 10px;
  }

  .action-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    padding: 6px 10px;
    border-radius: 8px;
    font-size: 0.76rem;
    font-weight: 700;
    border: none;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .btn-open-map {
    background-color: #EFF6FF;
    color: var(--brand-blue, #0284C7);
    border: 1px solid #BFDBFE;
  }

  .btn-open-map:hover {
    background-color: #DBEAFE;
  }

  .btn-guard-escort {
    background-color: #F0FDF4;
    color: #15803D;
    border: 1px solid #BBF7D0;
  }

  .btn-guard-escort:hover {
    background-color: #DCFCE7;
  }

  .btn-stop-live {
    background-color: #FFF1F2;
    color: #E11D48;
    border: 1px solid #FECDD3;
    flex: 0 0 auto;
  }

  .btn-stop-live:hover {
    background-color: #FEE2E2;
  }
</style>
