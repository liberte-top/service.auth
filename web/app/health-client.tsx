"use client";

import { useEffect, useState } from "react";

import { getAuthApi } from "@/openapi";
import type { Health } from "@/openapi/models";

export function HealthClient() {
  const [data, setData] = useState<Health | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const api = getAuthApi();
    api
      .health()
      .then((response) => {
        setData(response);
      })
      .catch((err: unknown) => {
        setError(err instanceof Error ? err.message : "unknown error");
      });
  }, []);

  return (
    <div className="rounded-xl border border-zinc-200 bg-white p-4 text-sm text-zinc-700">
      <div className="text-xs uppercase tracking-[0.2em] text-zinc-500">
        CSR example
      </div>
      <div className="mt-2">
        <span className="text-zinc-500">client health</span>: {data?.status ?? "pending"}
      </div>
      {error ? (
        <div className="mt-1 text-xs text-red-600">
          error: {error}
        </div>
      ) : null}
    </div>
  );
}
