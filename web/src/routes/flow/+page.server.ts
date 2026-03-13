import type { PageServerLoad } from "./$types";
import { sanitizeInternalPath } from "$lib/server/redirects";

export type FlowStep = "verify-success" | "verify-invalid" | "login-success" | "login-invalid";

export const load: PageServerLoad = async ({ url }) => {
  const requestedStep = url.searchParams.get("step");
  const step: FlowStep =
    requestedStep === "verify-invalid" || requestedStep === "login-success" || requestedStep === "login-invalid"
      ? requestedStep
      : "verify-success";

  return {
    step,
    email: url.searchParams.get("email") || "",
    rewrite: sanitizeInternalPath(url.searchParams.get("rewrite")),
    next: sanitizeInternalPath(url.searchParams.get("next")) || "/",
    canonical: `${url.origin}/flow?step=${step}`,
  };
};
