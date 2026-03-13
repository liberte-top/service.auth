export const LIBERTE_LANGUAGE_COOKIE = "liberte_language";
export const LIBERTE_LANGUAGE_HEADER = "x-liberte-language";
export const DEFAULT_LANGUAGE = "en";
export const SUPPORTED_LANGUAGES = ["en", "zh-CN"] as const;

export type LiberteLanguage = (typeof SUPPORTED_LANGUAGES)[number];

export function normalizeLanguage(value?: string | null): LiberteLanguage {
  if (!value) return DEFAULT_LANGUAGE;

  const lowered = value.trim().toLowerCase();
  if (lowered === "zh" || lowered === "zh-cn") return "zh-CN";
  if (lowered === "en" || lowered === "en-us" || lowered === "en-gb") return "en";
  return DEFAULT_LANGUAGE;
}
