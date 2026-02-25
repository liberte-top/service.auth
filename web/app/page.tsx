import { getAuthApi } from "@/openapi";
import { HealthClient } from "./health-client";

export default async function Home() {
  let ssrHealth: string | null = null;
  try {
    const api = getAuthApi();
    const data = await api.health();
    ssrHealth = data?.status ?? null;
  } catch {
    ssrHealth = null;
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans">
      <main className="flex w-full max-w-3xl flex-col gap-6 rounded-2xl border border-zinc-200 bg-white p-10 shadow-sm">
        <div className="text-xs uppercase tracking-[0.2em] text-zinc-500">
          Auth Web
        </div>
        <h1 className="text-3xl font-semibold tracking-tight text-zinc-900">
          Next.js smoke is live
        </h1>
        <p className="text-base leading-7 text-zinc-600">
          This is a minimal health surface for auth-web. Use it to verify
          routing, TLS, and rollout status.
        </p>
        <div className="grid grid-cols-1 gap-3 rounded-xl border border-zinc-200 bg-zinc-50 p-4 text-sm text-zinc-700">
          <div>
            <span className="text-zinc-500">env</span>:{" "}
            {process.env.NEXT_PUBLIC_ENV ?? "unknown"}
          </div>
          <div>
            <span className="text-zinc-500">ssr health</span>:{" "}
            {ssrHealth ?? "unavailable"}
          </div>
          <div>
            <span className="text-zinc-500">health</span>: /__health__
          </div>
        </div>
        <HealthClient />
        <div className="rounded-xl border border-dashed border-zinc-200 bg-zinc-50 p-4 text-sm text-zinc-700">
          <span className="text-zinc-500">api route</span>: /api/health
        </div>
      </main>
    </div>
  );
}
