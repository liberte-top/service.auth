import { fail, redirect } from "@sveltejs/kit";
import { translate } from "$lib/i18n/copy";
import { apiJson } from "$lib/server/api";
import { languageFromCookies, languageHeader } from "$lib/i18n/server";
import { sanitizeInternalPath } from "$lib/server/redirects";

type Mode = "login" | "register";

type AuthContext = {
  authenticated?: boolean;
  email?: string | null;
};

export async function loadAuthPage(fetch: typeof globalThis.fetch, url: URL, mode: Mode, cookies: import("@sveltejs/kit").Cookies) {
  const email = url.searchParams.get("email") || "";
  const rewrite = sanitizeInternalPath(url.searchParams.get("rewrite") || url.searchParams.get("return_to"));
  const verified = url.searchParams.get("verified") === "1";
  const language = languageFromCookies(cookies);
  const { data } = await apiJson<AuthContext>(fetch, "/api/v1/context", {
    headers: languageHeader(language),
  });

  return {
    mode,
    email,
    rewrite,
    verified,
    language,
    canonical: rewrite ? `${url.origin}/${mode}?rewrite=${encodeURIComponent(rewrite)}` : `${url.origin}/${mode}`,
    authContext: data || { authenticated: false, email: null },
  };
}

export const authActions = {
  login: async ({ fetch, request, url, cookies }: { fetch: typeof globalThis.fetch; request: Request; url: URL; cookies: import("@sveltejs/kit").Cookies }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      const language = languageFromCookies(cookies);
      return fail(400, { mode: "login" as const, message: translate(language, "auth.common.emailRequired"), tone: "error" as const, email, rewrite });
    }

    const language = languageFromCookies(cookies);
    const responseWithLanguage = await fetch("/api/v1/auth/login/email", {
      method: "POST",
      headers: {
        "content-type": "application/json",
        ...languageHeader(language),
      },
      body: JSON.stringify({
        email,
        rewrite: rewrite || null,
      }),
    });

    if (!responseWithLanguage.ok) {
      return fail(responseWithLanguage.status, { mode: "login" as const, message: translate(language, "auth.common.requestFailed", { status: responseWithLanguage.status }), tone: "error" as const, email, rewrite });
    }

    return { mode: "login" as const, message: translate(language, "auth.login.emailSent"), tone: "success" as const, email, rewrite };
  },
  register: async ({ fetch, request, url, cookies }: { fetch: typeof globalThis.fetch; request: Request; url: URL; cookies: import("@sveltejs/kit").Cookies }) => {
    const data = await request.formData();
    const intent = String(data.get("intent") || "register");
    const email = String(data.get("email") || "").trim();
    const displayName = String(data.get("display_name") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      const language = languageFromCookies(cookies);
      return fail(400, { mode: "register" as const, message: translate(language, "auth.common.emailRequired"), tone: "error" as const, email, displayName, rewrite });
    }

    const language = languageFromCookies(cookies);
    const path = intent === "resend" ? "/api/v1/auth/verify/email/resend" : "/api/v1/auth/register/email";
    const body = intent === "resend"
      ? {
          email,
          rewrite: rewrite || null,
        }
      : {
          email,
          display_name: displayName || null,
          rewrite: rewrite || null,
        };

    const response = await fetch(path, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        ...languageHeader(language),
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      return fail(response.status, { mode: "register" as const, message: translate(language, "auth.common.requestFailed", { status: response.status }), tone: "error" as const, email, displayName, rewrite, registrationRequested: true });
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

export function redirectToLogin(url: URL) {
  const rewrite = sanitizeInternalPath(url.searchParams.get("rewrite") || url.searchParams.get("return_to"));
  throw redirect(303, rewrite ? `/login?rewrite=${encodeURIComponent(rewrite)}` : "/login");
}
