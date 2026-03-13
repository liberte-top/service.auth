<script lang="ts">
  import { translate } from "$lib/i18n/copy";
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const signedOut = form?.success ?? false;
</script>

<svelte:head>
  <title>{signedOut ? `${translate(data.language, "auth.logout.successTitle")} - Liberte` : translate(data.language, "auth.logout.title")}</title>
  <meta name="description" content={translate(data.language, "auth.logout.confirmBody")} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<main class="page logout-page">
  <section class="simple-panel">
    <a class="brand-link" href="/login">liberte.top</a>

    <section class="card logout-card">
      <p class="section-label">{translate(data.language, "auth.logout.label")}</p>
      <h1>{signedOut ? translate(data.language, "auth.logout.successTitle") : translate(data.language, "auth.logout.confirmTitle")}</h1>
      <p class="muted">{signedOut ? translate(data.language, "auth.logout.successBody") : translate(data.language, "auth.logout.confirmBody")}</p>

      {#if signedOut}
        <div class="actions compact-actions">
          <a class="button-link" href="/login">{translate(data.language, "auth.logout.back")}</a>
        </div>
      {:else}
        <form class="actions compact-actions" method="POST">
          <button type="submit">{translate(data.language, "auth.common.logout")}</button>
          <a class="button-link secondary-link" href="/login">{translate(data.language, "auth.logout.cancel")}</a>
        </form>
      {/if}
    </section>
  </section>
</main>
