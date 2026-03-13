import { env } from "$env/dynamic/private";
import type { Cookies } from "@sveltejs/kit";

export function clearAuthCookie(cookies: Cookies) {
  cookies.set(env.FORWARDAUTH_SESSION_COOKIE_NAME || "liberte_session", "", {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    expires: new Date(0),
    domain: env.FORWARDAUTH_SESSION_COOKIE_DOMAIN || undefined,
  });
}

export function clearAuthCookieFromHeader(cookies: Cookies, setCookieHeader: string) {
  const [cookiePart, ...attributeParts] = setCookieHeader.split(";");
  const [rawName] = cookiePart.split("=");
  const name = rawName?.trim();

  if (!name) {
    clearAuthCookie(cookies);
    return;
  }

  let domain: string | undefined;
  let path = "/";

  for (const part of attributeParts) {
    const [rawKey, rawValue] = part.split("=");
    const key = rawKey?.trim().toLowerCase();
    const value = rawValue?.trim();

    if (key === "domain" && value) {
      domain = value;
    }

    if (key === "path" && value) {
      path = value;
    }
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
