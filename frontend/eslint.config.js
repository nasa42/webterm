import eslintPluginAstro from "eslint-plugin-astro";
import tsEsLint from "typescript-eslint";

export default [
  {
    ignores: ["src/generated/flatbuffers_schema/"],
  },
  ...tsEsLint.configs.recommended,
  ...eslintPluginAstro.configs["flat/recommended"],
  {
    rules: {
      // override/add rules settings here, such as:
      // "astro/no-set-html-directive": "error"
    },
  },
];
