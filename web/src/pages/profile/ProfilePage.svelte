<script lang="ts">
  import { onMount } from "svelte";
  import { apiClient } from "../../../openapi/http";

  type AuthContext = {
    authenticated?: boolean;
    subject?: string | null;
    auth_type?: string | null;
    scopes?: string[];
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

<main class="shell">
  <section class="hero hero-card card">
    <p class="eyebrow">Profile</p>
    <h1>Your auth session is ready.</h1>
    <p class="lede">This is the default landing page when no rewrite target is provided. From here you can confirm the active session and continue to other liberte.top surfaces.</p>
  </section>

  <section class="card profile-card">
    {#if error}
      <p class="banner error">{error}</p>
    {:else if context}
      <p class="eyebrow">Session details</p>
      <dl class="profile-grid">
        <div>
          <dt>Subject</dt>
          <dd><code>{context.subject}</code></dd>
        </div>
        <div>
          <dt>Auth type</dt>
          <dd>{context.auth_type}</dd>
        </div>
        <div>
          <dt>Scopes</dt>
          <dd>{(context.scopes || []).join(", ") || "none"}</dd>
        </div>
      </dl>

      <p class="hint">Your browser already holds the auth cookie, so linked apps under the same session domain can continue without another sign-in prompt.</p>

      <div class="actions">
        <a class="button-link" href="/">Back to auth home</a>
        <a class="button-link secondary-link" href="https://smoke.liberte.top/">Open smoke app</a>
      </div>
    {:else}
      <p>Loading profile...</p>
    {/if}
  </section>
</main>
