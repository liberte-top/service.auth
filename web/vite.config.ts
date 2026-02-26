import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "node:path";

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        health: resolve(__dirname, "health.html"),
        showcase: resolve(__dirname, "showcase.html"),
        notes: resolve(__dirname, "notes.html"),
      },
    },
  },
  plugins: [
    svelte({
      compilerOptions: {
        compatibility: {
          componentApi: 4,
        },
      },
    }),
  ],
});
