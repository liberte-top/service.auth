<script lang="ts">
  const items = [
    "adapter-static prefers build-time certainty",
    "each html file can be an independent entry",
    "compose can stay focused on runtime services",
  ];
</script>

<main>
  <section class="card">
    <h1>Notes Page</h1>
    <nav class="nav">
      <a href="/">Home</a>
      <a href="/health.html">Health page</a>
      <a href="/showcase.html">Showcase page</a>
    </nav>
    <p>Simple content page to demonstrate MPA navigation and independent bundles.</p>
    <ul>
      {#each items as item}
        <li>{item}</li>
      {/each}
    </ul>
  </section>
</main>
