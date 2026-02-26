import { customInstance } from "./http";

export type HealthResponse = {
  status: string;
};

export type AccountPayload = {
  account_type: string;
  username: string | null;
  email: string | null;
  phone: string | null;
  created_by: string | null;
};

export type AccountResponse = {
  uid: string;
  account_type: string;
  username: string | null;
  email: string | null;
  phone: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
};

export function getHealth(): Promise<HealthResponse> {
  return customInstance<HealthResponse>({
    url: "/api/v1/health",
    method: "GET",
  });
}

export function createAccount(payload: AccountPayload): Promise<AccountResponse> {
  return customInstance<AccountResponse>({
    url: "/api/v1/accounts",
    method: "POST",
    data: payload,
  });
}

export function getAccount(uid: string): Promise<AccountResponse> {
  return customInstance<AccountResponse>({
    url: `/api/v1/accounts/${uid}`,
    method: "GET",
  });
}
