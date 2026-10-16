<script lang="ts">
  interface Props {
    name: string;
    size?: number;
    online?: boolean;
    isGuardian?: boolean;
    isGroup?: boolean;
    color?: string;
  }

  let {
    name,
    size = 48,
    online = false,
    isGuardian = false,
    isGroup = false,
    color
  }: Props = $props();

  const initial = $derived(
    isGroup ? '👥' : (name.replace(/^@/, '').trim()[0]?.toUpperCase() || 'M')
  );
  // Stable hash based on name to keep pattern ID consistent without re-render flicker
  const patternId = $derived(
    `hatch-${name.replace(/[^a-zA-Z0-9]/g, '').toLowerCase() || 'default'}-${size}`
  );
</script>

<div
  class="crayon-avatar-container {isGuardian ? 'guardian-frame' : ''}"
  style="width: {size}px; height: {size}px;"
  title={name}
>
  {#if isGroup}
    <div
      class="group-circle"
      style="width: {size}px; height: {size}px; font-size: {Math.round(size * 0.44)}px;"
    >
      👥
    </div>
  {:else}
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      class="crayon-avatar-svg"
    >
      <defs>
        <!-- Blue wax crayon diagonal scribble pattern -->
        <pattern
          id={patternId}
          patternUnits="userSpaceOnUse"
          width="7"
          height="7"
          patternTransform="rotate(45)"
        >
          <line
            x1="0"
            y1="0"
            x2="0"
            y2="7"
            stroke="var(--brand-blue)"
            stroke-width="2.6"
            stroke-linecap="round"
          />
        </pattern>

        <!-- Mask shaped as the tilted -25° letter -->
        <mask id="{patternId}-mask">
          <rect width="100" height="100" fill="black" />
          <text
            x="50"
            y="54"
            text-anchor="middle"
            dominant-baseline="central"
            font-family="var(--font-crayon), DynaPuff, cursive, sans-serif"
            font-weight="700"
            font-size="78"
            fill="white"
            transform="rotate(-25 50 54)"
          >
            {initial}
          </text>
        </mask>
      </defs>

      <!-- Clean Base Circle with 1.5px (scaled to viewBox: 3px) contour -->
      <circle
        cx="50"
        cy="50"
        r="47"
        fill="#FFFDF8"
        stroke="#1E293B"
        stroke-width="3"
      />

      <!-- Scribbled Blue Wax Crayon Hatching inside letter -->
      <rect
        width="100"
        height="100"
        fill="url(#{patternId})"
        mask="url(#{patternId}-mask)"
      />

      <!-- Crisp Charcoal Outline around the tilted letter -->
      <text
        x="50"
        y="54"
        text-anchor="middle"
        dominant-baseline="central"
        font-family="var(--font-crayon), DynaPuff, cursive, sans-serif"
        font-weight="700"
        font-size="78"
        fill="none"
        stroke="#1E293B"
        stroke-width="2.8"
        stroke-linejoin="round"
        stroke-linecap="round"
        transform="rotate(-25 50 54)"
      >
        {initial}
      </text>
    </svg>
  {/if}

  {#if online}
    <span class="online-indicator"></span>
  {/if}
</div>

<style>
  .crayon-avatar-container {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .guardian-frame {
    padding: 2px;
    background: var(--safe-teal);
    border-radius: 50%;
  }

  .group-circle {
    border-radius: 50%;
    background-color: #EFF6FF;
    border: 1.5px solid #1E293B;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .crayon-avatar-svg {
    display: block;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    overflow: hidden;
  }

  .online-indicator {
    position: absolute;
    bottom: 0px;
    right: 0px;
    width: 11px;
    height: 11px;
    border-radius: 50%;
    background-color: #10B981;
    border: 2px solid #FFFFFF;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }
</style>
