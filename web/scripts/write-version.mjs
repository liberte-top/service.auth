import { mkdir, writeFile } from "node:fs/promises";

const version = process.env.npm_package_version ?? "0.0.0";
const gitSha = process.env.APP_BUILD_SHA ?? null;
const builtAt = process.env.APP_BUILD_TIMESTAMP ?? null;

await mkdir("public", { recursive: true });
await writeFile(
  "public/version.json",
  JSON.stringify(
    {
      version,
      git_sha: gitSha,
      built_at: builtAt,
    },
    null,
    2,
  ) + "\n",
  "utf8",
);
