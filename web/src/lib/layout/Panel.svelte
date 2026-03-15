<script lang="ts">
  type Props = {
    children?: import("svelte").Snippet;
    maxWidth?: string;
    gap?: string;
  };

  let {
    children,
    maxWidth = "400px",
    gap = "16px",
  }: Props = $props();
</script>

<section class="panel" style={`--panel-max-width: ${maxWidth}; --panel-gap: ${gap};`}>
  {@render children?.()}
</section>

<style>
  .panel {
    width: 100%;
    max-width: var(--panel-max-width);
    display: grid;
    gap: var(--panel-gap);
  }
</style>
