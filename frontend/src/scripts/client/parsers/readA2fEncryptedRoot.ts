import * as flatbuffers from "flatbuffers";
import { A2fEncryptedRoot } from "../../../generated/flatbuffers_schema/talk_v1/talk_v1";

export function readA2fEncryptedRoot(data: Uint8Array): A2fEncryptedRoot {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return A2fEncryptedRoot.getRootAsA2fEncryptedRoot(byteBuffer);
}
