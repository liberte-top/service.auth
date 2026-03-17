import { config } from "$lib/config";
import { openapi } from "$openapi";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch }) => {
  const api = openapi.create(fetch);
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
