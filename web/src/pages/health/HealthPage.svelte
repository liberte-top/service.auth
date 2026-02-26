<script lang="ts">
  import { onMount } from "svelte";
  import { getHealth } from "../../../openapi/client";

  let status = "loading";
  let checkedAt = "-";

  async function refresh() {
    try {
      const response = await getHealth();
      status = response.status;
      checkedAt = new Date().toLocaleTimeString();
    } catch {
      status = "unreachable";
      checkedAt = new Date().toLocaleTimeString();
    }
  }

  onMount(refresh);
</script>

<main>
  <section class="card">
    <h1>Health Page</h1>
    <nav class="nav">
      <a href="/">Home</a>
      <a href="/showcase.html">Showcase page</a>
      <a href="/notes.html">Notes page</a>
    </nav>
    <p>Current status: <strong>{status}</strong></p>
    <p>Last checked: <code>{checkedAt}</code></p>
    <button on:click={refresh}>Refresh</button>
  </section>
</main>
