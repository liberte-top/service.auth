import { config } from "$lib/config";
import { redirect } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ fetch, url }) => {
  const target = new URL(`${config.api.internalOrigin}/api/v1/auth/verify/email`);
  target.search = url.search;

  const response = await fetch(target, { redirect: "manual" });
  const location = response.headers.get("location");

  if (location) {
    throw redirect(response.status === 303 ? 303 : 302, location);
  }

  return new Response(await response.text(), {
    status: response.status,
    headers: response.headers,
  });
};
