import type { R2fRoot } from "../../../generated/flatbuffers_schema/talk_v1/r2f-root.ts";
import type { SendPayload } from "../models/SendPayload.ts";
import { R2fFromAgent, R2fRootPayload } from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { processA2f } from "./processA2f.ts";
import { readA2fRoot } from "../parsers/readA2fRoot.ts";

export const processR2f = async (relayRoot: R2fRoot, send: SendPayload) => {
  switch (relayRoot.rootPayloadType()) {
    case R2fRootPayload.FromAgent:
      const fromAgent = relayRoot.rootPayload(new R2fFromAgent()) as R2fFromAgent | null;
      const data = fromAgent?.payloadArray();
      if (!data) return;
      const payload = readA2fRoot(data);
      await processA2f(payload, send);
      return;
    default:
      console.error("Unknown relay root payload type:", relayRoot.rootPayloadType());
  }
};
