import type { APIRoute } from "astro";
import { readFileSync } from "fs";

export const GET: APIRoute = (_context): Response => {
  const file = readFileSync("../Cargo.toml", "utf-8");
  const versionMatch = file.match(/^\[workspace\.package\][\s\S]*?version\s*=\s*["'](.+?)["']/m);

  const version = versionMatch?.[1];

  if (!version) {
    throw new Error("Could not find version in Cargo.toml");
  }

  return new Response(version);
};
