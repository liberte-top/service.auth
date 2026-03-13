import type { Actions, PageServerLoad } from "./$types";
import { authActions, loadAuthPage } from "$lib/server/auth";

export const load: PageServerLoad = async ({ fetch, url }) => loadAuthPage(fetch, url, "register");

export const actions: Actions = {
  register: authActions.register,
  resend: authActions.resend,
};
