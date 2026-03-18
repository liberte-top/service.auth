import { createServerApi } from "$lib/server/api";
import type { PageServerLoad } from "./$types";

export type FlowStep = "verify-success" | "verify-invalid" | "login-success" | "login-invalid";

export const load: PageServerLoad = async ({ url, fetch, request }) => {
  const api = createServerApi(fetch, request.headers);
  const requestedStep = url.searchParams.get("step");
  const step: FlowStep =
    requestedStep === "verify-invalid" || requestedStep === "login-success" || requestedStep === "login-invalid"
      ? requestedStep
      : "verify-success";

  const { data: preferences } = await api.getPreferences();
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
