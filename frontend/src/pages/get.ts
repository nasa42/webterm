import type { APIRoute } from "astro";
import { readFileSync } from "fs";

export const GET: APIRoute = (_context): Response => {
  const script = readFileSync("./get.sh");
  return new Response(script);
};
