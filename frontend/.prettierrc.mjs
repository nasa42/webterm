/** @type {import("prettier").Config} */

export default {
  plugins: ["prettier-plugin-astro"],
  overrides: [
    {
      files: "*.astro",
      options: {
        parser: "astro",
      },
    },
  ],

  printWidth: 120,
  semi: true,
  endOfLine: "lf",
  singleQuote: false,
  tabWidth: 2,
  trailingComma: "all",
  useTabs: false,
  arrowParens: "always",
  quoteProps: "consistent",
  bracketSpacing: true,
};
