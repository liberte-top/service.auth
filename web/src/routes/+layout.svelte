<script lang="ts">
  import "../app.css";

  import type { LayoutData } from "./$types";

  export let data: LayoutData;
</script>

<svelte:head>
  <meta name="liberte:build-version" content={data.buildInfo.version} />
  <meta name="liberte:build-sha" content={data.buildInfo.sha} />
  <meta name="liberte:build-timestamp" content={data.buildInfo.timestamp} />
</svelte:head>

<slot />
