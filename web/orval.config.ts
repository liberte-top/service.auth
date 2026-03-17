import { defineConfig } from "orval";

export default defineConfig({
  auth: {
    input: {
      target: "http://localhost:3333/api/openapi.json",
    },
    output: {
      target: "openapi/client.ts",
      client: "fetch",
    },
  },
});
