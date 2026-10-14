<script lang="ts">
  import { mekaarStore } from '../stores';

  export type MikaPoseName =
    | 'hi'
    | 'neutral'
    | 'love'
    | 'happy'
    | 'shield'
    | 'ok'
    | 'phone'
    | 'welcome'
    | 'confused'
    | 'huft';

  interface Props {
    pose?: MikaPoseName;
    size?: number;
    alt?: string;
  }

  let { pose = 'hi', size = 160, alt = 'Maskot Mika' }: Props = $props();

  const poseIndexMap: Record<MikaPoseName, number> = {
    hi: 0,
    neutral: 1,
    love: 2,
    happy: 3,
    shield: 12,
    ok: 17,
    phone: 18,
    welcome: 20,
    confused: 22,
    huft: 24
  };

  const index = $derived(poseIndexMap[pose] ?? 0);
  const isDark = $derived(mekaarStore.themeMode === 'dark');
  const assetSrc = $derived(
    isDark
      ? `/mascot/mika_dark/mika_dark_${index}.png`
      : `/mascot/mika_light/mika_${index}.png`
  );
</script>

<div
  class="mika-mascot-wrapper"
  style="width: {size}px; height: {size}px;"
>
  <img
    src={assetSrc}
    {alt}
    class="mika-img"
    draggable="false"
    loading="eager"
  />
</div>

<style>
  .mika-mascot-wrapper {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    user-select: none;
    -webkit-user-select: none;
    flex-shrink: 0;
  }

  .mika-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    filter: drop-shadow(0 6px 14px rgba(21, 38, 65, 0.08));
    animation: mika-float 3s ease-in-out infinite alternate;
  }

  @keyframes mika-float {
    0% {
      transform: translateY(0px);
    }
    100% {
      transform: translateY(-5px);
    }
  }
</style>
