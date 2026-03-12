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
  let contextStatus = "checking";
  let lastContextCheck = "never";
  let lastContextHttpStatus = "n/a";
  let lastContextPayload = "not loaded";

  $: modeTitle = mode === "login" ? "Sign in with email" : "Create your account";
  $: modeSummary = mode === "login"
    ? "We email a one-time sign-in link so you can continue safely without a password."
    : "Register once, verify your inbox, and we will use the same email-link flow for future sign-ins.";
  $: primaryLabel = mode === "login" ? "Send sign-in link" : "Send verification email";
  $: destinationLabel = rewrite || profileUrl;

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
    const response = await apiClient.get<AuthContext>("/api/v1/context", { validateStatus: () => true });
    lastContextHttpStatus = String(response.status);
    lastContextPayload = JSON.stringify(response.data, null, 2);
    if (response.status >= 200 && response.status < 300) {
      return response.data;
    }
    throw new Error(`context request failed with HTTP ${response.status}`);
  }

  async function refreshContext(showBanner = false) {
    contextStatus = "checking";
    try {
      const context = await loadContext();
      authContext = context;
      contextStatus = context.authenticated ? "signed-in" : "signed-out";
      lastContextCheck = new Date().toLocaleTimeString();
      if (showBanner) {
        bannerTone = "info";
        banner = context.authenticated
          ? "Session refreshed. You are still signed in."
          : "Session refreshed. No active auth session is present.";
      }
      return context;
    } catch {
      contextStatus = "unavailable";
      lastContextCheck = new Date().toLocaleTimeString();
      bannerTone = "error";
      banner = "Auth context is currently unavailable.";
      return null;
    }
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

    const context = await refreshContext();
    if (context?.authenticated) {
      bannerTone = "success";
      banner = rewrite
        ? "You already have an active session. Continue to your destination whenever you are ready."
        : "You already have an active session. Continue to your profile whenever you are ready.";
    }
  });
</script>

<main class="shell">
  <section class="hero-layout">
    <section class="hero hero-card card">
      <div class="hero-intro">
        <p class="eyebrow">liberte.top auth</p>
        <h1>A clean sign-in flow for the whole liberte.top network.</h1>
        <p class="lede">Register, verify, and sign in from one destination. We preserve the app you originally requested, then return you there with an active session.</p>
      </div>

      <div class="hero-highlights">
        <div>
          <span>Zero passwords</span>
          <strong>One-time email links only</strong>
          <p>We never ask you to create or remember a password for everyday access.</p>
        </div>
        <div>
          <span>Cross-site session</span>
          <strong>One session across <code>.liberte.top</code></strong>
          <p>Complete auth once and continue through linked apps without starting over.</p>
        </div>
        <div>
          <span>Routing logic</span>
          <strong>Redirects honor <code>rewrite</code> first</strong>
          <p>If a destination is provided, we send you back there after verification or login.</p>
        </div>
      </div>

      <div class="hero-footnote">
        <div class="metric-chip">
          <span class="metric-label">Default destination</span>
          <strong>Profile page</strong>
        </div>
        <div class="metric-chip">
          <span class="metric-label">Verification step</span>
          <strong>Email-first onboarding</strong>
        </div>
        <div class="metric-chip">
          <span class="metric-label">Session state</span>
          <strong>{authContext.authenticated ? "Active in this browser" : "Waiting for sign-in"}</strong>
        </div>
      </div>
    </section>

    <aside class="hero-sidebar">
      <section class="card destination-card accent-card">
        <p class="eyebrow">Current destination</p>
        <p class="destination-copy">
          {#if rewrite}
            You will return directly to the requested page after the auth flow completes.
          {:else}
            No rewrite target was provided, so the default landing page will be your profile.
          {/if}
        </p>
        <p class="destination-chip"><code>{destinationLabel}</code></p>
      </section>

      <section class="card info-card">
        <p class="eyebrow">What to expect</p>
        <ul class="info-list">
          <li>Register once with your email.</li>
          <li>Open the verification or sign-in link from your inbox.</li>
          <li>Return with an authenticated same-domain session.</li>
        </ul>
      </section>
    </aside>
  </section>

  <section class="hero-panel">
    <section class="card session-card">
      <p class="eyebrow">Current session</p>
      <div class="session-status-row">
        <strong>{authContext.authenticated ? "Signed in" : "Signed out"}</strong>
        <span class:online={authContext.authenticated} class="session-pill">{authContext.authenticated ? "active" : "idle"}</span>
      </div>
      <p class="status-meta">
        Context status: <code>{contextStatus}</code> · last checked <code>{lastContextCheck}</code>
      </p>
      <p class="status-meta">
        HTTP status: <code>{lastContextHttpStatus}</code>
      </p>

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
          <button class="secondary" on:click={() => refreshContext(true)}>Refresh session</button>
        </div>
      {:else}
        <p class="destination-copy">No active auth session is present in this browser yet.</p>
        <div class="actions session-actions">
          <button class="secondary" on:click={() => refreshContext(true)}>Refresh session</button>
        </div>
      {/if}

      <details class="diagnostics-block">
        <summary>Raw auth context payload</summary>
        <pre>{lastContextPayload}</pre>
      </details>
    </section>

    <section class="card support-card">
      <p class="eyebrow">Why this flow feels normal</p>
      <div class="support-grid">
        <div>
          <strong>Familiar inbox journey</strong>
          <p>Every major step is guided by a clear email action instead of a custom password reset maze.</p>
        </div>
        <div>
          <strong>Single entrypoint</strong>
          <p>Registration, verification, and sign-in all happen from the same auth home.</p>
        </div>
        <div>
          <strong>Destination-aware</strong>
          <p>Your original app URL stays attached to the flow so you can pick up where you left off.</p>
        </div>
        <div>
          <strong>Visible session state</strong>
          <p>This page shows whether the browser already holds an auth session before you submit anything.</p>
        </div>
      </div>
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

        <div class="trust-strip">
          <div>
            <span class="trust-label">Mode</span>
            <strong>{mode === "login" ? "Returning user" : "New registration"}</strong>
          </div>
          <div>
            <span class="trust-label">Delivery</span>
            <strong>Email link</strong>
          </div>
          <div>
            <span class="trust-label">Fallback</span>
            <strong>Profile page</strong>
          </div>
        </div>
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
            <input bind:value={displayName} autocomplete="name" placeholder="Optional, shown on your account" />
          </label>
        {/if}

        <div class="field-note">
          <strong>Destination after auth</strong>
          <span>{rewrite ? "Your current request will send you back to the provided rewrite URL." : "No rewrite target is present, so successful auth opens your profile page."}</span>
        </div>

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

        <p class="helper-copy">Emails usually arrive within a minute. If you are registering and already requested a link, use resend verification to issue a fresh one.</p>
      </div>
    </div>
  </section>

  <section class="card faq-card">
    <div>
      <p class="eyebrow">Need to know</p>
      <h2>Common questions before you continue</h2>
    </div>
    <div class="faq-grid">
      <article>
        <strong>What if I already have a session?</strong>
        <p>This page detects the current browser session and lets you continue without starting the flow again.</p>
      </article>
      <article>
        <strong>What if I do not see the email?</strong>
        <p>Check spam or promotions, then resend the verification mail or request another sign-in link.</p>
      </article>
      <article>
        <strong>Where do I land after login?</strong>
        <p>If <code>rewrite</code> exists we use it first; otherwise we send you to your profile page.</p>
      </article>
    </div>
  </section>
</main>
