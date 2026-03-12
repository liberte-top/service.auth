<script lang="ts">
  import { onMount } from "svelte";
  import { apiClient } from "../../../openapi/http";

  type AuthContext = {
    authenticated?: boolean;
    email?: string | null;
  };

  let context: AuthContext | null = null;
  let error = "";

  onMount(async () => {
    try {
      const response = await apiClient.get<AuthContext>("/api/v1/context");
      if (!response.data?.authenticated) {
        window.location.assign("/");
        return;
      }
      context = response.data;
    } catch (value) {
      error = value instanceof Error ? value.message : "Unable to load profile.";
    }
  });
</script>

<main class="page profile-page">
  <section class="simple-panel">
    <a class="brand-mark" href="/">liberte.top</a>

    <section class="card profile-card-simple">
      {#if error}
        <p class="banner error">{error}</p>
      {:else if context}
        <p class="section-label">Profile</p>
        <h1>{context.email}</h1>
        <p class="muted">This is the email attached to your current session.</p>

        <div class="actions compact-actions">
          <a class="button-link secondary-link" href="/logout.html">Log out</a>
        </div>
      {:else}
        <p class="muted">Loading profile...</p>
      {/if}
    </section>
  </section>
</main>
