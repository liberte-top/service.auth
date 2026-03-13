import type { PageServerLoad } from "./$types";
import { redirectToLogin } from "$lib/server/auth";

export const load: PageServerLoad = async ({ url }) => {
  redirectToLogin(url);
};
