<script lang="ts">
  import { onMount } from "svelte";
  import { apiClient } from "../openapi/http";

  type Mode = "login" | "register";

  type AuthContext = {
    authenticated?: boolean;
    subject?: string | null;
    email?: string | null;
    auth_type?: string | null;
    scopes?: string[];
  };

  const profileUrl = "/profile.html";
  const logoutUrl = "/logout.html";

  let mode: Mode = "login";
  let email = "";
  let displayName = "";
  let busy = false;
  let registrationRequested = false;
  let bannerTone: "info" | "success" | "error" = "info";
  let banner = "";
  let rewrite = "";
  let authContext: AuthContext = { authenticated: false, subject: null, email: null, auth_type: null, scopes: [] };

  $: modeTitle = mode === "login" ? "Sign in" : "Create account";
  $: modeSummary = mode === "login"
    ? "We will send a one-time link to your email."
    : "Create your account with email, then verify it to continue.";
  $: primaryLabel = mode === "login" ? "Send sign-in link" : "Create account";
  $: preferredRedirectTarget = rewrite || profileUrl;
  $: pageTitle = `${mode === "login" ? "Sign in" : "Create account"} - Liberte`;

  $: if (typeof document !== "undefined") {
    document.title = pageTitle;
  }

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
      banner = "Email verified. You can now request your sign-in link.";
      mode = "login";
    }
  }

  async function refreshContext() {
    try {
      const response = await apiClient.get<AuthContext>("/api/v1/context", { validateStatus: () => true });
      if (response.status >= 200 && response.status < 300) {
        authContext = response.data;
      }
    } catch {
      authContext = { authenticated: false, subject: null, email: null, auth_type: null, scopes: [] };
    }
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
    registrationRequested = false;
    await submit(
      "/api/v1/auth/register/email",
      {
        email,
        display_name: displayName || null,
        rewrite: rewrite || null,
      },
      "Check your inbox for the verification email."
    );
    if (bannerTone === "success") {
      registrationRequested = true;
    }
  }

  async function resendVerification() {
    await submit(
      "/api/v1/auth/verify/email/resend",
      {
        email,
        rewrite: rewrite || null,
      },
      "Verification email sent again."
    );
  }

  async function requestLogin() {
    await submit(
      "/api/v1/auth/login/email",
      {
        email,
        rewrite: rewrite || null,
      },
      "Check your inbox for your sign-in link."
    );
  }

  onMount(async () => {
    rewrite = resolveRewrite();
    applyQueryState();
    await refreshContext();
  });
</script>

<main class="page auth-page">
  <section class="stack auth-stack">
    <a class="brand-link" href="/">liberte.top</a>

    {#if authContext.authenticated}
      <section class="card auth-card-shell status-card">
        <div class="card-header">
          <h1>You're already signed in</h1>
          <p>{authContext.email || "Your account session is active in this browser."}</p>
        </div>

        <div class="actions compact-actions">
          <a class="button-link" href={preferredRedirectTarget}>{rewrite ? "Continue" : "Open profile"}</a>
          <a class="button-link secondary-link" href={logoutUrl}>Log out</a>
        </div>
      </section>
    {:else}
      <section class="card auth-card-shell">
        <div class="card-header">
          <h1>{modeTitle}</h1>
          <p>{modeSummary}</p>
        </div>

        {#if banner}
          <p class={`banner ${bannerTone}`}>{banner}</p>
        {/if}

        <form class="form-fields" on:submit|preventDefault={mode === "login" ? requestLogin : requestRegistration}>
          <label>
            Email address
            <input bind:value={email} type="email" autocomplete="email" placeholder="you@example.com" />
          </label>

          {#if mode === "register"}
            <label>
              Name
              <input bind:value={displayName} autocomplete="name" placeholder="Optional" />
            </label>
          {/if}

          <button type="submit" disabled={busy || !email}>
            {busy ? "Sending..." : primaryLabel}
          </button>

          {#if mode === "register" && registrationRequested}
            <button class="secondary" type="button" disabled={busy || !email} on:click={resendVerification}>Resend verification</button>
          {/if}
        </form>
      </section>

      <p class="switch-copy">
        {#if mode === "login"}
          New to Liberte?
          <button class="inline-action" type="button" on:click={() => (mode = "register")}>Create an account</button>
        {:else}
          Already have an account?
          <button class="inline-action" type="button" on:click={() => (mode = "login")}>Sign in</button>
        {/if}
      </p>

      <p class="support-copy">
        We email a one-time link to continue. {rewrite ? "After sign-in, you'll return to your original destination." : "If no destination is provided, you'll land on your profile page."}
      </p>
    {/if}
  </section>
</main>
