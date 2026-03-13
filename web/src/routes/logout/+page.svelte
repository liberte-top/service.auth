<script lang="ts">
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const signedOut = form?.success ?? false;
</script>

<svelte:head>
  <title>{signedOut ? "Signed out - Liberte" : "Log out - Liberte"}</title>
  <meta name="description" content="Sign out of your current Liberte session." />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<main class="page logout-page">
  <section class="simple-panel">
    <a class="brand-link" href="/login">liberte.top</a>

    <section class="card logout-card">
      <p class="section-label">Logout</p>
      <h1>{signedOut ? "You are signed out" : "Sign out of this session"}</h1>
      <p class="muted">{signedOut ? "Your session cookie has been cleared." : "Use the button below to clear the current browser session."}</p>

      {#if signedOut}
        <div class="actions compact-actions">
          <a class="button-link" href="/login">Back to sign in</a>
        </div>
      {:else}
        <form class="actions compact-actions" method="POST">
          <button type="submit">Log out</button>
          <a class="button-link secondary-link" href="/login">Cancel</a>
        </form>
      {/if}
    </section>
  </section>
</main>
