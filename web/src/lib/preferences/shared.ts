export const LIBERTE_LANGUAGE_COOKIE = "liberte_language";
export const LIBERTE_THEME_COOKIE = "liberte_theme";
export const LIBERTE_LANGUAGE_HEADER = "x-liberte-language";

export const DEFAULT_LANGUAGE = "en";
export const SUPPORTED_LANGUAGES = ["en", "zh-CN"] as const;

export const DEFAULT_THEME = "system";
export const SUPPORTED_THEMES = ["system", "light", "dark"] as const;

export type LiberteLanguage = (typeof SUPPORTED_LANGUAGES)[number];
export type LiberteTheme = (typeof SUPPORTED_THEMES)[number];

export type LibertePreferences = {
  language: LiberteLanguage;
  theme: LiberteTheme;
};

export function normalizeLanguage(value?: string | null): LiberteLanguage {
  if (!value) return DEFAULT_LANGUAGE;

  const lowered = value.trim().toLowerCase();
  if (lowered === "zh" || lowered === "zh-cn") return "zh-CN";
  if (lowered === "en" || lowered === "en-us" || lowered === "en-gb") return "en";
  return DEFAULT_LANGUAGE;
}

export function normalizeTheme(value?: string | null): LiberteTheme {
  if (!value) return DEFAULT_THEME;

  const lowered = value.trim().toLowerCase();
  if (lowered === "light") return "light";
  if (lowered === "dark") return "dark";
  return DEFAULT_THEME;
}
