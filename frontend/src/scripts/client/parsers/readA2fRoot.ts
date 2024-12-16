import * as flatbuffers from "flatbuffers";
import { A2fRoot } from "../../../generated/flatbuffers_schema/talk_v1/talk_v1";

export function readA2fRoot(data: Uint8Array): A2fRoot {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return A2fRoot.getRootAsA2fRoot(byteBuffer);
}
