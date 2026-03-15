<script lang="ts">
  import type { HTMLAnchorAttributes } from "svelte/elements";

  type Props = HTMLAnchorAttributes & {
    children?: import("svelte").Snippet;
  };

  let {
    children,
    href = "/login",
    class: className = "",
    ...restProps
  }: Props = $props();
</script>

<a {...restProps} href={href} class={`brand-link ${className}`.trim()}>
  {@render children?.()}
</a>

<style>
  .brand-link {
    justify-self: center;
    font-size: 14px;
    font-weight: var(--lt-font-weight-strong);
    color: var(--lt-color-text);
    text-decoration: none;
  }

  .brand-link:hover {
    text-decoration: none;
  }
</style>
