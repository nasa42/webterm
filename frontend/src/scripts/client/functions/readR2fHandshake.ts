import * as flatbuffers from "flatbuffers";
import { R2fHandshake } from "../../../generated/flatbuffers_schema/handshake_v1/handshake_v1.ts";

export const readR2fHandshake = (data: Uint8Array): R2fHandshake => {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return R2fHandshake.getRootAsR2fHandshake(byteBuffer);
};
