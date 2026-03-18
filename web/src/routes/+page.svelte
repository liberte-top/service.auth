<script lang="ts">
  import { LinkButton, Card, CardHeader } from "@liberte-top/components";
  import { SectionLabel } from "@liberte-top/components";
  import BrandLink from "$lib/layout/BrandLink.svelte";
  import DisplayTitle from "$lib/layout/DisplayTitle.svelte";
  import MutedText from "$lib/layout/MutedText.svelte";
  import PageRoot from "$lib/layout/PageRoot.svelte";
  import Panel from "$lib/layout/Panel.svelte";

  let { data }: { data: { loginHref: string } } = $props();
</script>

<PageRoot>
  <Panel>
    <BrandLink>liberte.top</BrandLink>

    <Card>
      <CardHeader>
        <SectionLabel>Auth</SectionLabel>
        <DisplayTitle>Continue to sign in</DisplayTitle>
        <MutedText>This entrypoint stays lightweight so health checks and direct opens do not depend on redirect-only server behavior.</MutedText>
      </CardHeader>

      <LinkButton block href={data.loginHref}>Go to login</LinkButton>
    </Card>
  </Panel>
</PageRoot>
