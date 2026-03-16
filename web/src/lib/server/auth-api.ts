import type {
  AuthContextResponse,
  EmailActionAccepted,
  EmailOnlyRequest,
  PreferencesResponse,
  RegisterEmailRequest,
} from "$openapi/client";

type RequestResult<T> = {
  response: Response;
  data: T;
};

type RequestOptions = {
  method?: string;
  body?: unknown;
};

async function requestJson<T>(fetchFn: typeof fetch, path: string, options: RequestOptions = {}): Promise<RequestResult<T>> {
  const response = await fetchFn(path, {
    method: options.method || "GET",
    headers: options.body ? { "content-type": "application/json" } : undefined,
    body: options.body ? JSON.stringify(options.body) : undefined,
  });

  const data = (await response.json()) as T;
  return { response, data };
}

export async function getPreferences(fetchFn: typeof fetch) {
  return requestJson<PreferencesResponse>(fetchFn, "/api/v1/preferences");
}

export async function getAuthContext(fetchFn: typeof fetch) {
  return requestJson<AuthContextResponse>(fetchFn, "/api/v1/auth/context");
}

export async function requestEmailLogin(fetchFn: typeof fetch, payload: EmailOnlyRequest) {
  return requestJson<EmailActionAccepted>(fetchFn, "/api/v1/auth/login/email", {
    method: "POST",
    body: payload,
  });
}

export async function registerEmail(fetchFn: typeof fetch, payload: RegisterEmailRequest) {
  return requestJson<EmailActionAccepted>(fetchFn, "/api/v1/auth/register/email", {
    method: "POST",
    body: payload,
  });
}

export async function resendVerifyEmail(fetchFn: typeof fetch, payload: EmailOnlyRequest) {
  return requestJson<EmailActionAccepted>(fetchFn, "/api/v1/auth/verify/email/resend", {
    method: "POST",
    body: payload,
  });
}
