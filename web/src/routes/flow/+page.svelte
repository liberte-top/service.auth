<script lang="ts">
  import { browser } from "$app/environment";
  import { translate } from "$lib/i18n/copy";
  import { onMount } from "svelte";
  import type { PageData } from "./$types";

  export let data: PageData;

  let countdown = 4;

  const config = {
    "verify-success": {
      eyebrow: translate(data.language, "auth.flow.verifySuccess.eyebrow"),
      title: translate(data.language, "auth.flow.verifySuccess.title"),
      copy: translate(data.language, "auth.flow.verifySuccess.copy"),
      action: translate(data.language, "auth.flow.verifySuccess.action"),
      fallback: translate(data.language, "auth.flow.fallback"),
      tone: "success",
      autoRedirect: true,
      titleTag: translate(data.language, "auth.flow.verifySuccess.titleTag"),
    },
    "verify-invalid": {
      eyebrow: translate(data.language, "auth.flow.verifyInvalid.eyebrow"),
      title: translate(data.language, "auth.flow.verifyInvalid.title"),
      copy: translate(data.language, "auth.flow.verifyInvalid.copy"),
      action: translate(data.language, "auth.flow.verifyInvalid.action"),
      fallback: translate(data.language, "auth.flow.fallback"),
      tone: "error",
      autoRedirect: false,
      titleTag: translate(data.language, "auth.flow.verifyInvalid.titleTag"),
    },
    "login-success": {
      eyebrow: translate(data.language, "auth.flow.loginSuccess.eyebrow"),
      title: translate(data.language, "auth.flow.loginSuccess.title"),
      copy: translate(data.language, "auth.flow.loginSuccess.copy"),
      action: translate(data.language, "auth.flow.loginSuccess.action"),
      fallback: translate(data.language, "auth.flow.fallback"),
      tone: "success",
      autoRedirect: true,
      titleTag: translate(data.language, "auth.flow.loginSuccess.titleTag"),
    },
    "login-invalid": {
      eyebrow: translate(data.language, "auth.flow.loginInvalid.eyebrow"),
      title: translate(data.language, "auth.flow.loginInvalid.title"),
      copy: translate(data.language, "auth.flow.loginInvalid.copy"),
      action: translate(data.language, "auth.flow.loginInvalid.action"),
      fallback: translate(data.language, "auth.flow.fallback"),
      tone: "error",
      autoRedirect: false,
      titleTag: translate(data.language, "auth.flow.loginInvalid.titleTag"),
    },
  }[data.step];

  const destination = data.step === "login-success" ? data.next : data.rewrite || "/profile";

  onMount(() => {
    if (!browser || !config.autoRedirect) return;

    const interval = window.setInterval(() => {
      countdown -= 1;
      if (countdown <= 0) {
        window.clearInterval(interval);
        window.location.assign(destination);
      }
    }, 1000);

    return () => window.clearInterval(interval);
  });
</script>

<svelte:head>
  <title>{config.titleTag}</title>
  <meta name="description" content={config.copy} />
  <meta name="robots" content="noindex, nofollow" />
  <link rel="canonical" href={data.canonical} />
</svelte:head>

<main class="page auth-page">
  <section class="flow-shell">
    <a class="brand-link" href="/login">liberte.top</a>

    <section class="card flow-card">
      <div class="card-header">
        <p class="section-label">{config.eyebrow}</p>
        <h1>{config.title}</h1>
        <p>{config.copy}</p>
      </div>

      <p class={`banner ${config.tone}`} role={config.tone === "error" ? "alert" : "status"} aria-live={config.autoRedirect ? "polite" : config.tone === "error" ? "assertive" : "polite"}>
        {#if config.autoRedirect}
          {translate(data.language, "auth.flow.redirecting", { countdown })}
        {:else}
          {translate(data.language, "auth.flow.retry")}
        {/if}
      </p>

      <div class="actions">
        <a class="button-link" href={destination}>{config.action}</a>
        <a class="button-link secondary-link" href="/login">{config.fallback}</a>
      </div>

      <div class="flow-meta">
        {#if data.email}
          <p class="flow-copy">{translate(data.language, "auth.flow.email")}: <strong>{data.email}</strong></p>
        {/if}
        <p class="flow-copy">{translate(data.language, "auth.flow.destination")}: <code>{destination}</code></p>
      </div>
    </section>
  </section>
</main>
