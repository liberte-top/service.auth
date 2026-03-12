<script lang="ts">
  import { onMount } from "svelte";
  import { apiClient } from "../../../openapi/http";

  let busy = true;
  let error = "";

  $: if (typeof document !== "undefined") {
    document.title = error ? "Logout error - Liberte" : busy ? "Signing out - Liberte" : "Signed out - Liberte";
  }

  async function logout() {
    busy = true;
    error = "";

    try {
      const response = await apiClient.post("/api/v1/auth/logout", null, { validateStatus: () => true });
      if (response.status < 200 || response.status >= 300) {
        throw new Error(`Logout failed with HTTP ${response.status}.`);
      }
    } catch (value) {
      error = value instanceof Error ? value.message : "Logout failed.";
    } finally {
      busy = false;
    }
  }

  onMount(async () => {
    await logout();
  });
</script>

<main class="page logout-page">
  <section class="simple-panel">
    <a class="brand-mark" href="/">liberte.top</a>

    <section class="card logout-card">
      <p class="section-label">Logout</p>
      <h1>{error ? "Could not sign you out" : busy ? "Signing you out..." : "You are signed out"}</h1>
      <p class="muted">
        {#if error}
          {error}
        {:else if busy}
          Clearing your session for this browser.
        {:else}
          Your session cookie has been cleared.
        {/if}
      </p>

      <div class="actions compact-actions">
        <a class="button-link" href="/">Back to sign in</a>
        {#if error}
          <button class="secondary" on:click={logout}>Try again</button>
        {/if}
      </div>
    </section>
  </section>
</main>
