import type { TerminalOutputRoot } from "../../../generated/flatbuffers_schema/talk_v1/activity/terminal-output-root.ts";
import type { SendPayload } from "../models/SendPayload.ts";
import { TerminalOutput } from "../../../generated/flatbuffers_schema/talk_v1/activity/terminal-output.ts";
import { VectorTable } from "../../../generated/flatbuffers_schema/talk_v1/vector-table.ts";

export const processTerminalOutput = (root: TerminalOutputRoot, send: SendPayload) => {
  switch (root.payloadType()) {
    case TerminalOutput.Output:
      let payload = root.payload(new VectorTable()) as VectorTable | null;
      if (!payload) return;
      send.toTerminal = payload.dataArray();
      return;
    default:
      console.error("Unknown terminal output payload type:", root.payloadType());
      return;
  }
};
