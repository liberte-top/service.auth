import type { Actions, PageServerLoad } from "./$types";
import { authActions, loadAuthPage } from "$lib/server/auth";

export const load: PageServerLoad = async ({ fetch, url }) => loadAuthPage(fetch, url, "login");

export const actions: Actions = {
  login: authActions.login,
};
