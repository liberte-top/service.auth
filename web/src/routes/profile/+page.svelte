<script lang="ts">
  import { Button, Card, SectionLabel } from "@liberte-top/components";
  import BrandLink from "$lib/layout/BrandLink.svelte";
  import DisplayTitle from "$lib/layout/DisplayTitle.svelte";
  import MutedText from "$lib/layout/MutedText.svelte";
  import PageRoot from "$lib/layout/PageRoot.svelte";
  import Panel from "$lib/layout/Panel.svelte";
  import Stack from "$lib/layout/Stack.svelte";
  import { translate } from "$lib/i18n/copy";
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<svelte:head>
  <title>{translate(data.language, "auth.profile.title")} - Liberte</title>
  <meta name="description" content={translate(data.language, "auth.profile.description")} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<PageRoot>
  <Panel>
    <BrandLink>liberte.top</BrandLink>

    <Card>
      <Stack gap="12px">
        <SectionLabel>{translate(data.language, "auth.profile.title")}</SectionLabel>
        <DisplayTitle>{data.email}</DisplayTitle>
        <MutedText>{translate(data.language, "auth.profile.subtitle")}</MutedText>

        <form class="actions compact-actions" method="POST" action="/logout">
          <Button variant="secondary" block type="submit">{translate(data.language, "auth.common.logout")}</Button>
        </form>
      </Stack>
    </Card>
  </Panel>
</PageRoot>

<style>
  .actions {
    display: grid;
    gap: 12px;
  }

  .compact-actions {
    margin-top: 4px;
  }
</style>
