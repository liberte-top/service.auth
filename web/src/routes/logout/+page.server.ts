import type { PageServerLoad } from "./$types";
import { clearAuthCookie } from "$lib/server/cookies";

export const load: PageServerLoad = async ({ cookies }) => {
  clearAuthCookie(cookies);

  return {
    success: true,
  };
};
