import { createServerApi } from "$lib/server/api";
import { config } from "$lib/config";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, request }) => {
  const api = createServerApi(fetch, request.headers);
  const [{ data: preferences }, { data: preferenceOptions }] = await Promise.all([
    api.getPreferences(),
    api.getPreferenceOptions(),
  ]);
  return {
    buildInfo: config.buildInfo,
    preferences: preferences,
    preferenceOptions,
  };
};
