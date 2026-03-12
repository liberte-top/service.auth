<script lang="ts">
  import { onMount } from "svelte";

  type FlowStep = "verify-success" | "verify-invalid" | "login-success" | "login-invalid";

  type FlowConfig = {
    eyebrow: string;
    title: string;
    lede: string;
    tone: "success" | "error" | "info";
    primaryLabel: string;
    fallbackLabel: string;
    notes: string[];
    autoRedirect: boolean;
  };

  const HOME_URL = "/";

  let countdown = 4;
  let step: FlowStep = "verify-success";
  let email = "";
  let rewrite = "";
  let next = HOME_URL;

  function sanitizeTarget(value: string | null) {
    const trimmed = (value || "").trim();
    if (!trimmed) return "";
    if (trimmed.startsWith("/")) return trimmed;
    try {
      const url = new URL(trimmed);
      return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : "";
    } catch {
      return "";
    }
  }

  function configFor(currentStep: FlowStep): FlowConfig {
    switch (currentStep) {
      case "verify-success":
        return {
          eyebrow: "Email verified",
          title: "Your address is confirmed.",
          lede: "We are sending you back to the sign-in page so you can request your first passwordless login link.",
          tone: "success",
          primaryLabel: "Continue to sign in",
          fallbackLabel: "Back to auth home",
          notes: [
            "Verification is complete for this email address.",
            "Your next step is opening the one-time sign-in link from your inbox.",
            "If a destination was attached, we will keep it for the next redirect.",
          ],
          autoRedirect: true,
        };
      case "verify-invalid":
        return {
          eyebrow: "Verification issue",
          title: "This verification link is no longer valid.",
          lede: "The token may have expired, already been used, or been copied incorrectly. You can return to auth home and request a fresh email.",
          tone: "error",
          primaryLabel: "Request a new verification email",
          fallbackLabel: "Back to auth home",
          notes: [
            "Expired and already-used links are rejected automatically.",
            "If you still have the registration form open, use resend verification.",
            "You can safely start the flow again without harming the account.",
          ],
          autoRedirect: false,
        };
      case "login-success":
        return {
          eyebrow: "Sign-in complete",
          title: "You are signed in.",
          lede: "Your session cookie is set in this browser. We are redirecting you to the destination that started this auth flow.",
          tone: "success",
          primaryLabel: "Continue now",
          fallbackLabel: "Back to auth home",
          notes: [
            "This browser now holds an active auth session.",
            "Apps under the same liberte.top session domain can reuse it.",
            "If no destination was provided, the default landing page is your profile.",
          ],
          autoRedirect: true,
        };
      case "login-invalid":
        return {
          eyebrow: "Sign-in issue",
          title: "This sign-in link cannot be used anymore.",
          lede: "The email link may have expired or already been consumed. Return to the auth page and request another one-time link.",
          tone: "error",
          primaryLabel: "Request a fresh sign-in link",
          fallbackLabel: "Back to auth home",
          notes: [
            "Sign-in links are single-use by design.",
            "If your email client opened the link twice, the second attempt will fail.",
            "Requesting a new link is the fastest recovery path.",
          ],
          autoRedirect: false,
        };
    }
  }

  $: config = configFor(step);
  $: safeRewrite = rewrite || "/profile.html";
  $: destinationLabel = step === "login-success" ? next : safeRewrite;
  $: primaryHref = next || HOME_URL;

  onMount(() => {
    const url = new URL(window.location.href);
    const requestedStep = url.searchParams.get("step");
    if (
      requestedStep === "verify-success" ||
      requestedStep === "verify-invalid" ||
      requestedStep === "login-success" ||
      requestedStep === "login-invalid"
    ) {
      step = requestedStep;
    }

    email = url.searchParams.get("email") || "";
    rewrite = sanitizeTarget(url.searchParams.get("rewrite"));
    next = sanitizeTarget(url.searchParams.get("next")) || HOME_URL;

    if (!configFor(step).autoRedirect) {
      return;
    }

    const interval = window.setInterval(() => {
      countdown -= 1;
      if (countdown <= 0) {
        window.clearInterval(interval);
        window.location.assign(primaryHref);
      }
    }, 1000);

    return () => window.clearInterval(interval);
  });
</script>

<main class="shell">
  <section class="hero-layout flow-layout">
    <section class="hero hero-card card">
      <p class="eyebrow">{config.eyebrow}</p>
      <h1>{config.title}</h1>
      <p class="lede">{config.lede}</p>

      <p class={`banner ${config.tone} flow-banner`}>
        {#if config.autoRedirect}
          Redirecting in {countdown}s.
        {:else}
          You can recover by starting the flow again from auth home.
        {/if}
      </p>

      <div class="hero-footnote flow-metrics">
        <div class="metric-chip">
          <span class="metric-label">Current step</span>
          <strong>{step}</strong>
        </div>
        <div class="metric-chip">
          <span class="metric-label">Email</span>
          <strong>{email || "Not available"}</strong>
        </div>
        <div class="metric-chip">
          <span class="metric-label">Destination</span>
          <strong>{destinationLabel}</strong>
        </div>
      </div>
    </section>

    <aside class="hero-sidebar">
      <section class="card accent-card">
        <p class="eyebrow">Flow state</p>
        <p class="destination-copy">
          {#if step === "verify-success"}
            Verification is finished and the next step is requesting a sign-in link.
          {:else if step === "login-success"}
            Your auth session is already active in this browser.
          {:else}
            The email action could not be completed, so you need a fresh link.
          {/if}
        </p>
        <p class="destination-chip"><code>{destinationLabel}</code></p>
      </section>

      <section class="card info-card">
        <p class="eyebrow">Recovery notes</p>
        <ul class="info-list">
          {#each config.notes as note}
            <li>{note}</li>
          {/each}
        </ul>
      </section>
    </aside>
  </section>

  <section class="card auth-card flow-actions-card">
    <div class="auth-shell flow-actions-shell">
      <div class="auth-copy">
        <p class="eyebrow">Next action</p>
        <h2>{config.primaryLabel}</h2>
        <p class="auth-summary">
          {#if step === "verify-success"}
            Continue to the auth home with the verified email prefilled so the next step is one click away.
          {:else if step === "login-success"}
            Continue to the destination that requested authentication.
          {:else}
            Open auth home and request a replacement email from the current flow state.
          {/if}
        </p>
      </div>

      <div class="auth-form-block flow-action-panel">
        <div class="actions auth-actions">
          <a class="button-link" href={primaryHref}>{config.primaryLabel}</a>
          <a class="button-link secondary-link" href={HOME_URL}>{config.fallbackLabel}</a>
        </div>

        {#if config.autoRedirect}
          <p class="helper-copy">If the automatic redirect does not fire, use the primary action button above.</p>
        {:else}
          <p class="helper-copy">If you still have the original tab open, return there and resend the latest email action.</p>
        {/if}
      </div>
    </div>
  </section>
</main>
