import * as flatbuffers from "flatbuffers";
import { F2rHandshake, Version } from "../../../generated/flatbuffers_schema/handshake_v1/handshake_v1.ts";

export const createF2rHandshake = (serverId: string): Uint8Array => {
  const builder = new flatbuffers.Builder(1024);

  const serverIdOffset = builder.createString(serverId);

  F2rHandshake.startF2rHandshake(builder);
  const versionOffset = Version.createVersion(builder, 0, 1, 0);
  F2rHandshake.addFrontendVersion(builder, versionOffset);
  F2rHandshake.addServerId(builder, serverIdOffset);
  const offset = F2rHandshake.endF2rHandshake(builder);

  builder.finish(offset);
  return builder.asUint8Array();
};
