// @ts-check
import { defineConfig } from "astro/config";
import { webtermVersionPlugin } from "./src/scripts/build/webterm-version-plugin";

import icon from "astro-icon";

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [webtermVersionPlugin()],
  },

  integrations: [icon()],
});
