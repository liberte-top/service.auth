import { env } from "$env/dynamic/private";
import pkg from "../../package.json";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async () => ({
  buildInfo: {
    version: pkg.version,
    sha: env.APP_BUILD_SHA || "unknown",
    timestamp: env.APP_BUILD_TIMESTAMP || "unknown",
  },
});
