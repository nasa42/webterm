import * as flatbuffers from "flatbuffers";

import {
  AgentToFrontendMessage,
  AgentToFrontendMessageType,
  FrontendToAgentMessage,
  FrontendToAgentMessageType,
  FrontendToRelayMessage,
  FrontendToRelayMessageType,
  RelayToFrontendMessage,
  RelayToFrontendMessageType,
  ResizeData,
} from "../../generated/flatbuffers_schema/schema";

export function createFrontendToRelayMessage(type: FrontendToRelayMessageType, data: Uint8Array): Uint8Array {
  const builder = new flatbuffers.Builder(1024);

  const dataOffset = builder.createByteVector(data);
  FrontendToRelayMessage.startFrontendToRelayMessage(builder);
  FrontendToRelayMessage.addType(builder, type);
  FrontendToRelayMessage.addData(builder, dataOffset);
  const offset = FrontendToRelayMessage.endFrontendToRelayMessage(builder);

  builder.finish(offset);
  return builder.asUint8Array();
}

export function createFrontendToAgentMessage(type: FrontendToAgentMessageType, data: Uint8Array): Uint8Array {
  const builder = new flatbuffers.Builder(1024);

  const dataOffset = builder.createByteVector(data);
  FrontendToAgentMessage.startFrontendToAgentMessage(builder);
  FrontendToAgentMessage.addType(builder, type);
  FrontendToAgentMessage.addData(builder, dataOffset);
  const offset = FrontendToAgentMessage.endFrontendToAgentMessage(builder);

  builder.finish(offset);
  return builder.asUint8Array();
}

export function createResizeData(cols: number, rows: number): Uint8Array {
  const builder = new flatbuffers.Builder(1024);

  ResizeData.startResizeData(builder);
  ResizeData.addCols(builder, cols);
  ResizeData.addRows(builder, rows);
  const offset = ResizeData.endResizeData(builder);

  builder.finish(offset);
  return builder.asUint8Array();
}

export function readRelayToFrontendMessage(data: Uint8Array): {
  type: RelayToFrontendMessageType;
  data: Uint8Array;
} | null {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  const outerMessage = RelayToFrontendMessage.getRootAsRelayToFrontendMessage(byteBuffer);

  const type = outerMessage.type();
  const dataArray = outerMessage.dataArray();
  if (!dataArray) {
    return null;
  }

  return { type, data: dataArray };
}

export function readAgentToFrontendMessage(data: Uint8Array): {
  type: AgentToFrontendMessageType;
  data: Uint8Array;
} | null {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  const innerMessage = AgentToFrontendMessage.getRootAsAgentToFrontendMessage(byteBuffer);

  const type = innerMessage.type();
  const dataArray = innerMessage.dataArray();
  if (!dataArray) {
    return null;
  }

  return { type, data: dataArray };
}

export function readResizeData(data: Uint8Array): { cols: number; rows: number } | null {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  const resizeMessage = ResizeData.getRootAsResizeData(byteBuffer);

  const cols = resizeMessage.cols();
  const rows = resizeMessage.rows();

  return { cols, rows };
}
