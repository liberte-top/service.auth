import { env } from "$env/dynamic/private";
import { getPreferences } from "$lib/server/auth-api";
import pkg from "../../package.json";
import versionInfo from "../../public/version.json";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch }) => {
  const { data: preferences } = await getPreferences(fetch);
  return {
    buildInfo: {
      version: versionInfo.version || pkg.version,
      sha: versionInfo.git_sha || env.APP_BUILD_SHA || "unknown",
      timestamp: versionInfo.built_at || env.APP_BUILD_TIMESTAMP || "unknown",
    },
    preferences: preferences,
  };
};
