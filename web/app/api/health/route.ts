import { getAuthApi } from "@/openapi";

export async function GET() {
  try {
    const api = getAuthApi();
    const data = await api.health();
    return Response.json({ source: "auth-api", data });
  } catch (error) {
    const message = error instanceof Error ? error.message : "unknown error";
    return Response.json(
      { source: "auth-api", error: message },
      { status: 502 },
    );
  }
}
