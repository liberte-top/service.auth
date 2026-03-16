import { ensure } from "@liberte-top/shared/ensure";
import { AppError } from "$lib/error";
import type { RequestHandler } from "./$types";

function apiOrigin() {
  return ensure.nonEmpty(process.env.AUTH_API_INTERNAL_URL || process.env.WEB_VITE_AUTH_API_BASE_URL, () => AppError.missingApiOrigin());
}

async function proxy({ fetch, params, request, url }: Parameters<RequestHandler>[0]) {
  const target = new URL(`/api/${params.path}`, apiOrigin());
  target.search = url.search;

  const headers = new Headers(request.headers);
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
