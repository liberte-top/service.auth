import type { Actions, PageServerLoad } from "./$types";
import { fail } from "@sveltejs/kit";
import { openapi } from "$openapi";
import { translate } from "$lib/i18n/copy";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const api = openapi.create(fetch);
  const [{ data: authContext }, { data: preferences }] = await Promise.all([api.getAuthContext(), api.getPreferences()]);
  const email = url.searchParams.get("email") || "";
  const rewrite = url.searchParams.get("rewrite") || url.searchParams.get("return_to") || "";
  const verified = url.searchParams.get("verified") === "1";

  return {
    mode: "login" as const,
    email,
    rewrite,
    verified,
    language: preferences.language,
    canonical: rewrite ? `${url.origin}/login?rewrite=${encodeURIComponent(rewrite)}` : `${url.origin}/login`,
    authContext,
  };
};

export const actions: Actions = {
  default: async ({ fetch, request, url }) => {
    const api = openapi.create(fetch);
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const rewrite = String(data.get("rewrite") || url.searchParams.get("rewrite") || "").trim();
    const { data: preferences } = await api.getPreferences();
    const language = preferences.language;

    if (!email) {
      return fail(400, { mode: "login" as const, message: translate(language, "auth.common.emailRequired"), tone: "error" as const, email, rewrite });
    }

    const result = await api.requestEmailLogin({ email, rewrite: rewrite || null });
    if (!result.response.ok) {
      return fail(result.response.status, { mode: "login" as const, message: translate(language, "auth.common.requestFailed", { status: result.response.status }), tone: "error" as const, email, rewrite });
    }

    return { mode: "login" as const, message: translate(language, "auth.login.emailSent"), tone: "success" as const, email, rewrite };
  },
};
