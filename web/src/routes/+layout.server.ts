import { env } from "$env/dynamic/private";
import type { PreferencesResponse } from "$openapi/client";
import { apiJson } from "$lib/server/api";
import pkg from "../../package.json";
import versionInfo from "../../public/version.json";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch }) => {
  const { data } = await apiJson<PreferencesResponse>(fetch, "/api/v1/preferences");
  const preferences = data || {
    language: "en",
    theme: "system",
    supported_languages: ["en", "zh-CN"],
    supported_themes: ["system", "light", "dark"],
  };

  return {
    buildInfo: {
      version: versionInfo.version || pkg.version,
      sha: versionInfo.git_sha || env.APP_BUILD_SHA || "unknown",
      timestamp: versionInfo.built_at || env.APP_BUILD_TIMESTAMP || "unknown",
    },
    preferences,
  };
};
