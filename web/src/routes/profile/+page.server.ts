import { fail, redirect } from "@sveltejs/kit";
import { openapi } from "$openapi";
import { translate } from "$lib/i18n/copy";
import type { Actions, PageServerLoad } from "./$types";

function parseOptionalDateTime(value: string): string | null | symbol {
  if (!value) {
    return null;
  }

  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return Symbol.for("invalid-datetime");
  }

  return parsed.toISOString();
}

export const load: PageServerLoad = async ({ url, fetch }) => {
  const api = openapi.create(fetch);
  const [{ data: authContext }, { data: preferences }] = await Promise.all([api.getAuthContext(), api.getPreferences()]);

  if (!authContext.authenticated) {
    throw redirect(303, "/");
  }

  const [{ data: profile }, { data: tokens }, { data: scopeCatalog }] = await Promise.all([
    api.getSelfProfile(),
    api.listSelfTokens(),
    api.getScopeCatalog(),
  ]);

  return {
    authContext,
    profile,
    scopeCatalog,
    tokens,
    language: preferences.language,
    canonical: `${url.origin}/profile`,
  };
};

export const actions: Actions = {
  updateProfile: async ({ fetch, request }) => {
    const api = openapi.create(fetch);
    const data = await request.formData();
    const displayName = String(data.get("display_name") || "").trim();
    const { data: preferences } = await api.getPreferences();
    const language = preferences.language;
    const result = await api.updateSelfProfile({
      display_name: displayName || null,
    });

    if (!result.response.ok) {
      return fail(result.response.status, {
        kind: "profile",
        tone: "error" as const,
        message: translate(language, "auth.profile.updateError", { status: result.response.status }),
        displayName,
      });
    }

    return {
      kind: "profile",
      tone: "success" as const,
      message: translate(language, "auth.profile.updateSuccess"),
      displayName: result.data.display_name || "",
    };
  },

  createToken: async ({ fetch, request }) => {
    const api = openapi.create(fetch);
    const data = await request.formData();
    const name = String(data.get("name") || "").trim();
    const expiresAtRaw = String(data.get("expires_at") || "").trim();
    const [{ data: preferences }, { data: profile }] = await Promise.all([api.getPreferences(), api.getSelfProfile()]);
    const language = preferences.language;
    const grantedScopes = profile.scopes;
    const grantedScopeSet = new Set(grantedScopes);
    const selectedScopes = Array.from(
      new Set(data.getAll("scopes").map((value) => String(value).trim()).filter((scope) => grantedScopeSet.has(scope))),
    );

    if (!name) {
      return fail(400, {
        kind: "token-create",
        tone: "error" as const,
        message: translate(language, "auth.tokens.nameRequired"),
        name,
        expiresAt: expiresAtRaw,
        scopes: selectedScopes,
      });
    }

    const expiresAt = parseOptionalDateTime(expiresAtRaw);
    if (expiresAt === Symbol.for("invalid-datetime")) {
      return fail(400, {
        kind: "token-create",
        tone: "error" as const,
        message: translate(language, "auth.tokens.invalidExpiry"),
        name,
        expiresAt: expiresAtRaw,
        scopes: selectedScopes,
      });
    }

    const result = await api.createSelfToken({
      name,
      expires_at: expiresAt,
      scopes: selectedScopes,
    });

    if (!result.response.ok) {
      return fail(result.response.status, {
        kind: "token-create",
        tone: "error" as const,
        message: translate(language, "auth.tokens.createError", { status: result.response.status }),
        name,
        expiresAt: expiresAtRaw,
        scopes: selectedScopes,
      });
    }

    return {
      kind: "token-create",
      tone: "success" as const,
      message: translate(language, "auth.tokens.createSuccess"),
      name: "",
      expiresAt: "",
      scopes: grantedScopes,
      createdToken: result.data,
    };
  },

  revokeToken: async ({ fetch, request }) => {
    const api = openapi.create(fetch);
    const data = await request.formData();
    const id = Number(String(data.get("id") || "").trim());
    const { data: preferences } = await api.getPreferences();
    const language = preferences.language;

    if (!Number.isInteger(id) || id <= 0) {
      return fail(400, {
        kind: "token-revoke",
        tone: "error" as const,
        message: translate(language, "auth.tokens.revokeInvalid"),
      });
    }

    const result = await api.revokeSelfToken(id);

    if (!result.response.ok) {
      return fail(result.response.status, {
        kind: "token-revoke",
        tone: "error" as const,
        message: translate(language, "auth.tokens.revokeError", { status: result.response.status }),
      });
    }

    return {
      kind: "token-revoke",
      tone: "success" as const,
      message: translate(language, "auth.tokens.revokeSuccess"),
      revokedTokenId: id,
    };
  },
};
