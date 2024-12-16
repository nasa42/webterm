import * as flatbuffers from "flatbuffers";
import { A2fActivityCreateResponse } from "../../../generated/flatbuffers_schema/talk_v1/a2f-activity-create-response.ts";

export const readCreateActivityResponse = (data: Uint8Array): A2fActivityCreateResponse => {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return A2fActivityCreateResponse.getRootAsA2fActivityCreateResponse(byteBuffer);
};
