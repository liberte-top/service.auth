<script lang="ts">
  import { onMount } from "svelte";
  import { createAccount, getAccount, getHealth, type AccountPayload, type AccountResponse } from "../openapi/client";
  import { apiClient } from "../openapi/http";

  const envLabel = import.meta.env.VITE_ENV_LABEL ?? "local";

  let healthText = "loading";
  let healthOk = false;
  let busy = false;
  let createError = "";
  let created: AccountResponse | null = null;
  let fetched: AccountResponse | null = null;
  let authBusy = false;
  let authEmail = "demo-auth@example.com";
  let authDisplayName = "Demo Auth";
  let authStatus = "idle";
  let authResult = "";
  let authContext = "";

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

    await refreshAuthContext();
  });

  async function postJson(path: string, payload: Record<string, unknown>) {
    const response = await apiClient.post(path, payload, {
      validateStatus: () => true,
    });
    return {
      ok: response.status >= 200 && response.status < 300,
      status: response.status,
      text: typeof response.data === "string" ? response.data : JSON.stringify(response.data),
    };
  }

  async function refreshAuthContext() {
    const response = await apiClient.get("/api/v1/context", {
      validateStatus: (status: number) => status === 200 || status === 401,
    });
    authContext = response.status === 401
      ? "signed out"
      : typeof response.data === "string"
        ? response.data
        : JSON.stringify(response.data);
  }

  async function runAuthAction(label: string, path: string, payload: Record<string, unknown>) {
    authBusy = true;
    authStatus = `${label}...`;
    authResult = "";

    try {
      const result = await postJson(path, payload);
      authStatus = `${label} ${result.ok ? "accepted" : "failed"}`;
      authResult = `HTTP ${result.status}\n${result.text}`;
      await refreshAuthContext();
    } catch (error) {
      authStatus = `${label} failed`;
      authResult = error instanceof Error ? error.message : "request failed";
    } finally {
      authBusy = false;
    }
  }

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

  async function runRegister() {
    await runAuthAction("register", "/api/v1/auth/register/email", {
      email: authEmail,
      display_name: authDisplayName || null,
    });
  }

  async function runResendVerify() {
    await runAuthAction("resend verify", "/api/v1/auth/verify/email/resend", {
      email: authEmail,
    });
  }

  async function runRequestLogin() {
    await runAuthAction("request login", "/api/v1/auth/login/email", {
      email: authEmail,
    });
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
    <h2>Email auth flow</h2>
    <p>Start registration or request a sign-in link here, then complete the flow from the email link.</p>

    <label>
      auth email
      <input bind:value={authEmail} />
    </label>
    <label>
      display name
      <input bind:value={authDisplayName} />
    </label>

    <div class="actions">
      <button disabled={authBusy} on:click={runRegister}>register email</button>
      <button disabled={authBusy} on:click={runResendVerify}>resend verify link</button>
      <button disabled={authBusy} on:click={runRequestLogin}>request login link</button>
      <button disabled={authBusy} on:click={refreshAuthContext}>refresh session</button>
    </div>

    <p>
      Status:
      <strong>{authStatus}</strong>
    </p>

    {#if authResult}
      <pre>{authResult}</pre>
    {/if}

    <p>Current auth context</p>
    <pre>{authContext}</pre>
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

<style>
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
</style>
