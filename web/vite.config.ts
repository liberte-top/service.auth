import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "node:path";

const buildVersion = process.env.npm_package_version ?? "0.0.0";
const buildSha = process.env.APP_BUILD_SHA ?? "unknown";
const buildTimestamp = process.env.APP_BUILD_TIMESTAMP ?? "unknown";

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        flow: resolve(__dirname, "flow.html"),
        profile: resolve(__dirname, "profile.html"),
        health: resolve(__dirname, "health.html"),
        showcase: resolve(__dirname, "showcase.html"),
        notes: resolve(__dirname, "notes.html"),
      },
    },
  },
  plugins: [
    {
      name: "inject-build-meta",
      transformIndexHtml(html) {
        return html.replace(
          "</head>",
          [
            `    <meta name="liberte:build-version" content="${buildVersion}" />`,
            `    <meta name="liberte:build-sha" content="${buildSha}" />`,
            `    <meta name="liberte:build-timestamp" content="${buildTimestamp}" />`,
            "  </head>",
          ].join("\n")
        );
      },
    },
    svelte({
      compilerOptions: {
        compatibility: {
          componentApi: 4,
        },
      },
    }),
  ],
});
