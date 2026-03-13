<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<svelte:head>
  <title>Profile - Liberte</title>
  <meta name="description" content="View the email address attached to your current Liberte session." />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<main class="page profile-page">
  <section class="simple-panel">
    <a class="brand-link" href="/login">liberte.top</a>

    <section class="card profile-card-simple">
      <p class="section-label">Profile</p>
      <h1>{data.email}</h1>
      <p class="muted">This is the email attached to your current session.</p>

      <form class="actions compact-actions" method="POST" action="/logout">
        <button class="secondary" type="submit">Log out</button>
      </form>
    </section>
  </section>
</main>
