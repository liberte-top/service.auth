<script lang="ts">
  import { onMount } from "svelte";
  import { createAccount, getAccount, getHealth, type AccountPayload, type AccountResponse } from "../openapi/client";

  const envLabel = import.meta.env.VITE_ENV_LABEL ?? "local";

  let healthText = "loading";
  let healthOk = false;
  let busy = false;
  let createError = "";
  let created: AccountResponse | null = null;
  let fetched: AccountResponse | null = null;

  let accountType = "user";
  let username = "demo-user";
  let email = "demo@example.com";

  onMount(async () => {
    try {
      const health = await getHealth();
      healthText = health.status;
      healthOk = health.status === "ok";
    } catch {
      healthText = "unreachable";
      healthOk = false;
    }
  });

  async function runCreateAndRead() {
    busy = true;
    createError = "";
    created = null;
    fetched = null;

    try {
      const suffix = `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 6)}`;
      const payload: AccountPayload = {
        account_type: accountType,
        username: (username ? `${username}-${suffix}` : null),
        email: (email ? `${suffix}-${email}` : null),
        phone: null,
        created_by: null,
      };
      const createResult = await createAccount(payload);
      created = createResult;
      fetched = await getAccount(createResult.uid);
    } catch (error) {
      createError = error instanceof Error ? error.message : "request failed";
    } finally {
      busy = false;
    }
  }
</script>

<main>
  <section class="card">
    <h1>service.auth web</h1>
    <nav class="nav">
      <a href="/">Home</a>
      <a href="/health.html">Health page</a>
      <a href="/showcase.html">Showcase page</a>
      <a href="/notes.html">Notes page</a>
    </nav>
    <p>Environment: <strong>{envLabel}</strong></p>
    <p>
      API health:
      <strong class={healthOk ? "status-ok" : "status-fail"}>{healthText}</strong>
    </p>
  </section>

  <section class="card">
    <h2>Accounts CRUD demo</h2>
    <label>
      account_type
      <input bind:value={accountType} />
    </label>
    <label>
      username
      <input bind:value={username} />
    </label>
    <label>
      email
      <input bind:value={email} />
    </label>

    <button disabled={busy} on:click={runCreateAndRead}>
      {busy ? "Processing..." : "Create + Read account"}
    </button>

    {#if createError}
      <p class="status-fail">{createError}</p>
    {/if}

    {#if created}
      <p>Created account uid: <code>{created.uid}</code></p>
      <pre>{JSON.stringify(created, null, 2)}</pre>
    {/if}

    {#if fetched}
      <p>Fetched account</p>
      <pre>{JSON.stringify(fetched, null, 2)}</pre>
    {/if}
  </section>
</main>
