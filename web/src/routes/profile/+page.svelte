<script lang="ts">
  import { translate } from "$lib/i18n/copy";
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<svelte:head>
  <title>{translate(data.language, "auth.profile.title")} - Liberte</title>
  <meta name="description" content={translate(data.language, "auth.profile.description")} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<main class="page profile-page">
  <section class="simple-panel">
    <a class="brand-link" href="/login">liberte.top</a>

    <section class="card profile-card-simple">
      <p class="section-label">{translate(data.language, "auth.profile.title")}</p>
      <h1>{data.email}</h1>
      <p class="muted">{translate(data.language, "auth.profile.subtitle")}</p>

      <form class="actions compact-actions" method="POST" action="/logout">
        <button class="secondary" type="submit">{translate(data.language, "auth.common.logout")}</button>
      </form>
    </section>
  </section>
</main>
