<script lang="ts">
  type Props = {
    children?: import("svelte").Snippet;
  };

  let { children }: Props = $props();
</script>

<p class="centered-muted-text">
  {@render children?.()}
</p>

<style>
  .centered-muted-text {
    margin: 0;
    color: var(--lt-color-text-muted);
    line-height: 1.5;
    text-align: center;
    font-size: 14px;
  }
</style>
