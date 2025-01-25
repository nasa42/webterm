import * as flatbuffers from "flatbuffers";
import { R2fHandshakeRoot } from "../../../generated/flatbuffers_schema/handshake_v1/handshake_v1.ts";

export const readR2fHandshakeRoot = (data: Uint8Array): R2fHandshakeRoot => {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return R2fHandshakeRoot.getRootAsR2fHandshakeRoot(byteBuffer);
};
