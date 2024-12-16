import * as flatbuffers from "flatbuffers";
import { TerminalOutputRoot } from "../../../generated/flatbuffers_schema/talk_v1/activity.ts";

export const readTerminalOutput = (data: Uint8Array): TerminalOutputRoot => {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return TerminalOutputRoot.getRootAsTerminalOutputRoot(byteBuffer);
};
