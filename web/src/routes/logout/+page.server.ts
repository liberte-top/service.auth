import type { Actions, PageServerLoad } from "./$types";
import { languageFromCookies } from "$lib/i18n/server";
import { clearAuthCookie, clearAuthCookieFromHeader } from "$lib/server/cookies";

async function performLogout(cookies: Parameters<PageServerLoad>[0]["cookies"], fetch: Parameters<PageServerLoad>[0]["fetch"]) {
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
}

export const load: PageServerLoad = async ({ url, cookies }) => {
  return {
    language: languageFromCookies(cookies),
    canonical: `${url.origin}/logout`,
  };
};

export const actions: Actions = {
  default: async ({ cookies, fetch }) => {
    await performLogout(cookies, fetch);

    return {
      success: true,
    };
  },
};
