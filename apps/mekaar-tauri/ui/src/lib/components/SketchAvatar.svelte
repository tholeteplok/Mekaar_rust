<script lang="ts">
  interface Props {
    name: string;
    color?: string;
    size?: number;
    online?: boolean;
    isGroup?: boolean;
  }

  let {
    name,
    color = '#BFDBFE',
    size = 44,
    online = false,
    isGroup = false
  }: Props = $props();

  const initials = $derived(
    name
      .replace(/^@/, '')
      .split(' ')
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? '')
      .join('') || 'MK'
  );
</script>

<div
  class="crayon-avatar"
  style="width: {size}px; height: {size}px; background-color: {color}; font-size: {Math.round(size * 0.4)}px;"
  title={name}
>
  <span class="tilted-glyph">{isGroup ? '👥' : initials}</span>
  {#if online}
    <span
      class="online-dot"
      style="position: absolute; bottom: 2px; right: 2px; width: 10px; height: 10px; border-radius: 999px; background: #10B981; border: 2px solid #1C1B18;"
    ></span>
  {/if}
</div>
