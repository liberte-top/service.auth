import { config } from "$lib/config";
import { openapi } from "$openapi";
import type { FetchLike } from "@liberte-top/shared/openapi";

export function createServerApi(fetchFn: FetchLike, requestHeaders: Headers) {
  return openapi.create(async (input, init) => {
    const url = typeof input === "string" ? input : String(input);

    if (!url.startsWith("/api/")) {
      return fetchFn(input, init);
    }

    const headers = new Headers(init?.headers);
    const cookie = requestHeaders.get("cookie");
    const authorization = requestHeaders.get("authorization");

    if (cookie && !headers.has("cookie")) {
      headers.set("cookie", cookie);
    }

    if (authorization && !headers.has("authorization")) {
      headers.set("authorization", authorization);
    }

    return fetch(`${config.api.internalOrigin}${url}`, {
      ...init,
      headers,
      redirect: "manual",
    });
  });
}
