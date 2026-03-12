<script lang="ts">
  import { onMount } from "svelte";
  import { apiClient } from "../openapi/http";

  type Mode = "login" | "register";

  type AuthContext = {
    authenticated?: boolean;
    subject?: string | null;
    auth_type?: string | null;
    scopes?: string[];
  };

  const profileUrl = "/profile.html";

  let mode: Mode = "login";
  let email = "";
  let displayName = "";
  let busy = false;
  let bannerTone: "info" | "success" | "error" = "info";
  let banner = "";
  let rewrite = "";

  function sanitizeRewrite(value: string) {
    const trimmed = value.trim();
    if (!trimmed) return "";
    if (trimmed.startsWith("/")) return trimmed;
    try {
      const url = new URL(trimmed);
      return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : "";
    } catch {
      return "";
    }
  }

  function preferredRedirectTarget() {
    return rewrite || profileUrl;
  }

  function resolveRewrite() {
    const url = new URL(window.location.href);
    return sanitizeRewrite(url.searchParams.get("rewrite") || url.searchParams.get("return_to") || "");
  }

  function applyQueryState() {
    const url = new URL(window.location.href);
    const requestedMode = url.searchParams.get("mode");
    const verified = url.searchParams.get("verified") === "1";
    const queryEmail = url.searchParams.get("email") || "";

    if (requestedMode === "login" || requestedMode === "register") {
      mode = requestedMode;
    }
    if (queryEmail) {
      email = queryEmail;
    }
    if (verified) {
      bannerTone = "success";
      banner = rewrite
        ? "Email verified. Request your login link to continue back to your destination."
        : "Email verified. Request your login link to continue to your profile.";
      mode = "login";
    }
  }

  async function loadContext() {
    const response = await apiClient.get<AuthContext>("/api/v1/context");
    return response.data;
  }

  async function submit(path: string, payload: Record<string, unknown>, successMessage: string) {
    busy = true;
    try {
      const response = await apiClient.post(path, payload, { validateStatus: () => true });
      if (response.status >= 200 && response.status < 300) {
        bannerTone = "success";
        banner = successMessage;
        return;
      }
      bannerTone = "error";
      banner = `Request failed with HTTP ${response.status}.`;
    } catch (error) {
      bannerTone = "error";
      banner = error instanceof Error ? error.message : "Request failed.";
    } finally {
      busy = false;
    }
  }

  async function requestRegistration() {
    await submit(
      "/api/v1/auth/register/email",
      {
        email,
        display_name: displayName || null,
        rewrite: rewrite || null,
      },
      "Check your inbox for the verification link."
    );
  }

  async function resendVerification() {
    await submit(
      "/api/v1/auth/verify/email/resend",
      {
        email,
        rewrite: rewrite || null,
      },
      "A fresh verification link is on the way."
    );
  }

  async function requestLogin() {
    await submit(
      "/api/v1/auth/login/email",
      {
        email,
        rewrite: rewrite || null,
      },
      rewrite
        ? "Check your inbox for the sign-in link back to your destination."
        : "Check your inbox for the sign-in link to your profile."
    );
  }

  onMount(async () => {
    rewrite = resolveRewrite();
    applyQueryState();

    try {
      const context = await loadContext();
      if (context.authenticated) {
        window.location.assign(preferredRedirectTarget());
      }
    } catch {
      bannerTone = "error";
      banner = "Auth context is currently unavailable.";
    }
  });
</script>

<main class="shell">
  <section class="hero card">
    <p class="eyebrow">liberte.top auth</p>
    <h1>Register once, sign in by email, and continue where you meant to go.</h1>
    <p class="lede">
      {#if rewrite}
        You are signing in for <code>{rewrite}</code>.
      {:else}
        This is the standard entrypoint for account registration and sign-in.
      {/if}
    </p>
  </section>

  <section class="card auth-card">
    <div class="mode-switch" role="tablist" aria-label="Auth mode">
      <button class:active={mode === "login"} on:click={() => (mode = "login")}>Sign in</button>
      <button class:active={mode === "register"} on:click={() => (mode = "register")}>Register</button>
    </div>

    {#if banner}
      <p class={`banner ${bannerTone}`}>{banner}</p>
    {/if}

    <label>
      Email
      <input bind:value={email} type="email" autocomplete="email" placeholder="you@example.com" />
    </label>

    {#if mode === "register"}
      <label>
        Display name
        <input bind:value={displayName} autocomplete="name" placeholder="Optional" />
      </label>
    {/if}

    <div class="actions">
      {#if mode === "register"}
        <button disabled={busy || !email} on:click={requestRegistration}>
          {busy ? "Sending..." : "Send verification email"}
        </button>
        <button class="secondary" disabled={busy || !email} on:click={resendVerification}>Resend verification</button>
      {:else}
        <button disabled={busy || !email} on:click={requestLogin}>
          {busy ? "Sending..." : "Send sign-in link"}
        </button>
      {/if}
    </div>

    <p class="hint">
      After you complete email verification and sign-in, we redirect using <code>rewrite</code> first and
      your profile second.
    </p>
  </section>
</main>
