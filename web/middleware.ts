import { NextResponse } from "next/server";

export function middleware(request: Request) {
  const url = new URL(request.url);
  if (url.pathname === "/__health__") {
    return NextResponse.json({
      status: "ok",
      service: "auth-web",
      timestamp: new Date().toISOString(),
    });
  }
  return NextResponse.next();
}

export const config = {
  matcher: ["/__health__"],
};
