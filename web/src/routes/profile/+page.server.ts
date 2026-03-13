import { redirect } from "@sveltejs/kit";
import { languageHeader, languageFromCookies } from "$lib/i18n/server";
import type { PageServerLoad } from "./$types";
import { apiJson } from "$lib/server/api";

type AuthContext = {
  authenticated?: boolean;
  email?: string | null;
};

export const load: PageServerLoad = async ({ fetch, url, cookies }) => {
  const language = languageFromCookies(cookies);
  const { data } = await apiJson<AuthContext>(fetch, "/api/v1/context", {
    headers: languageHeader(language),
  });

  if (!data?.authenticated) {
    throw redirect(303, "/");
  }

  return {
    email: data.email || "",
    language,
    canonical: `${url.origin}/profile`,
  };
};
