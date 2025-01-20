// @ts-check
import { defineConfig } from "astro/config";
import { webtermVersionPlugin } from "./src/scripts/build/webterm-version-plugin";

import icon from "astro-icon";

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [webtermVersionPlugin()],
    build: {
      minify: import.meta.env.APP_ENABLE_MINIFY === "true",
    },
  },

  integrations: [icon()],
});
