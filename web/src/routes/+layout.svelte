<script lang="ts">
  import "../app.css";
  import LanguageSwitcher from "$lib/i18n/LanguageSwitcher.svelte";

  import type { LayoutData } from "./$types";
  import { setContext } from "svelte";

  export let data: LayoutData;

  setContext("liberte-language", data.language);
</script>

<svelte:head>
  <meta name="liberte:build-version" content={data.buildInfo.version} />
  <meta name="liberte:build-sha" content={data.buildInfo.sha} />
  <meta name="liberte:build-timestamp" content={data.buildInfo.timestamp} />
</svelte:head>

<LanguageSwitcher language={data.language} supportedLanguages={data.supportedLanguages} />
<slot />
