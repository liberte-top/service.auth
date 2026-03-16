import { env } from "$env/dynamic/private";
import { openapi } from "$openapi";
import pkg from "../../package.json";
import versionInfo from "../../public/version.json";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch }) => {
  const api = openapi.create(fetch);
  const [{ data: preferences }, { data: preferenceOptions }] = await Promise.all([
    api.getPreferences(),
    api.getPreferenceOptions(),
  ]);
  return {
    buildInfo: {
      version: versionInfo.version || pkg.version,
      sha: versionInfo.git_sha || env.APP_BUILD_SHA || "unknown",
      timestamp: versionInfo.built_at || env.APP_BUILD_TIMESTAMP || "unknown",
    },
    preferences: preferences,
    preferenceOptions,
  };
};
