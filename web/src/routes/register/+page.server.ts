import type { Actions, PageServerLoad } from "./$types";
import { authActions, loadAuthPage } from "$lib/server/auth";

export const load: PageServerLoad = async ({ fetch, url, cookies }) => loadAuthPage(fetch, url, "register", cookies);

export const actions: Actions = {
  default: authActions.register,
};
