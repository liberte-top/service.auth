import type {
  AuthContextResponse,
  EmailActionAccepted,
  EmailOnlyRequest,
  PreferenceOptionsResponse,
  PreferencesResponse,
  RegisterEmailRequest,
  UpdatePreferencesRequest,
} from "./client";
import type { AuthContext, AuthScopeDefinition } from "@liberte-top/shared/auth";
import { fetch_json, type FetchLike } from "@liberte-top/shared/openapi";

export type LocalAuthContextResponse = AuthContext<"user" | "team" | "robot", "session" | "api_key">;

export type SelfProfileResponse = {
  subject: string;
  principal_type: string;
  email?: string | null;
  email_verified: boolean;
  display_name?: string | null;
  scopes: string[];
};

export type UpdateSelfProfileRequest = {
  display_name?: string | null;
};

export type ApiTokenSummary = {
  id: number;
  name: string;
  prefix: string;
  created_at: string;
  last_used_at?: string | null;
  expires_at?: string | null;
  revoked_at?: string | null;
};

export type ApiTokenSecret = {
  token: string;
  summary: ApiTokenSummary;
};

export type CreateApiTokenRequest = {
  name: string;
  expires_at?: string | null;
};

export const openapi = {
  create(fetchFn: FetchLike) {
    return {
      getPreferences() {
        return fetch_json<PreferencesResponse>(fetchFn, {
          path: "/api/v1/preferences",
        });
      },

      getPreferenceOptions() {
        return fetch_json<PreferenceOptionsResponse>(fetchFn, {
          path: "/api/v1/preferences/options",
        });
      },

      updatePreferences(payload: UpdatePreferencesRequest) {
        return fetch_json<PreferencesResponse>(fetchFn, {
          path: "/api/v1/preferences",
          method: "POST",
          body: payload,
        });
      },

      getAuthContext() {
        return fetch_json<LocalAuthContextResponse>(fetchFn, {
          path: "/api/v1/auth/context",
        });
      },

      getScopeCatalog() {
        return fetch_json<AuthScopeDefinition[]>(fetchFn, {
          path: "/api/v1/auth/scopes",
        });
      },

      getSelfProfile() {
        return fetch_json<SelfProfileResponse>(fetchFn, {
          path: "/api/v1/self/profile",
        });
      },

      updateSelfProfile(payload: UpdateSelfProfileRequest) {
        return fetch_json<SelfProfileResponse>(fetchFn, {
          path: "/api/v1/self/profile",
          method: "PATCH",
          body: payload,
        });
      },

      listSelfTokens() {
        return fetch_json<ApiTokenSummary[]>(fetchFn, {
          path: "/api/v1/self/tokens",
        });
      },

      createSelfToken(payload: CreateApiTokenRequest) {
        return fetch_json<ApiTokenSecret>(fetchFn, {
          path: "/api/v1/self/tokens",
          method: "POST",
          body: payload,
        });
      },

      revokeSelfToken(id: number) {
        return fetch_json<ApiTokenSummary>(fetchFn, {
          path: `/api/v1/self/tokens/${id}`,
          method: "DELETE",
        });
      },

      requestEmailLogin(payload: EmailOnlyRequest) {
        return fetch_json<EmailActionAccepted>(fetchFn, {
          path: "/api/v1/auth/login/email",
          method: "POST",
          body: payload,
        });
      },

      registerEmail(payload: RegisterEmailRequest) {
        return fetch_json<EmailActionAccepted>(fetchFn, {
          path: "/api/v1/auth/register/email",
          method: "POST",
          body: payload,
        });
      },

      resendVerifyEmail(payload: EmailOnlyRequest) {
        return fetch_json<EmailActionAccepted>(fetchFn, {
          path: "/api/v1/auth/verify/email/resend",
          method: "POST",
          body: payload,
        });
      },
    };
  },
};
