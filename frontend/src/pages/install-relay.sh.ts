import type { APIRoute } from "astro";
import ejs from "ejs";

export const GET: APIRoute = async (_context): Promise<Response> => {
  const script = await ejs.renderFile("./src/templates/install.sh.ejs", { binary_name: "webterm-relay" });
  return new Response(script);
};
