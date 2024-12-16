import * as flatbuffers from "flatbuffers";
import { R2fRoot } from "../../../generated/flatbuffers_schema/talk_v1/talk_v1";

export function readR2fRoot(data: Uint8Array): R2fRoot {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return R2fRoot.getRootAsR2fRoot(byteBuffer);
}
