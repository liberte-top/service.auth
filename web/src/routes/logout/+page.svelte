<script lang="ts">
  import { Button, Card, LinkButton, SectionLabel } from "@liberte-top/components";
  import BrandLink from "$lib/layout/BrandLink.svelte";
  import DisplayTitle from "$lib/layout/DisplayTitle.svelte";
  import MutedText from "$lib/layout/MutedText.svelte";
  import PageRoot from "$lib/layout/PageRoot.svelte";
  import Panel from "$lib/layout/Panel.svelte";
  import Stack from "$lib/layout/Stack.svelte";
  import { translate } from "$lib/i18n/copy";
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const signedOut = form?.success ?? false;
</script>

<svelte:head>
  <title>{signedOut ? `${translate(data.language, "auth.logout.successTitle")} - Liberte` : translate(data.language, "auth.logout.title")}</title>
  <meta name="description" content={translate(data.language, "auth.logout.confirmBody")} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<PageRoot>
  <Panel>
    <BrandLink>liberte.top</BrandLink>

    <Card>
      <Stack gap="12px">
        <SectionLabel>{translate(data.language, "auth.logout.label")}</SectionLabel>
        <DisplayTitle>{signedOut ? translate(data.language, "auth.logout.successTitle") : translate(data.language, "auth.logout.confirmTitle")}</DisplayTitle>
        <MutedText>{signedOut ? translate(data.language, "auth.logout.successBody") : translate(data.language, "auth.logout.confirmBody")}</MutedText>

        {#if signedOut}
          <Stack gap="12px" marginTop="4px">
            <LinkButton block href="/login">{translate(data.language, "auth.logout.back")}</LinkButton>
          </Stack>
        {:else}
          <form class="logout-actions" method="POST">
            <Button block type="submit">{translate(data.language, "auth.common.logout")}</Button>
            <LinkButton variant="secondary" block href="/login">{translate(data.language, "auth.logout.cancel")}</LinkButton>
          </form>
        {/if}
      </Stack>
    </Card>
  </Panel>
</PageRoot>

<style>
  .logout-actions {
    display: grid;
    gap: 12px;
    margin-top: 4px;
  }

</style>
