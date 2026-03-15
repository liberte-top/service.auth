<script lang="ts">
  import { Alert, Button, Card, CardHeader, Field, Input, LinkButton } from "@liberte-top/components";
  import BrandLink from "$lib/layout/BrandLink.svelte";
  import CenteredMutedText from "$lib/layout/CenteredMutedText.svelte";
  import PageRoot from "$lib/layout/PageRoot.svelte";
  import Panel from "$lib/layout/Panel.svelte";
  import Stack from "$lib/layout/Stack.svelte";
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

<PageRoot>
  <Panel>
    <BrandLink>liberte.top</BrandLink>

    {#if authContext.authenticated}
      <Card>
        <Stack gap="4px">
          <CardHeader>
            <h1>{translate(language, "auth.common.signedIn")}</h1>
            <p>{authContext.email || translate(language, "auth.common.activeSession")}</p>
          </CardHeader>

          <Stack gap="12px" marginTop="4px">
            <LinkButton block href={currentRewrite || "/profile"}>{currentRewrite ? translate(language, "auth.common.continue") : translate(language, "auth.common.openProfile")}</LinkButton>
            <form method="POST" action="/logout">
              <Button variant="secondary" block type="submit">{translate(language, "auth.common.logout")}</Button>
            </form>
          </Stack>
        </Stack>
      </Card>
    {:else}
      <Card>
        <CardHeader>
          <h1>{translate(language, currentMode === "register" ? "auth.register.title" : "auth.login.title")}</h1>
          <p>{translate(language, currentMode === "register" ? "auth.register.summary" : "auth.login.summary")}</p>
        </CardHeader>

        {#if banner}
          <Alert class="banner" tone={bannerTone} role={bannerTone === "error" ? "alert" : "status"} aria-live={bannerTone === "error" ? "assertive" : "polite"}>
            {banner}
          </Alert>
        {/if}

        <form class="form-fields" method="POST" action={primaryAction}>
          <input type="hidden" name="rewrite" value={currentRewrite} />
          <input type="hidden" name="intent" value="register" />

          <Field label={translate(language, "auth.common.emailLabel")} required>
            <Input name="email" type="email" autocomplete="email" autocapitalize="off" spellcheck="false" inputmode="email" required placeholder="you@example.com" value={currentEmail} />
          </Field>

          {#if currentMode === "register"}
            <Field label={translate(language, "auth.common.nameLabel")} optional={translate(language, "auth.common.namePlaceholder")}>
              <Input name="display_name" autocomplete="name" autocapitalize="words" spellcheck="false" placeholder={translate(language, "auth.common.namePlaceholder")} value={displayName} />
            </Field>
          {/if}

          <Button block type="submit">{translate(language, currentMode === "register" ? "auth.register.submit" : "auth.login.submit")}</Button>
        </form>

        {#if currentMode === "register" && registrationRequested}
          <form class="resend-form" method="POST" action={primaryAction}>
            <input type="hidden" name="email" value={currentEmail} />
            <input type="hidden" name="display_name" value={displayName} />
            <input type="hidden" name="rewrite" value={currentRewrite} />
            <input type="hidden" name="intent" value="resend" />
            <Button variant="secondary" block type="submit">{translate(language, "auth.register.resend")}</Button>
          </form>
        {/if}
      </Card>

      <CenteredMutedText>
        {#if currentMode === "login"}
          {translate(language, "auth.login.switchPrompt")}
          <a href={alternateHref}>{translate(language, "auth.login.switchAction")}</a>
        {:else}
          {translate(language, "auth.register.switchPrompt")}
          <a href={alternateHref}>{translate(language, "auth.register.switchAction")}</a>
        {/if}
      </CenteredMutedText>

      <CenteredMutedText>
        {currentRewrite ? translate(language, "auth.common.supportWithRewrite") : translate(language, "auth.common.supportDefault")}
      </CenteredMutedText>
    {/if}
  </Panel>
</PageRoot>

<style>
  .form-fields,
  .resend-form {
    display: grid;
  }

  .form-fields {
    gap: 16px;
  }

  .resend-form {
    gap: 12px;
    margin-top: 4px;
  }

  :global(.banner) {
    margin-bottom: 16px;
  }
</style>
