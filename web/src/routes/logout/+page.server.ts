import type { PageServerLoad } from "./$types";
import { clearAuthCookie, clearAuthCookieFromHeader } from "$lib/server/cookies";

export const load: PageServerLoad = async ({ cookies, fetch }) => {
  try {
    const response = await fetch("/api/v1/auth/logout", {
      method: "POST",
    });

    const setCookie = response.headers.get("set-cookie");
    if (setCookie) {
      clearAuthCookieFromHeader(cookies, setCookie);
    } else {
      clearAuthCookie(cookies);
    }
  } catch {
    clearAuthCookie(cookies);
  }

  return {
    success: true,
  };
};
