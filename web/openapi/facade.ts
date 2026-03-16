import type {
  AuthContextResponse,
  EmailActionAccepted,
  EmailOnlyRequest,
  PreferenceOptionsResponse,
  PreferencesResponse,
  RegisterEmailRequest,
  UpdatePreferencesRequest,
} from "./client";
import { fetch_json, type FetchLike } from "@liberte-top/shared/openapi";

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
        return fetch_json<AuthContextResponse>(fetchFn, {
          path: "/api/v1/auth/context",
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
