import { sveltekit } from "@sveltejs/kit/vite";
import biomePlugin from "vite-plugin-biome";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    sveltekit(),
    biomePlugin({
      mode: "format",
      logKind: "compact",
      files: "src/",
    }),
  ],
});
