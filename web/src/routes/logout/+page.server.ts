import type { Actions, PageServerLoad } from "./$types";
import { ensure } from "@liberte-top/shared/ensure";
import { config } from "$lib/config";
import { AppError } from "$lib/error";
import { openapi } from "$openapi";
import type { Cookies } from "@sveltejs/kit";

function clearAuthCookie(cookies: Cookies) {
  cookies.set(config.authSessionCookie.name, "", {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    expires: new Date(0),
    domain: config.authSessionCookie.domain,
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

  const setCookie = ensure.nonEmpty(response.headers.get("set-cookie"), () => AppError.logoutCookieMissing(), () => clearAuthCookie(cookies));
  ensure(response.ok, () => AppError.logoutFailed(response.status), () => clearAuthCookie(cookies));

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
  const api = openapi.create(fetch);
  const { data: preferences } = await api.getPreferences();
  return {
    language: preferences.language,
    canonical: `${url.origin}/logout`,
  };
};
