import { defineConfig } from "orval";

export default defineConfig({
  auth: {
    input: {
      target: "https://auth.liberte.top/api/openapi.json",
    },
    output: {
      target: "openapi/client.ts",
      schemas: "openapi/models",
      client: "axios",
      override: {
        mutator: {
          path: "./openapi/http.ts",
          name: "customInstance",
        },
      },
    },
  },
});
