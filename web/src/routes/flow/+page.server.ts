import type { PageServerLoad } from "./$types";

export type FlowStep = "verify-success" | "verify-invalid" | "login-success" | "login-invalid";

function sanitizeTarget(value: string | null) {
  const trimmed = (value || "").trim();
  if (!trimmed) return "";
  if (trimmed.startsWith("/")) return trimmed;
  try {
    const url = new URL(trimmed);
    return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : "";
  } catch {
    return "";
  }
}

export const load: PageServerLoad = async ({ url }) => {
  const requestedStep = url.searchParams.get("step");
  const step: FlowStep =
    requestedStep === "verify-invalid" || requestedStep === "login-success" || requestedStep === "login-invalid"
      ? requestedStep
      : "verify-success";

  return {
    step,
    email: url.searchParams.get("email") || "",
    rewrite: sanitizeTarget(url.searchParams.get("rewrite")),
    next: sanitizeTarget(url.searchParams.get("next")) || "/",
  };
};
