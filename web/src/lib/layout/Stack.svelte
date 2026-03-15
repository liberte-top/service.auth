<script lang="ts">
  type Props = {
    children?: import("svelte").Snippet;
    gap?: string;
    marginTop?: string;
  };

  let {
    children,
    gap = "12px",
    marginTop = "0",
  }: Props = $props();
</script>

<div class="stack" style={`--stack-gap: ${gap}; --stack-margin-top: ${marginTop};`}>
  {@render children?.()}
</div>

<style>
  .stack {
    display: grid;
    gap: var(--stack-gap);
    margin-top: var(--stack-margin-top);
  }

  .stack :global(form) {
    margin: 0;
  }
</style>
