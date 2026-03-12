<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<svelte:head>
  <title>Profile - Liberte</title>
</svelte:head>

<main class="page profile-page">
  <section class="simple-panel">
    <a class="brand-link" href="/">liberte.top</a>

    <section class="card profile-card-simple">
      <p class="section-label">Profile</p>
      <h1>{data.email}</h1>
      <p class="muted">This is the email attached to your current session.</p>

      <div class="actions compact-actions">
        <a class="button-link secondary-link" href="/logout">Log out</a>
      </div>
    </section>
  </section>
</main>
