<script lang="ts">
  type Props = {
    children?: import("svelte").Snippet;
  };

  let { children }: Props = $props();
</script>

<h1 class="display-title">
  {@render children?.()}
</h1>

<style>
  .display-title {
    margin: 0;
    font-size: 24px;
    line-height: 1.25;
    letter-spacing: -0.02em;
  }
</style>
