export function sanitizeInternalPath(value: string | null) {
  const trimmed = (value || "").trim();
  if (!trimmed) return "";
  if (!trimmed.startsWith("/")) return "";
  if (trimmed.startsWith("//")) return "";

  try {
    const url = new URL(trimmed, "https://auth.liberte.top");
    return `${url.pathname}${url.search}${url.hash}`;
  } catch {
    return "";
  }
}
