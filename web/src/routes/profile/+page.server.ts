import { redirect } from "@sveltejs/kit";
import type { PreferencesResponse } from "$openapi/client";
import type { PageServerLoad } from "./$types";
import { apiJson } from "$lib/server/api";

type AuthContext = {
  authenticated?: boolean;
  email?: string | null;
};

export const load: PageServerLoad = async ({ fetch, url, cookies }) => {
  const { data } = await apiJson<AuthContext>(fetch, "/api/v1/context");
  const preferences = await apiJson<PreferencesResponse>(fetch, "/api/v1/preferences");
  const language = preferences.data?.language || "en";

  if (!data?.authenticated) {
    throw redirect(303, "/");
  }

  return {
    email: data.email || "",
    language,
    canonical: `${url.origin}/profile`,
  };
};
