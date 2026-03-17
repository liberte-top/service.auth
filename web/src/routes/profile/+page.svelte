<script lang="ts">
  import { Alert, Button, Card, CardHeader, Field, Input, LinkButton, SectionLabel } from "@liberte-top/components";
  import { translate } from "$lib/i18n/copy";
  import BrandLink from "$lib/layout/BrandLink.svelte";
  import DisplayTitle from "$lib/layout/DisplayTitle.svelte";
  import MutedText from "$lib/layout/MutedText.svelte";
  import PageRoot from "$lib/layout/PageRoot.svelte";
  import Panel from "$lib/layout/Panel.svelte";
  import Stack from "$lib/layout/Stack.svelte";
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const profileMessage = $derived(form?.kind === "profile" ? form.message : "");
  const profileTone = $derived((form?.kind === "profile" ? form.tone : "info") as "success" | "error" | "info");
  const tokenMessage = $derived(form?.kind === "token-create" || form?.kind === "token-revoke" ? form.message : "");
  const tokenTone = $derived(((form?.kind === "token-create" || form?.kind === "token-revoke") ? form.tone : "info") as "success" | "error" | "info");
  const profileDisplayName = $derived(form?.kind === "profile" ? form.displayName : (data.profile.display_name ?? ""));
  const tokenName = $derived(form?.kind === "token-create" ? form.name : "");
  const tokenExpiry = $derived(form?.kind === "token-create" ? form.expiresAt : "");
  const createdToken = $derived(form?.kind === "token-create" ? form.createdToken : undefined);
  const scopeCatalog = $derived(new Map(data.scopeCatalog.map((scope) => [scope.name, scope])));

  function formatDateTime(value: string | null | undefined) {
    if (!value) {
      return "-";
    }

    return new Intl.DateTimeFormat(data.language, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    }).format(new Date(value));
  }
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
        <CardHeader>
          <SectionLabel>{translate(data.language, "auth.profile.eyebrow")}</SectionLabel>
          <DisplayTitle>{data.profile.display_name || data.profile.email || translate(data.language, "auth.profile.fallbackTitle")}</DisplayTitle>
          <MutedText>{translate(data.language, "auth.profile.subtitle")}</MutedText>
        </CardHeader>

        <div class="identity-grid">
          <div class="identity-item">
            <span class="identity-label">{translate(data.language, "auth.profile.emailLabel")}</span>
            <strong>{data.profile.email || "-"}</strong>
          </div>
          <div class="identity-item">
            <span class="identity-label">{translate(data.language, "auth.profile.emailStatusLabel")}</span>
            <strong>{data.profile.email_verified ? translate(data.language, "auth.profile.emailVerified") : translate(data.language, "auth.profile.emailUnverified")}</strong>
          </div>
          <div class="identity-item">
            <span class="identity-label">{translate(data.language, "auth.profile.authTypeLabel")}</span>
                <strong>{data.authContext.authType || "-"}</strong>
          </div>
          <div class="identity-item">
            <span class="identity-label">{translate(data.language, "auth.profile.principalTypeLabel")}</span>
            <strong>{data.profile.principal_type}</strong>
          </div>
        </div>

        <div class="meta-block">
          <span class="identity-label">{translate(data.language, "auth.profile.subjectLabel")}</span>
          <code>{data.profile.subject}</code>
        </div>

          <div class="meta-block">
            <span class="identity-label">{translate(data.language, "auth.profile.scopesLabel")}</span>
            <div class="scope-list">
              {#each data.profile.scopes as scope}
                <div class="scope-chip">
                  <code>{scope}</code>
                  <span>{scopeCatalog.get(scope)?.description || scope}</span>
                </div>
              {/each}
            </div>
          </div>

        <div class="actions compact-actions">
          <LinkButton block href="/logout">{translate(data.language, "auth.profile.securityAction")}</LinkButton>
          <form method="POST" action="/logout">
            <Button variant="secondary" block type="submit">{translate(data.language, "auth.common.logout")}</Button>
          </form>
        </div>
      </Stack>
    </Card>

    <Card>
      <CardHeader>
        <SectionLabel>{translate(data.language, "auth.profile.detailsEyebrow")}</SectionLabel>
        <DisplayTitle>{translate(data.language, "auth.profile.detailsTitle")}</DisplayTitle>
        <MutedText>{translate(data.language, "auth.profile.detailsSummary")}</MutedText>
      </CardHeader>

      {#if profileMessage}
        <Alert class="banner" tone={profileTone} role={profileTone === "error" ? "alert" : "status"} aria-live={profileTone === "error" ? "assertive" : "polite"}>
          {profileMessage}
        </Alert>
      {/if}

      <form class="form-fields" method="POST" action="?/updateProfile">
        <Field label={translate(data.language, "auth.common.nameLabel")} optional={translate(data.language, "auth.profile.clearHint")}>
          <Input name="display_name" autocomplete="name" autocapitalize="words" spellcheck="false" placeholder={translate(data.language, "auth.common.namePlaceholder")} value={profileDisplayName} />
        </Field>

        <Button block type="submit">{translate(data.language, "auth.profile.saveAction")}</Button>
      </form>
    </Card>

    <Card>
      <CardHeader>
        <SectionLabel>{translate(data.language, "auth.tokens.eyebrow")}</SectionLabel>
        <DisplayTitle>{translate(data.language, "auth.tokens.title")}</DisplayTitle>
        <MutedText>{translate(data.language, "auth.tokens.summary")}</MutedText>
      </CardHeader>

      {#if tokenMessage}
        <Alert class="banner" tone={tokenTone} role={tokenTone === "error" ? "alert" : "status"} aria-live={tokenTone === "error" ? "assertive" : "polite"}>
          {tokenMessage}
        </Alert>
      {/if}

      {#if createdToken}
        <Alert class="banner" tone="success" role="status" aria-live="polite">
          <div class="secret-block">
            <strong>{translate(data.language, "auth.tokens.secretLabel")}</strong>
            <code>{createdToken.token}</code>
            <span>{translate(data.language, "auth.tokens.secretHint")}</span>
          </div>
        </Alert>
      {/if}

      <form class="form-fields token-form" method="POST" action="?/createToken">
        <Field label={translate(data.language, "auth.tokens.nameLabel")} required>
          <Input name="name" required placeholder={translate(data.language, "auth.tokens.namePlaceholder")} value={tokenName} />
        </Field>

        <Field label={translate(data.language, "auth.tokens.expiryLabel")} optional={translate(data.language, "auth.tokens.expiryOptional")}>
          <Input name="expires_at" type="datetime-local" value={tokenExpiry} />
        </Field>

        <Button block type="submit">{translate(data.language, "auth.tokens.createAction")}</Button>
      </form>

      <div class="token-list">
        {#if data.tokens.length === 0}
          <MutedText>{translate(data.language, "auth.tokens.empty")}</MutedText>
        {:else}
          {#each data.tokens as token}
            <div class="token-row">
              <div class="token-main">
                <strong>{token.name}</strong>
                <code>{token.prefix}</code>
              </div>

              <div class="token-meta">
                <span>{translate(data.language, "auth.tokens.createdLabel")}: {formatDateTime(token.created_at)}</span>
                <span>{translate(data.language, "auth.tokens.lastUsedLabel")}: {formatDateTime(token.last_used_at)}</span>
                <span>{translate(data.language, "auth.tokens.expiresLabel")}: {formatDateTime(token.expires_at)}</span>
                <span>{translate(data.language, "auth.tokens.statusLabel")}: {token.revoked_at ? translate(data.language, "auth.tokens.statusRevoked") : translate(data.language, "auth.tokens.statusActive")}</span>
              </div>

              <form method="POST" action="?/revokeToken">
                <input type="hidden" name="id" value={token.id} />
                <Button variant="secondary" block type="submit" disabled={Boolean(token.revoked_at)}>
                  {token.revoked_at ? translate(data.language, "auth.tokens.revokedAction") : translate(data.language, "auth.tokens.revokeAction")}
                </Button>
              </form>
            </div>
          {/each}
        {/if}
      </div>
    </Card>
  </Panel>
</PageRoot>

<style>
  .actions,
  .form-fields,
  .token-list,
  .token-meta,
  .secret-block,
  .scope-list {
    display: grid;
  }

  .actions,
  .compact-actions {
    gap: 12px;
  }

  .compact-actions {
    margin-top: 4px;
  }

  .form-fields,
  .token-list,
  .token-meta,
  .secret-block,
  .scope-list {
    gap: 16px;
  }

  .identity-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 16px;
  }

  .identity-item,
  .meta-block,
  .token-row {
    display: grid;
    gap: 6px;
  }

  .identity-label {
    color: rgba(255, 255, 255, 0.68);
    font-size: 0.82rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .meta-block code,
  .scope-list code,
  .token-main code,
  .secret-block code {
    overflow-wrap: anywhere;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.08);
    padding: 8px 10px;
  }

  .scope-list {
    grid-template-columns: repeat(auto-fit, minmax(140px, max-content));
  }

  .scope-chip {
    display: grid;
    gap: 6px;
  }

  .scope-chip span {
    color: rgba(255, 255, 255, 0.72);
    font-size: 0.82rem;
    max-width: 18rem;
  }

  .token-row {
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    padding-top: 16px;
  }

  .token-main {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 12px;
  }

  .token-meta {
    color: rgba(255, 255, 255, 0.8);
    font-size: 0.95rem;
  }

  .token-form {
    margin-bottom: 20px;
  }

  :global(.banner) {
    margin-bottom: 16px;
  }

  @media (max-width: 720px) {
    .identity-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
