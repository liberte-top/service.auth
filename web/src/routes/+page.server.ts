import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url }) => {
  const rewrite = url.searchParams.get("rewrite") || url.searchParams.get("return_to") || "";
  throw new Response(null, {
    status: 303,
    headers: {
      location: rewrite ? `/login?rewrite=${encodeURIComponent(rewrite)}` : "/login",
    },
  });
};
