import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url }) => {
  const rewrite = url.searchParams.get("rewrite") || url.searchParams.get("return_to") || "";
  return {
    loginHref: rewrite ? `/login?rewrite=${encodeURIComponent(rewrite)}` : "/login",
  };
};
