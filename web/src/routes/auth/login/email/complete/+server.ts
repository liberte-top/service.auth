import { config } from "$lib/config";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ fetch, url, cookies }) => {
  const target = new URL(`${config.api.internalOrigin}/api/v1/auth/login/email/complete`);
  target.search = url.search;

  const response = await fetch(target, { redirect: "manual" });
  const headers = new Headers();
  const location = response.headers.get("location");
  const setCookie = response.headers.get("set-cookie");

  if (location) {
    headers.set("location", location);
  }

  if (setCookie) {
    headers.append("set-cookie", setCookie);
  }

  return new Response(null, {
    status: response.status,
    headers,
  });
};
