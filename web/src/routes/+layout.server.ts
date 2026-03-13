import { env } from "$env/dynamic/private";
import pkg from "../../package.json";
import versionInfo from "../../public/version.json";
import type { LayoutServerLoad } from "./$types";
import { DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES, languageFromCookies } from "$lib/i18n/server";

export const load: LayoutServerLoad = async ({ cookies }) => ({
  buildInfo: {
    version: versionInfo.version || pkg.version,
    sha: versionInfo.git_sha || env.APP_BUILD_SHA || "unknown",
    timestamp: versionInfo.built_at || env.APP_BUILD_TIMESTAMP || "unknown",
  },
  language: languageFromCookies(cookies),
  defaultLanguage: DEFAULT_LANGUAGE,
  supportedLanguages: SUPPORTED_LANGUAGES,
});
