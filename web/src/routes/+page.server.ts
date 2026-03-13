import { fail } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
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

export const load: PageServerLoad = async ({ fetch, url }) => {
  const mode = url.searchParams.get("mode") === "register" ? "register" : "login";
  const email = url.searchParams.get("email") || "";
  const rewrite = sanitizeInternalPath(url.searchParams.get("rewrite") || url.searchParams.get("return_to"));
  const verified = url.searchParams.get("verified") === "1";
  const { data } = await apiJson<AuthContext>(fetch, "/api/v1/context");

  return {
    mode,
    email,
    rewrite,
    verified,
    canonical: rewrite ? `${url.origin}/?mode=${mode}&rewrite=${encodeURIComponent(rewrite)}` : `${url.origin}/?mode=${mode}`,
    authContext: data || { authenticated: false, email: null },
  };
};

export const actions: Actions = {
  login: async ({ fetch, request, url }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      return fail(400, { mode: "login" satisfies Mode, message: "Email is required.", tone: "error", email, rewrite });
    }

    const response = await postJson(fetch, "/api/v1/auth/login/email", {
      email,
      rewrite: rewrite || null,
    });

    if (!response.ok) {
      return fail(response.status, { mode: "login" satisfies Mode, message: `Request failed with HTTP ${response.status}.`, tone: "error", email, rewrite });
    }

    return { mode: "login" satisfies Mode, message: "Check your inbox for your sign-in link.", tone: "success", email, rewrite };
  },
  register: async ({ fetch, request, url }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const displayName = String(data.get("display_name") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      return fail(400, { mode: "register" satisfies Mode, message: "Email is required.", tone: "error", email, displayName, rewrite });
    }

    const response = await postJson(fetch, "/api/v1/auth/register/email", {
      email,
      display_name: displayName || null,
      rewrite: rewrite || null,
    });

    if (!response.ok) {
      return fail(response.status, { mode: "register" satisfies Mode, message: `Request failed with HTTP ${response.status}.`, tone: "error", email, displayName, rewrite });
    }

    return {
      mode: "register" satisfies Mode,
      message: "Check your inbox for the verification email.",
      tone: "success",
      email,
      displayName,
      rewrite,
      registrationRequested: true,
    };
  },
  resend: async ({ fetch, request, url }) => {
    const data = await request.formData();
    const email = String(data.get("email") || "").trim();
    const displayName = String(data.get("display_name") || "").trim();
    const rewrite = sanitizeInternalPath(String(data.get("rewrite") || url.searchParams.get("rewrite") || ""));

    if (!email) {
      return fail(400, { mode: "register" satisfies Mode, message: "Email is required.", tone: "error", email, displayName, rewrite, registrationRequested: true });
    }

    const response = await postJson(fetch, "/api/v1/auth/verify/email/resend", {
      email,
      rewrite: rewrite || null,
    });

    if (!response.ok) {
      return fail(response.status, { mode: "register" satisfies Mode, message: `Request failed with HTTP ${response.status}.`, tone: "error", email, displayName, rewrite, registrationRequested: true });
    }

    return {
      mode: "register" satisfies Mode,
      message: "Verification email sent again.",
      tone: "success",
      email,
      displayName,
      rewrite,
      registrationRequested: true,
    };
  },
};
