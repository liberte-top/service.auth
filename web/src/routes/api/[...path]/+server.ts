import { apiOrigin } from "$lib/server/api";
import { languageFromCookies } from "$lib/i18n/server";
import { LIBERTE_LANGUAGE_HEADER } from "$lib/i18n/shared";
import type { RequestHandler } from "./$types";

async function proxy({ fetch, params, request, url, cookies }: Parameters<RequestHandler>[0]) {
  const target = new URL(`/api/${params.path}`, apiOrigin());
  target.search = url.search;

  const headers = new Headers(request.headers);
  headers.set(LIBERTE_LANGUAGE_HEADER, headers.get(LIBERTE_LANGUAGE_HEADER) || languageFromCookies(cookies));
  headers.delete("host");
  headers.delete("connection");
  headers.delete("content-length");

  const response = await fetch(target, {
    method: request.method,
    headers,
    body: request.method === "GET" || request.method === "HEAD" ? undefined : await request.arrayBuffer(),
    redirect: "manual",
  });

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: new Headers(response.headers),
  });
}

export const GET: RequestHandler = proxy;
export const POST: RequestHandler = proxy;
export const PUT: RequestHandler = proxy;
export const PATCH: RequestHandler = proxy;
export const DELETE: RequestHandler = proxy;
export const OPTIONS: RequestHandler = proxy;
