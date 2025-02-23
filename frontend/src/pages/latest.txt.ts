import type { APIRoute } from "astro";
import { readFileSync } from "fs";

export const GET: APIRoute = (_context): Response => {
  const content = readFileSync("./latest.txt", "utf-8");
  return new Response(content);
};
