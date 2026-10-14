<script lang="ts">
  interface Props {
    size?: number;
    variant?: 'raw' | 'sketch' | 'circle';
    alt?: string;
    class?: string;
  }

  let { size = 32, variant = 'sketch', alt = 'MEKAAR Logo', class: className = '' }: Props = $props();
</script>

<img
  src="/brand-icon.png"
  {alt}
  width={size}
  height={size}
  class="brand-icon variant-{variant} {className}"
  style="width: {size}px; height: {size}px;"
  loading="lazy"
/>

<style>
  .brand-icon {
    display: inline-block;
    flex-shrink: 0;
    object-fit: contain;
    vertical-align: middle;
    transition: transform 0.15s ease;
  }

  .variant-sketch {
    border: 2px solid var(--sketch-border-color);
    box-shadow: 2px 2px 0px var(--sketch-shadow);
    border-radius: var(--sketch-radius-md, 12px);
    background: #ffffff;
    padding: 2px;
  }

  .variant-circle {
    border: 2px solid var(--sketch-border-color);
    border-radius: 9999px;
    background: #ffffff;
    padding: 2px;
  }

  .variant-raw {
    border-radius: 6px;
  }
</style>
