import * as flatbuffers from "flatbuffers";
import { F2rHandshake, R2fHandshake, Version } from "../../generated/flatbuffers_schema/handshake_v1/handshake_v1.ts";
import { TEST_SERVER_ID } from "./config.ts";

export const createF2rHandshake = (): Uint8Array => {
  const builder = new flatbuffers.Builder(1024);

  const serverIdOffset = builder.createString(TEST_SERVER_ID);

  F2rHandshake.startF2rHandshake(builder);
  const versionOffset = Version.createVersion(builder, 0, 1, 0);
  F2rHandshake.addFrontendVersion(builder, versionOffset);
  F2rHandshake.addServerId(builder, serverIdOffset);
  const offset = F2rHandshake.endF2rHandshake(builder);

  builder.finish(offset);
  return builder.asUint8Array();
};

export const readR2fHandshake = (data: Uint8Array): R2fHandshake => {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return R2fHandshake.getRootAsR2fHandshake(byteBuffer);
};
