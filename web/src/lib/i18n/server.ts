import type { Cookies } from "@sveltejs/kit";
import { DEFAULT_LANGUAGE, LIBERTE_LANGUAGE_COOKIE, LIBERTE_LANGUAGE_HEADER, SUPPORTED_LANGUAGES, normalizeLanguage, type LiberteLanguage } from "$lib/i18n/shared";

export function languageFromCookies(cookies: Cookies): LiberteLanguage {
  return normalizeLanguage(cookies.get(LIBERTE_LANGUAGE_COOKIE));
}

export function languageHeader(language: LiberteLanguage) {
  return {
    [LIBERTE_LANGUAGE_HEADER]: language,
  };
}

export function setLanguageCookie(cookies: Cookies, language: LiberteLanguage) {
  cookies.set(LIBERTE_LANGUAGE_COOKIE, language, {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    domain: ".liberte.top",
    maxAge: 60 * 60 * 24 * 365,
  });
}

export { DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES };
