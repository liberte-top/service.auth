<script lang="ts">
  import { Alert, Button, Card, CardHeader, Field, Input, LinkButton } from "@liberte-top/components";
  import { translate } from "$lib/i18n/copy";
  import BrandLink from "$lib/layout/BrandLink.svelte";
  import CenteredMutedText from "$lib/layout/CenteredMutedText.svelte";
  import DisplayTitle from "$lib/layout/DisplayTitle.svelte";
  import MutedText from "$lib/layout/MutedText.svelte";
  import PageRoot from "$lib/layout/PageRoot.svelte";
  import Panel from "$lib/layout/Panel.svelte";
  import Stack from "$lib/layout/Stack.svelte";
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const banner = $derived(form?.message ?? "");
  const bannerTone = $derived((form?.tone ?? "info") as "success" | "error" | "info");
  const registrationRequested = $derived(form?.registrationRequested ?? false);
  const currentRewrite = $derived(form?.rewrite ?? data.rewrite);
  const currentEmail = $derived(form?.email ?? data.email);
  const currentDisplayName = $derived(form?.displayName ?? "");
</script>

<svelte:head>
  <title>{translate(data.language, "auth.register.title")} - Liberte</title>
  <meta name="description" content="Create your Liberte account with a passwordless email verification flow." />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<PageRoot>
  <Panel>
    <BrandLink>liberte.top</BrandLink>

    {#if data.authContext.authenticated}
      <Card>
        <Stack gap="4px">
          <CardHeader>
            <DisplayTitle>{translate(data.language, "auth.common.signedIn")}</DisplayTitle>
            <MutedText>{data.authContext.email || translate(data.language, "auth.common.activeSession")}</MutedText>
          </CardHeader>

          <Stack gap="12px" marginTop="4px">
            <LinkButton block href={currentRewrite || "/profile"}>{currentRewrite ? translate(data.language, "auth.common.continue") : translate(data.language, "auth.common.openProfile")}</LinkButton>
            <form method="POST" action="/logout">
              <Button variant="secondary" block type="submit">{translate(data.language, "auth.common.logout")}</Button>
            </form>
          </Stack>
        </Stack>
      </Card>
    {:else}
      <Card>
        <CardHeader>
          <DisplayTitle>{translate(data.language, "auth.register.title")}</DisplayTitle>
          <MutedText>{translate(data.language, "auth.register.summary")}</MutedText>
        </CardHeader>

        {#if banner}
          <Alert class="banner" tone={bannerTone} role={bannerTone === "error" ? "alert" : "status"} aria-live={bannerTone === "error" ? "assertive" : "polite"}>
            {banner}
          </Alert>
        {/if}

        <form class="form-fields" method="POST">
          <input type="hidden" name="rewrite" value={currentRewrite} />
          <input type="hidden" name="intent" value="register" />

          <Field label={translate(data.language, "auth.common.emailLabel")} required>
            <Input name="email" type="email" autocomplete="email" autocapitalize="off" spellcheck="false" inputmode="email" required placeholder="you@example.com" value={currentEmail} />
          </Field>

          <Field label={translate(data.language, "auth.common.nameLabel")} optional={translate(data.language, "auth.common.namePlaceholder")}>
            <Input name="display_name" autocomplete="name" autocapitalize="words" spellcheck="false" placeholder={translate(data.language, "auth.common.namePlaceholder")} value={currentDisplayName} />
          </Field>

          <Button block type="submit">{translate(data.language, "auth.register.submit")}</Button>
        </form>

        {#if registrationRequested}
          <form class="resend-form" method="POST">
            <input type="hidden" name="email" value={currentEmail} />
            <input type="hidden" name="display_name" value={currentDisplayName} />
            <input type="hidden" name="rewrite" value={currentRewrite} />
            <input type="hidden" name="intent" value="resend" />
            <Button variant="secondary" block type="submit">{translate(data.language, "auth.register.resend")}</Button>
          </form>
        {/if}
      </Card>

      <CenteredMutedText>
        {translate(data.language, "auth.register.switchPrompt")}
        <a href={currentRewrite ? `/login?rewrite=${encodeURIComponent(currentRewrite)}` : "/login"}>{translate(data.language, "auth.register.switchAction")}</a>
      </CenteredMutedText>

      <CenteredMutedText>
        {currentRewrite ? translate(data.language, "auth.common.supportWithRewrite") : translate(data.language, "auth.common.supportDefault")}
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
