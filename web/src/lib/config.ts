import { env } from "$env/dynamic/private";
import pkg from "../../package.json";
import versionInfo from "../../public/version.json";

function withFallback<T>(value: T | null | undefined | "", fallback: T): T {
  return value || fallback;
}

export const authSessionCookieConfig = {
  name: withFallback(env.FORWARDAUTH_SESSION_COOKIE_NAME, "liberte_session"),
  domain: env.FORWARDAUTH_SESSION_COOKIE_DOMAIN || undefined,
} as const;

export const apiConfig = {
  internalOrigin: withFallback(env.AUTH_API_INTERNAL_URL, "http://auth-api:3333"),
} as const;

export const buildInfoConfig = {
  version: withFallback(versionInfo.version, pkg.version),
  sha: withFallback(versionInfo.git_sha || env.APP_BUILD_SHA, "unknown"),
  timestamp: withFallback(versionInfo.built_at || env.APP_BUILD_TIMESTAMP, "unknown"),
} as const;

export const config = {
  api: apiConfig,
  authSessionCookie: authSessionCookieConfig,
  buildInfo: buildInfoConfig,
} as const;
