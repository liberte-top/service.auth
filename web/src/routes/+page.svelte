<script lang="ts">
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const mode = form?.mode ?? data.mode;
  const email = form?.email ?? data.email;
  const displayName = form?.displayName ?? "";
  const rewrite = form?.rewrite ?? data.rewrite;
  const banner = form?.message ?? (data.verified ? "Email verified. You can now request your sign-in link." : "");
  const bannerTone = form?.tone ?? (data.verified ? "success" : "info");
  const registrationRequested = form?.registrationRequested ?? false;
  const pageTitle = `${mode === "register" ? "Create account" : "Sign in"} - Liberte`;
  const description = mode === "register"
    ? "Create your Liberte account with a passwordless email verification flow."
    : "Sign in to Liberte with a secure one-time email link.";
</script>

<svelte:head>
  <title>{pageTitle}</title>
  <meta name="description" content={description} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<main class="page auth-page">
  <section class="stack auth-stack">
    <a class="brand-link" href="/">liberte.top</a>

    {#if data.authContext.authenticated}
      <section class="card auth-card-shell status-card">
        <div class="card-header">
          <h1>You're already signed in</h1>
          <p>{data.authContext.email || "Your account session is active in this browser."}</p>
        </div>

        <div class="actions compact-actions">
          <a class="button-link" href={rewrite || "/profile"}>{rewrite ? "Continue" : "Open profile"}</a>
          <form method="POST" action="/logout">
            <button class="secondary" type="submit">Log out</button>
          </form>
        </div>
      </section>
    {:else}
      <section class="card auth-card-shell">
        <div class="card-header">
          <h1>{mode === "register" ? "Create account" : "Sign in"}</h1>
          <p>{mode === "register" ? "Create your account with email, then verify it to continue." : "We will send a one-time link to your email."}</p>
        </div>

        {#if banner}
          <p class={`banner ${bannerTone}`} role={bannerTone === "error" ? "alert" : "status"} aria-live={bannerTone === "error" ? "assertive" : "polite"}>
            {banner}
          </p>
        {/if}

        <form class="form-fields" method="POST" action={mode === "register" ? "?/register" : "?/login"}>
          <input type="hidden" name="rewrite" value={rewrite} />

          <label>
            Email address
            <input name="email" type="email" autocomplete="email" autocapitalize="off" spellcheck="false" inputmode="email" required placeholder="you@example.com" value={email} />
          </label>

          {#if mode === "register"}
            <label>
              Name
              <input name="display_name" autocomplete="name" autocapitalize="words" spellcheck="false" placeholder="Optional" value={displayName} />
            </label>
          {/if}

          <button type="submit">{mode === "register" ? "Create account" : "Send sign-in link"}</button>
        </form>

        {#if mode === "register" && registrationRequested}
          <form class="actions compact-actions" method="POST" action="?/resend">
            <input type="hidden" name="email" value={email} />
            <input type="hidden" name="display_name" value={displayName} />
            <input type="hidden" name="rewrite" value={rewrite} />
            <button class="secondary" type="submit">Resend verification</button>
          </form>
        {/if}
      </section>

      <p class="switch-copy">
        {#if mode === "login"}
          New to Liberte?
          <a href={rewrite ? `/?mode=register&rewrite=${encodeURIComponent(rewrite)}` : "/?mode=register"}>Create an account</a>
        {:else}
          Already have an account?
          <a href={rewrite ? `/?mode=login&rewrite=${encodeURIComponent(rewrite)}` : "/?mode=login"}>Sign in</a>
        {/if}
      </p>

      <p class="support-copy">
        We email a one-time link to continue. {rewrite ? "After sign-in, you'll return to your original destination." : "If no destination is provided, you'll land on your profile page."}
      </p>
    {/if}
  </section>
</main>
