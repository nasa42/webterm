import eslintPluginAstro from "eslint-plugin-astro";
import tsEsLint from "typescript-eslint";

export default [
  {
    ignores: [".astro/", "src/env.d.ts", "src/generated/flatbuffers_schema/"],
  },
  ...tsEsLint.configs.recommended,
  ...eslintPluginAstro.configs["flat/recommended"],
  {
    rules: {
      // override/add rules settings here, such as:
      // "astro/no-set-html-directive": "error"
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          args: "after-used",
          argsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
          destructuredArrayIgnorePattern: "^_",
          varsIgnorePattern: "^_",
        },
      ],
    },
  },
];
