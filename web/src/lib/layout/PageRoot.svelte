<script lang="ts">
  type Props = {
    children?: import("svelte").Snippet;
  };

  let { children }: Props = $props();
</script>

<main class="page-root">
  {@render children?.()}
</main>

<style>
  .page-root {
    min-height: 100vh;
    padding: 40px 16px;
    display: flex;
    justify-content: center;
    align-items: flex-start;
  }

  @media (max-width: 640px) {
    .page-root {
      padding: 24px 16px;
    }
  }
</style>
