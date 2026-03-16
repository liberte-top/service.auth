import type { Actions, PageServerLoad } from "./$types";
import { fail } from "@sveltejs/kit";
import { getAuthContext, getPreferences, registerEmail, resendVerifyEmail } from "$lib/server/auth-api";
import { translate } from "$lib/i18n/copy";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const [{ data: authContext }, { data: preferences }] = await Promise.all([getAuthContext(fetch), getPreferences(fetch)]);
  const email = url.searchParams.get("email") || "";
  const rewrite = url.searchParams.get("rewrite") || url.searchParams.get("return_to") || "";
  const verified = url.searchParams.get("verified") === "1";

  return {
    mode: "register" as const,
    email,
    rewrite,
    verified,
    language: preferences.language,
    canonical: rewrite ? `${url.origin}/register?rewrite=${encodeURIComponent(rewrite)}` : `${url.origin}/register`,
    authContext,
  };
};

export const actions: Actions = {
  default: async ({ fetch, request, url }) => {
    const data = await request.formData();
    const intent = String(data.get("intent") || "register");
    const email = String(data.get("email") || "").trim();
    const displayName = String(data.get("display_name") || "").trim();
    const rewrite = String(data.get("rewrite") || url.searchParams.get("rewrite") || "").trim();
    const { data: preferences } = await getPreferences(fetch);
    const language = preferences.language;

    if (!email) {
      return fail(400, { mode: "register" as const, message: translate(language, "auth.common.emailRequired"), tone: "error" as const, email, displayName, rewrite });
    }

    const result = intent === "resend"
      ? await resendVerifyEmail(fetch, { email, rewrite: rewrite || null })
      : await registerEmail(fetch, { email, display_name: displayName || null, rewrite: rewrite || null });

    if (!result.response.ok) {
      return fail(result.response.status, { mode: "register" as const, message: translate(language, "auth.common.requestFailed", { status: result.response.status }), tone: "error" as const, email, displayName, rewrite, registrationRequested: true });
    }

    return {
      mode: "register" as const,
      message: translate(language, intent === "resend" ? "auth.register.resendSent" : "auth.register.emailSent"),
      tone: "success" as const,
      email,
      displayName,
      rewrite,
      registrationRequested: true,
    };
  },
};
