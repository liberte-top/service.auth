<script lang="ts">
  import type { LiberteLanguage } from "$lib/i18n/shared";

  let {
    language,
    supportedLanguages,
  }: {
    language: LiberteLanguage;
    supportedLanguages: readonly LiberteLanguage[];
  } = $props();

  let busy = $state(false);

  async function setLanguage(nextLanguage: LiberteLanguage) {
    if (busy || nextLanguage === language) return;
    busy = true;

    try {
      const response = await fetch("/api/v1/preferences/language", {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify({ language: nextLanguage }),
      });

      if (!response.ok) {
        throw new Error(`language update failed: ${response.status}`);
      }

      window.location.reload();
    } finally {
      busy = false;
    }
  }

  function labelFor(language: LiberteLanguage) {
    return language === "zh-CN" ? "中文" : "EN";
  }
</script>

<div class="language-switcher" aria-label="Language switcher">
  {#each supportedLanguages as candidate}
    <button
      class:active={candidate === language}
      class="language-chip"
      type="button"
      disabled={busy && candidate !== language}
      aria-pressed={candidate === language}
      onclick={() => setLanguage(candidate)}
    >
      {labelFor(candidate)}
    </button>
  {/each}
</div>

<style>
  .language-switcher {
    position: fixed;
    top: 16px;
    right: 16px;
    z-index: 10;
    display: inline-flex;
    gap: 8px;
    padding: 6px;
    border: 1px solid var(--lt-color-border);
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.92);
    box-shadow: var(--lt-card-shadow);
  }

  .language-chip {
    width: auto;
    min-height: 32px;
    padding: 0 10px;
    border-radius: 999px;
    border: 1px solid transparent;
    background: transparent;
    color: var(--lt-color-text-muted);
    font-size: 13px;
    font-weight: var(--lt-font-weight-strong);
    cursor: pointer;
  }

  .language-chip.active {
    background: var(--lt-color-primary);
    color: var(--lt-color-on-primary);
  }

  .language-chip:hover:not(:disabled) {
    background: var(--lt-color-surface-hover);
    color: var(--lt-color-text);
  }

  .language-chip.active:hover:not(:disabled) {
    background: var(--lt-color-primary-hover);
    color: var(--lt-color-on-primary);
  }

  @media (max-width: 640px) {
    .language-switcher {
      top: 12px;
      right: 12px;
    }
  }
</style>
