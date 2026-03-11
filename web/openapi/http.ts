import type { AxiosRequestConfig } from "axios";
import { createLibertEAuthAllowRequest, createUnauthorizedRedirectHandler } from "@liberte-top/shared/auth";
import { createOpenApiClient } from "@liberte-top/shared/openapi";

const { instance: apiClient, request } = createOpenApiClient({
  onUnauthorized: createUnauthorizedRedirectHandler({
    loginUrl: "/",
    allowRequest: createLibertEAuthAllowRequest,
  }),
});

export { apiClient };

export async function customInstance<T>(config: AxiosRequestConfig): Promise<T> {
  return request<T>(config);
}
