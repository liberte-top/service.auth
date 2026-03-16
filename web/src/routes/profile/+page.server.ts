import { redirect } from "@sveltejs/kit";
import { getAuthContext, getPreferences } from "$lib/server/auth-api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url, fetch }) => {
  const [{ data }, { data: preferences }] = await Promise.all([getAuthContext(fetch), getPreferences(fetch)]);
  const language = preferences.language;

  if (!data.authenticated) {
    throw redirect(303, "/");
  }

  return {
    email: data.email || "",
    language,
    canonical: `${url.origin}/profile`,
  };
};
