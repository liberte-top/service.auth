import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { SUPPORTED_LANGUAGES, languageFromCookies, setLanguageCookie } from "$lib/i18n/server";
import { normalizeLanguage } from "$lib/i18n/shared";

export const POST: RequestHandler = async ({ cookies, request }) => {
  let language = languageFromCookies(cookies);

  try {
    const payload = (await request.json()) as { language?: string };
    language = normalizeLanguage(payload.language);
  } catch {
    return json({ error: "invalid_json" }, { status: 400 });
  }

  setLanguageCookie(cookies, language);

  return json({ language, supportedLanguages: SUPPORTED_LANGUAGES });
};
