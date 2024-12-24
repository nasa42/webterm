import { PtyOutput, type PtyOutputRoot } from "../../../generated/flatbuffers_schema/talk_v1/activity.ts";
import type { SendPayload } from "../models/SendPayload.ts";
import { VectorTable } from "../../../generated/flatbuffers_schema/talk_v1/vector-table.ts";

export const processTerminalOutput = (root: PtyOutputRoot, send: SendPayload) => {
  switch (root.payloadType()) {
    case PtyOutput.Output:
      const payload = root.payload(new VectorTable()) as VectorTable | null;
      if (!payload) return;
      send.toTerminal = payload.dataArray();
      return;
    default:
      console.error("Unknown terminal output payload type:", root.payloadType());
      return;
  }
};
