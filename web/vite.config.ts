import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

const buildVersion = process.env.npm_package_version ?? "0.0.0";
const buildSha = process.env.APP_BUILD_SHA ?? "unknown";
const buildTimestamp = process.env.APP_BUILD_TIMESTAMP ?? "unknown";

export default defineConfig({
  plugins: [
    sveltekit(),
    {
      name: "inject-build-meta",
      transformIndexHtml(html) {
        return html.replace(
          "%liberte.head%",
          [
            `<meta name="liberte:build-version" content="${buildVersion}" />`,
            `<meta name="liberte:build-sha" content="${buildSha}" />`,
            `<meta name="liberte:build-timestamp" content="${buildTimestamp}" />`,
            "%liberte.head%",
          ].join("\n    ")
        );
      },
    },
  ],
});
