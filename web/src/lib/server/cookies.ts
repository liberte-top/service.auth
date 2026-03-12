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
