<script lang="ts">
  import { translate } from "$lib/i18n/copy";
  import type { LiberteLanguage } from "$lib/i18n/shared";

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
    primaryAction,
    language,
  }: {
    mode: Mode;
    authContext: AuthContext;
    email: string;
    rewrite: string;
    verified?: boolean;
    canonical: string;
    form?: FormState;
    primaryAction: string;
    language: LiberteLanguage;
  } = $props();

  const currentMode = $derived((form?.mode ?? mode) as Mode);
  const currentEmail = $derived(form?.email ?? email);
  const displayName = $derived(form?.displayName ?? "");
  const currentRewrite = $derived(form?.rewrite ?? rewrite);
  const banner = $derived(form?.message ?? (verified ? "Email verified. You can now request your sign-in link." : ""));
  const bannerTone = $derived((form?.tone ?? (verified ? "success" : "info")) as "success" | "error" | "info");
  const registrationRequested = $derived(form?.registrationRequested ?? false);
  const pageTitle = $derived(`${translate(language, currentMode === "register" ? "auth.register.title" : "auth.login.title")} - Liberte`);
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
          <h1>{translate(language, "auth.common.signedIn")}</h1>
          <p>{authContext.email || translate(language, "auth.common.activeSession")}</p>
        </div>

        <div class="actions compact-actions">
          <a class="button-link" href={currentRewrite || "/profile"}>{currentRewrite ? translate(language, "auth.common.continue") : translate(language, "auth.common.openProfile")}</a>
          <form method="POST" action="/logout">
            <button class="secondary" type="submit">{translate(language, "auth.common.logout")}</button>
          </form>
        </div>
      </section>
    {:else}
      <section class="card auth-card-shell">
        <div class="card-header">
          <h1>{translate(language, currentMode === "register" ? "auth.register.title" : "auth.login.title")}</h1>
          <p>{translate(language, currentMode === "register" ? "auth.register.summary" : "auth.login.summary")}</p>
        </div>

        {#if banner}
          <p class={`banner ${bannerTone}`} role={bannerTone === "error" ? "alert" : "status"} aria-live={bannerTone === "error" ? "assertive" : "polite"}>
            {banner}
          </p>
        {/if}

        <form class="form-fields" method="POST" action={primaryAction}>
          <input type="hidden" name="rewrite" value={currentRewrite} />
          <input type="hidden" name="intent" value="register" />

          <label>
            {translate(language, "auth.common.emailLabel")}
            <input name="email" type="email" autocomplete="email" autocapitalize="off" spellcheck="false" inputmode="email" required placeholder="you@example.com" value={currentEmail} />
          </label>

          {#if currentMode === "register"}
            <label>
              {translate(language, "auth.common.nameLabel")}
              <input name="display_name" autocomplete="name" autocapitalize="words" spellcheck="false" placeholder={translate(language, "auth.common.namePlaceholder")} value={displayName} />
            </label>
          {/if}

          <button type="submit">{translate(language, currentMode === "register" ? "auth.register.submit" : "auth.login.submit")}</button>
        </form>

        {#if currentMode === "register" && registrationRequested}
          <form class="actions compact-actions" method="POST" action={primaryAction}>
            <input type="hidden" name="email" value={currentEmail} />
            <input type="hidden" name="display_name" value={displayName} />
            <input type="hidden" name="rewrite" value={currentRewrite} />
            <input type="hidden" name="intent" value="resend" />
            <button class="secondary" type="submit">{translate(language, "auth.register.resend")}</button>
          </form>
        {/if}
      </section>

      <p class="switch-copy">
        {#if currentMode === "login"}
          {translate(language, "auth.login.switchPrompt")}
          <a href={alternateHref}>{translate(language, "auth.login.switchAction")}</a>
        {:else}
          {translate(language, "auth.register.switchPrompt")}
          <a href={alternateHref}>{translate(language, "auth.register.switchAction")}</a>
        {/if}
      </p>

      <p class="support-copy">
        {currentRewrite ? translate(language, "auth.common.supportWithRewrite") : translate(language, "auth.common.supportDefault")}
      </p>
    {/if}
  </section>
</main>
