import { ensure } from "@liberte-top/shared/ensure";
import { AppError } from "$lib/error";
import { redirect } from "@sveltejs/kit";
import { getAuthContext, getPreferences } from "$lib/server/auth-api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url, fetch }) => {
  const [{ data }, { data: preferences }] = await Promise.all([getAuthContext(fetch), getPreferences(fetch)]);
  const language = preferences.language;

  if (!data.authenticated) {
    throw redirect(303, "/");
  }

  const email = ensure.nonEmpty(data.email, () => AppError.authenticatedEmailMissing());

  return {
    email,
    language,
    canonical: `${url.origin}/profile`,
  };
};
