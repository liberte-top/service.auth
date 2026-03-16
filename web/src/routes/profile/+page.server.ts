import { ensure } from "@liberte-top/shared/ensure";
import { AppError } from "$lib/error";
import { openapi } from "$openapi";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url, fetch }) => {
  const api = openapi.create(fetch);
  const [{ data }, { data: preferences }] = await Promise.all([api.getAuthContext(), api.getPreferences()]);
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
