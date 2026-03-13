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
