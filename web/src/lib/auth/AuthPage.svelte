<script lang="ts">
  type Mode = "login" | "register";

  type AuthContext = {
    authenticated?: boolean;
    email?: string | null;
  };

  type FormState = {
    mode?: Mode;
    message?: string;
    tone?: "success" | "error" | "info";
    email?: string;
    displayName?: string;
    rewrite?: string;
    registrationRequested?: boolean;
  } | null | undefined;

  let {
    mode,
    authContext,
    email,
    rewrite,
    verified,
    canonical,
    form,
  }: {
    mode: Mode;
    authContext: AuthContext;
    email: string;
    rewrite: string;
    verified?: boolean;
    canonical: string;
    form?: FormState;
  } = $props();

  const currentMode = $derived((form?.mode ?? mode) as Mode);
  const currentEmail = $derived(form?.email ?? email);
  const displayName = $derived(form?.displayName ?? "");
  const currentRewrite = $derived(form?.rewrite ?? rewrite);
  const banner = $derived(form?.message ?? (verified ? "Email verified. You can now request your sign-in link." : ""));
  const bannerTone = $derived((form?.tone ?? (verified ? "success" : "info")) as "success" | "error" | "info");
  const registrationRequested = $derived(form?.registrationRequested ?? false);
  const pageTitle = $derived(`${currentMode === "register" ? "Create account" : "Sign in"} - Liberte`);
  const description = $derived(currentMode === "register"
    ? "Create your Liberte account with a passwordless email verification flow."
    : "Sign in to Liberte with a secure one-time email link.");
  const alternateHref = $derived(
    currentMode === "login"
      ? currentRewrite
        ? `/register?rewrite=${encodeURIComponent(currentRewrite)}`
        : "/register"
      : currentRewrite
        ? `/login?rewrite=${encodeURIComponent(currentRewrite)}`
        : "/login"
  );
</script>

<svelte:head>
  <title>{pageTitle}</title>
  <meta name="description" content={description} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={canonical} />
</svelte:head>

<main class="page auth-page">
  <section class="stack auth-stack">
    <a class="brand-link" href="/login">liberte.top</a>

    {#if authContext.authenticated}
      <section class="card auth-card-shell status-card">
        <div class="card-header">
          <h1>You're already signed in</h1>
          <p>{authContext.email || "Your account session is active in this browser."}</p>
        </div>

        <div class="actions compact-actions">
          <a class="button-link" href={currentRewrite || "/profile"}>{currentRewrite ? "Continue" : "Open profile"}</a>
          <form method="POST" action="/logout">
            <button class="secondary" type="submit">Log out</button>
          </form>
        </div>
      </section>
    {:else}
      <section class="card auth-card-shell">
        <div class="card-header">
          <h1>{currentMode === "register" ? "Create account" : "Sign in"}</h1>
          <p>{currentMode === "register" ? "Create your account with email, then verify it to continue." : "We will send a one-time link to your email."}</p>
        </div>

        {#if banner}
          <p class={`banner ${bannerTone}`} role={bannerTone === "error" ? "alert" : "status"} aria-live={bannerTone === "error" ? "assertive" : "polite"}>
            {banner}
          </p>
        {/if}

        <form class="form-fields" method="POST" action={currentMode === "register" ? "?/register" : "?/login"}>
          <input type="hidden" name="rewrite" value={currentRewrite} />

          <label>
            Email address
            <input name="email" type="email" autocomplete="email" autocapitalize="off" spellcheck="false" inputmode="email" required placeholder="you@example.com" value={currentEmail} />
          </label>

          {#if currentMode === "register"}
            <label>
              Name
              <input name="display_name" autocomplete="name" autocapitalize="words" spellcheck="false" placeholder="Optional" value={displayName} />
            </label>
          {/if}

          <button type="submit">{currentMode === "register" ? "Create account" : "Send sign-in link"}</button>
        </form>

        {#if currentMode === "register" && registrationRequested}
          <form class="actions compact-actions" method="POST" action="?/resend">
            <input type="hidden" name="email" value={currentEmail} />
            <input type="hidden" name="display_name" value={displayName} />
            <input type="hidden" name="rewrite" value={currentRewrite} />
            <button class="secondary" type="submit">Resend verification</button>
          </form>
        {/if}
      </section>

      <p class="switch-copy">
        {#if currentMode === "login"}
          New to Liberte?
          <a href={alternateHref}>Create an account</a>
        {:else}
          Already have an account?
          <a href={alternateHref}>Sign in</a>
        {/if}
      </p>

      <p class="support-copy">
        We email a one-time link to continue. {currentRewrite ? "After sign-in, you'll return to your original destination." : "If no destination is provided, you'll land on your profile page."}
      </p>
    {/if}
  </section>
</main>
