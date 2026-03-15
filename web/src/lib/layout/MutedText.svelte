<script lang="ts">
  type Props = {
    children?: import("svelte").Snippet;
  };

  let { children }: Props = $props();
</script>

<p class="muted-text">
  {@render children?.()}
</p>

<style>
  .muted-text {
    color: var(--lt-color-text-muted);
    line-height: 1.5;
  }
</style>
