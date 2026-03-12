import { env } from "$env/dynamic/private";

const DEFAULT_DEV_API_URL = "http://localhost:3333";
const DEFAULT_DOCKER_API_URL = "http://auth-api:3333";

export function apiOrigin() {
  return env.AUTH_API_INTERNAL_URL || env.WEB_VITE_AUTH_API_BASE_URL || (env.NODE_ENV === "development" ? DEFAULT_DEV_API_URL : DEFAULT_DOCKER_API_URL);
}

export async function apiJson<T>(fetchFn: typeof fetch, input: string, init?: RequestInit) {
  const response = await fetchFn(input, init);
  let data: T | null = null;

  try {
    data = (await response.json()) as T;
  } catch {
    data = null;
  }

  return { response, data };
}
