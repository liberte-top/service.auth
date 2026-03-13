<script lang="ts">
  import { browser } from "$app/environment";
  import { onMount } from "svelte";
  import type { PageData } from "./$types";

  export let data: PageData;

  let countdown = 4;

  const config = {
    "verify-success": {
      eyebrow: "Email verified",
      title: "Your address is confirmed",
      copy: "You can now continue to sign in with your email link.",
      action: "Continue to sign in",
      fallback: "Back to sign in",
      tone: "success",
      autoRedirect: true,
      titleTag: "Email verified - Liberte",
    },
    "verify-invalid": {
      eyebrow: "Verification issue",
      title: "This verification link is no longer valid",
      copy: "The link may have expired or already been used. Return to the sign-in page and request a fresh email.",
      action: "Request a new verification email",
      fallback: "Back to sign in",
      tone: "error",
      autoRedirect: false,
      titleTag: "Verification link expired - Liberte",
    },
    "login-success": {
      eyebrow: "Sign-in complete",
      title: "You are signed in",
      copy: "Your session is active in this browser and you can continue to the requested page.",
      action: "Continue",
      fallback: "Back to sign in",
      tone: "success",
      autoRedirect: true,
      titleTag: "Signed in - Liberte",
    },
    "login-invalid": {
      eyebrow: "Sign-in issue",
      title: "This sign-in link cannot be used anymore",
      copy: "The email link may have expired or already been consumed. Return to the sign-in page to request another one.",
      action: "Request a fresh sign-in link",
      fallback: "Back to sign in",
      tone: "error",
      autoRedirect: false,
      titleTag: "Sign-in link expired - Liberte",
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
          Redirecting in {countdown} seconds.
        {:else}
          Return to the sign-in page to request a fresh email.
        {/if}
      </p>

      <div class="actions">
        <a class="button-link" href={destination}>{config.action}</a>
        <a class="button-link secondary-link" href="/login">{config.fallback}</a>
      </div>

      <div class="flow-meta">
        {#if data.email}
          <p class="flow-copy">Email: <strong>{data.email}</strong></p>
        {/if}
        <p class="flow-copy">Destination: <code>{destination}</code></p>
      </div>
    </section>
  </section>
</main>
