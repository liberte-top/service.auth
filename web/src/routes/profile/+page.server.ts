import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { apiJson } from "$lib/server/api";

type AuthContext = {
  authenticated?: boolean;
  email?: string | null;
};

export const load: PageServerLoad = async ({ fetch, url }) => {
  const { data } = await apiJson<AuthContext>(fetch, "/api/v1/context");

  if (!data?.authenticated) {
    throw redirect(303, "/");
  }

  return {
    email: data.email || "",
    canonical: `${url.origin}/profile`,
  };
};
