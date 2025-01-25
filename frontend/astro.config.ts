import { defineConfig } from "astro/config";
import { webtermVersionPlugin } from "./src/scripts/build/webterm-version-plugin";

import react from "@astrojs/react";
import vitePluginSvgr from "vite-plugin-svgr";

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [webtermVersionPlugin(), vitePluginSvgr()],
    build: {
      minify: import.meta.env.APP_ENABLE_MINIFY === "true",
    },
  },

  integrations: [react()],
  experimental: {
    svg: true,
  },
});
