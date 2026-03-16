import type { PageServerLoad } from "./$types";
import { getPreferences } from "$lib/server/auth-api";

export type FlowStep = "verify-success" | "verify-invalid" | "login-success" | "login-invalid";

export const load: PageServerLoad = async ({ url, fetch }) => {
  const requestedStep = url.searchParams.get("step");
  const step: FlowStep =
    requestedStep === "verify-invalid" || requestedStep === "login-success" || requestedStep === "login-invalid"
      ? requestedStep
      : "verify-success";

  const { data: preferences } = await getPreferences(fetch);
  return {
    step,
    email: url.searchParams.get("email") || "",
    rewrite: url.searchParams.get("rewrite") || "",
    next: url.searchParams.get("next") || "/",
    traceId: url.searchParams.get("trace_id") || "",
    language: preferences.language,
    canonical: `${url.origin}/flow?step=${step}`,
  };
};
