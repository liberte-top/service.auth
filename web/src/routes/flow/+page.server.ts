import type { PageServerLoad } from "./$types";
import type { PreferencesResponse } from "$openapi/client";
import { apiJson } from "$lib/server/api";
import { sanitizeInternalPath } from "$lib/server/redirects";

export type FlowStep = "verify-success" | "verify-invalid" | "login-success" | "login-invalid";

export const load: PageServerLoad = async ({ url, fetch }) => {
  const requestedStep = url.searchParams.get("step");
  const step: FlowStep =
    requestedStep === "verify-invalid" || requestedStep === "login-success" || requestedStep === "login-invalid"
      ? requestedStep
      : "verify-success";

  const preferences = await apiJson<PreferencesResponse>(fetch, "/api/v1/preferences");
  return {
    step,
    email: url.searchParams.get("email") || "",
    rewrite: sanitizeInternalPath(url.searchParams.get("rewrite")),
    next: sanitizeInternalPath(url.searchParams.get("next")) || "/",
    traceId: url.searchParams.get("trace_id") || "",
    language: preferences.data?.language || "en",
    canonical: `${url.origin}/flow?step=${step}`,
  };
};
