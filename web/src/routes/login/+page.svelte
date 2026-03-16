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

  const banner = $derived(form?.message ?? (data.verified ? translate(data.language, "auth.login.verified") : ""));
  const bannerTone = $derived((form?.tone ?? (data.verified ? "success" : "info")) as "success" | "error" | "info");
</script>

<svelte:head>
  <title>{translate(data.language, "auth.login.title")} - Liberte</title>
  <meta name="description" content="Sign in to Liberte with a secure one-time email link." />
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
            <LinkButton block href={data.rewrite || "/profile"}>{data.rewrite ? translate(data.language, "auth.common.continue") : translate(data.language, "auth.common.openProfile")}</LinkButton>
            <form method="POST" action="/logout">
              <Button variant="secondary" block type="submit">{translate(data.language, "auth.common.logout")}</Button>
            </form>
          </Stack>
        </Stack>
      </Card>
    {:else}
      <Card>
        <CardHeader>
          <DisplayTitle>{translate(data.language, "auth.login.title")}</DisplayTitle>
          <MutedText>{translate(data.language, "auth.login.summary")}</MutedText>
        </CardHeader>

        {#if banner}
          <Alert class="banner" tone={bannerTone} role={bannerTone === "error" ? "alert" : "status"} aria-live={bannerTone === "error" ? "assertive" : "polite"}>
            {banner}
          </Alert>
        {/if}

        <form class="form-fields" method="POST">
          <input type="hidden" name="rewrite" value={form?.rewrite ?? data.rewrite} />

          <Field label={translate(data.language, "auth.common.emailLabel")} required>
            <Input name="email" type="email" autocomplete="email" autocapitalize="off" spellcheck="false" inputmode="email" required placeholder="you@example.com" value={form?.email ?? data.email} />
          </Field>

          <Button block type="submit">{translate(data.language, "auth.login.submit")}</Button>
        </form>
      </Card>

      <CenteredMutedText>
        {translate(data.language, "auth.login.switchPrompt")}
        <a href={data.rewrite ? `/register?rewrite=${encodeURIComponent(data.rewrite)}` : "/register"}>{translate(data.language, "auth.login.switchAction")}</a>
      </CenteredMutedText>

      <CenteredMutedText>
        {data.rewrite ? translate(data.language, "auth.common.supportWithRewrite") : translate(data.language, "auth.common.supportDefault")}
      </CenteredMutedText>
    {/if}
  </Panel>
</PageRoot>

<style>
  .form-fields {
    display: grid;
    gap: 16px;
  }

  :global(.banner) {
    margin-bottom: 16px;
  }
</style>
