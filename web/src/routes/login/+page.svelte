<script lang="ts">
  import AuthPage from "$lib/auth/AuthPage.svelte";
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();
</script>

<AuthPage mode="login" authContext={data.authContext} email={data.email} rewrite={data.rewrite} verified={data.verified} canonical={data.canonical} primaryAction="?/login" {form} />
