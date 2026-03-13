import { fail, redirect } from "@sveltejs/kit";
import { apiJson } from "$lib/server/api";
import { sanitizeInternalPath } from "$lib/server/redirects";

type Mode = "login" | "register";

type AuthContext = {
  authenticated?: boolean;
  email?: string | null;
};

async function postJson(fetch: typeof globalThis.fetch, path: string, payload: Record<string, unknown>) {
  return fetch(path, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(payload),
  });
}

export async function loadAuthPage(fetch: typeof globalThis.fetch, url: URL, mode: Mode) {
  const email = url.searchParams.get("email") || "";
  const rewrite = sanitizeInternalPath(url.searchParams.get("rewrite") || url.searchParams.get("return_to"));
  const verified = url.searchParams.get("verified") === "1";
  const { data } = await apiJson<AuthContext>(fetch, "/api/v1/context");

  return {
    mode,
    email,
    rewrite,
    verified,
    canonical: rewrite ? `${url.origin}/${mode}?rewrite=${encodeURIComponent(rewrite)}` : `${url.origin}/${mode}`,
    authContext: data || { authenticated: false, email: null },
  };
}

export const authActions = {
  login: async ({ fetch, request, url }: { fetch: typeof globalThis.fetch; request: Request; url: URL }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      return fail(400, { mode: "login" as const, message: "Email is required.", tone: "error" as const, email, rewrite });
    }

    const response = await postJson(fetch, "/api/v1/auth/login/email", {
      email,
      rewrite: rewrite || null,
    });

    if (!response.ok) {
      return fail(response.status, { mode: "login" as const, message: `Request failed with HTTP ${response.status}.`, tone: "error" as const, email, rewrite });
    }

    return { mode: "login" as const, message: "Check your inbox for your sign-in link.", tone: "success" as const, email, rewrite };
  },
  register: async ({ fetch, request, url }: { fetch: typeof globalThis.fetch; request: Request; url: URL }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const displayName = String(data.get("display_name") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      return fail(400, { mode: "register" as const, message: "Email is required.", tone: "error" as const, email, displayName, rewrite });
    }

    const response = await postJson(fetch, "/api/v1/auth/register/email", {
      email,
      display_name: displayName || null,
      rewrite: rewrite || null,
    });

    if (!response.ok) {
      return fail(response.status, { mode: "register" as const, message: `Request failed with HTTP ${response.status}.`, tone: "error" as const, email, displayName, rewrite });
    }

    return {
      mode: "register" as const,
      message: "Check your inbox for the verification email.",
      tone: "success" as const,
      email,
      displayName,
      rewrite,
      registrationRequested: true,
    };
  },
  resend: async ({ fetch, request, url }: { fetch: typeof globalThis.fetch; request: Request; url: URL }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const displayName = String(data.get("display_name") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      return fail(400, { mode: "register" as const, message: "Email is required.", tone: "error" as const, email, displayName, rewrite, registrationRequested: true });
    }

    const response = await postJson(fetch, "/api/v1/auth/verify/email/resend", {
      email,
      rewrite: rewrite || null,
    });

    if (!response.ok) {
      return fail(response.status, { mode: "register" as const, message: `Request failed with HTTP ${response.status}.`, tone: "error" as const, email, displayName, rewrite, registrationRequested: true });
    }

    return {
      mode: "register" as const,
      message: "Verification email sent again.",
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
