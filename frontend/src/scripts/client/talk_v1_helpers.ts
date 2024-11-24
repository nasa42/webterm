import * as flatbuffers from "flatbuffers";
import {
  A2fMessage,
  F2aMessage,
  F2aMessageType,
  F2rMessage,
  F2rMessageType,
  R2fMessage,
  ResizeData,
} from "../../generated/flatbuffers_schema/talk_v1/talk_v1";

export function createF2rMessage(type: F2rMessageType, data: Uint8Array): Uint8Array {
  const builder = new flatbuffers.Builder(1024);

  const dataOffset = builder.createByteVector(data);
  F2rMessage.startF2rMessage(builder);
  F2rMessage.addType(builder, type);
  F2rMessage.addData(builder, dataOffset);
  const offset = F2rMessage.endF2rMessage(builder);

  builder.finish(offset);
  return builder.asUint8Array();
}

export function createF2aMessage(type: F2aMessageType, data: Uint8Array): Uint8Array {
  const builder = new flatbuffers.Builder(1024);

  const dataOffset = builder.createByteVector(data);
  F2aMessage.startF2aMessage(builder);
  F2aMessage.addType(builder, type);
  F2aMessage.addData(builder, dataOffset);
  const offset = F2aMessage.endF2aMessage(builder);

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

export function readR2fMessage(data: Uint8Array): R2fMessage {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return R2fMessage.getRootAsR2fMessage(byteBuffer);
}

export function readA2fMessage(data: Uint8Array): A2fMessage {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return A2fMessage.getRootAsA2fMessage(byteBuffer);
}

export function readResizeData(data: Uint8Array): ResizeData {
  const byteBuffer = new flatbuffers.ByteBuffer(data);
  return ResizeData.getRootAsResizeData(byteBuffer);
}
