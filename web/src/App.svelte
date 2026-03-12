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
  let authContext: AuthContext = { authenticated: false, subject: null, auth_type: null, scopes: [] };

  $: modeTitle = mode === "login" ? "Sign in with email" : "Create your account";
  $: modeSummary = mode === "login"
    ? "We email a one-time sign-in link so you can continue safely without a password."
    : "Register once, verify your inbox, and we will use the same email-link flow for future sign-ins.";
  $: primaryLabel = mode === "login" ? "Send sign-in link" : "Send verification email";

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

  function signedInSummary() {
    return authContext.subject ? `${authContext.subject.slice(0, 8)}...` : "anonymous";
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
      authContext = context;
      if (context.authenticated) {
        bannerTone = "success";
        banner = rewrite
          ? "You already have an active session. Continue to your destination whenever you are ready."
          : "You already have an active session. Continue to your profile whenever you are ready.";
      }
    } catch {
      bannerTone = "error";
      banner = "Auth context is currently unavailable.";
    }
  });
</script>

<main class="shell">
  <section class="hero-panel">
    <section class="hero card">
      <p class="eyebrow">liberte.top auth</p>
      <h1>Simple email auth for every app entrypoint.</h1>
      <p class="lede">Register, verify, and sign in from one place. We keep the destination context so you land where you intended.</p>

      <div class="hero-points">
        <div>
          <strong>Passwordless</strong>
          <p>Every sign-in uses a one-time email link instead of stored passwords.</p>
        </div>
        <div>
          <strong>Same session domain</strong>
          <p>Once you complete auth, the session follows you across <code>.liberte.top</code>.</p>
        </div>
        <div>
          <strong>Redirect priority</strong>
          <p>After auth we honor <code>rewrite</code> first, then fall back to your profile.</p>
        </div>
      </div>
    </section>

    <section class="card destination-card">
      <p class="eyebrow">Current destination</p>
      {#if rewrite}
        <p class="destination-copy">Your session will continue to:</p>
        <p class="destination-chip"><code>{rewrite}</code></p>
      {:else}
        <p class="destination-copy">No rewrite target was provided. Successful login falls back to your profile page.</p>
      {/if}
    </section>

    <section class="card session-card">
      <p class="eyebrow">Current session</p>
      <div class="session-status-row">
        <strong>{authContext.authenticated ? "Signed in" : "Signed out"}</strong>
        <span class:online={authContext.authenticated} class="session-pill">{authContext.authenticated ? "active" : "idle"}</span>
      </div>

      {#if authContext.authenticated}
        <dl class="session-grid">
          <div>
            <dt>Subject</dt>
            <dd><code>{signedInSummary()}</code></dd>
          </div>
          <div>
            <dt>Auth type</dt>
            <dd>{authContext.auth_type}</dd>
          </div>
          <div>
            <dt>Scopes</dt>
            <dd>{authContext.scopes?.join(", ") || "none"}</dd>
          </div>
        </dl>
        <div class="actions session-actions">
          <a class="button-link" href={preferredRedirectTarget()}>{rewrite ? "Continue" : "Open profile"}</a>
          <a class="button-link secondary-link" href="https://smoke.liberte.top/">Open smoke app</a>
        </div>
      {:else}
        <p class="destination-copy">No active auth session is present in this browser yet.</p>
      {/if}
    </section>
  </section>

  <section class="card auth-card">
    <div class="auth-shell">
      <div class="auth-copy">
        <p class="eyebrow">Account access</p>
        <h2>{modeTitle}</h2>
        <p class="auth-summary">{modeSummary}</p>

        <ol class="flow-list">
          <li>Enter your email address.</li>
          <li>{mode === "register" ? "Verify the email we send you." : "Open the one-time sign-in link."}</li>
          <li>We route you to <code>rewrite</code> first, otherwise to your profile.</li>
        </ol>
      </div>

      <div class="auth-form-block">
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

        <div class="actions auth-actions">
          <button disabled={busy || !email} on:click={mode === "login" ? requestLogin : requestRegistration}>
            {busy ? "Sending..." : primaryLabel}
          </button>
          {#if mode === "register"}
            <button class="secondary" disabled={busy || !email} on:click={resendVerification}>Resend verification</button>
          {/if}
        </div>

        <p class="mode-helper">
          {#if mode === "login"}
            Need a new account?
            <button class="inline-action" on:click={() => (mode = "register")}>Register instead</button>
          {:else}
            Already verified?
            <button class="inline-action" on:click={() => (mode = "login")}>Send a sign-in link</button>
          {/if}
        </p>
      </div>
    </div>
  </section>
</main>
