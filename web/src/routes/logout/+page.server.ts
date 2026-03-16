import type { Actions, PageServerLoad } from "./$types";
import { ensure } from "@liberte-top/shared/ensure";
import { AppError } from "$lib/error";
import { getPreferences } from "$lib/server/auth-api";
import type { Cookies } from "@sveltejs/kit";

function clearAuthCookie(cookies: Cookies) {
  cookies.set(process.env.FORWARDAUTH_SESSION_COOKIE_NAME || "liberte_session", "", {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    expires: new Date(0),
    domain: process.env.FORWARDAUTH_SESSION_COOKIE_DOMAIN || undefined,
  });
}

function clearAuthCookieFromHeader(cookies: Cookies, setCookieHeader: string) {
  const [cookiePart, ...attributeParts] = setCookieHeader.split(";");
  const [rawName] = cookiePart.split("=");
  const name = ensure.nonEmpty(rawName?.trim(), () => AppError.logoutCookieNameMissing(), () => clearAuthCookie(cookies));

  let domain: string | undefined;
  let path = "/";

  for (const part of attributeParts) {
    const [rawKey, rawValue] = part.split("=");
    const key = rawKey?.trim().toLowerCase();
    const value = rawValue?.trim();

    if (key === "domain" && value) domain = value;
    if (key === "path" && value) path = value;
  }

  cookies.set(name, "", {
    path,
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    expires: new Date(0),
    domain,
  });
}

async function performLogout(cookies: Parameters<PageServerLoad>[0]["cookies"], fetch: Parameters<PageServerLoad>[0]["fetch"]) {
  const response = await fetch("/api/v1/auth/logout", {
    method: "POST",
  });

  const setCookie = response.headers.get("set-cookie");
  ensure(response.ok, () => AppError.logoutFailed(response.status), () => clearAuthCookie(cookies));
  ensure.nonEmpty(setCookie, () => AppError.logoutCookieMissing(), () => clearAuthCookie(cookies));

  clearAuthCookieFromHeader(cookies, setCookie);
}

export const actions: Actions = {
  default: async ({ cookies, fetch }) => {
    await performLogout(cookies, fetch);

    return {
      success: true,
    };
  }
};

export const load: PageServerLoad = async ({ url, fetch }) => {
  const { data: preferences } = await getPreferences(fetch);
  return {
    language: preferences.language,
    canonical: `${url.origin}/logout`,
  };
};
