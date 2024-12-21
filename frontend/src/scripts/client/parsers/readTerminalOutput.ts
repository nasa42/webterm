import * as flatbuffers from "flatbuffers";
import { PtyOutputRoot } from "../../../generated/flatbuffers_schema/talk_v1/activity.ts";

export const readTerminalOutput = (data: Uint8Array): PtyOutputRoot => {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return PtyOutputRoot.getRootAsPtyOutputRoot(byteBuffer);
};
