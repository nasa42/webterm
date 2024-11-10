// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { AgentToRelayMessageType } from './agent-to-relay-message-type.js';


export class AgentToRelayMessage {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):AgentToRelayMessage {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsAgentToRelayMessage(bb:flatbuffers.ByteBuffer, obj?:AgentToRelayMessage):AgentToRelayMessage {
  return (obj || new AgentToRelayMessage()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsAgentToRelayMessage(bb:flatbuffers.ByteBuffer, obj?:AgentToRelayMessage):AgentToRelayMessage {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new AgentToRelayMessage()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

type():AgentToRelayMessageType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : AgentToRelayMessageType.ToFrontend;
}

data(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

dataLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

dataArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

static startAgentToRelayMessage(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addType(builder:flatbuffers.Builder, type:AgentToRelayMessageType) {
  builder.addFieldInt8(0, type, AgentToRelayMessageType.ToFrontend);
}

static addData(builder:flatbuffers.Builder, dataOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, dataOffset, 0);
}

static createDataVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startDataVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static endAgentToRelayMessage(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createAgentToRelayMessage(builder:flatbuffers.Builder, type:AgentToRelayMessageType, dataOffset:flatbuffers.Offset):flatbuffers.Offset {
  AgentToRelayMessage.startAgentToRelayMessage(builder);
  AgentToRelayMessage.addType(builder, type);
  AgentToRelayMessage.addData(builder, dataOffset);
  return AgentToRelayMessage.endAgentToRelayMessage(builder);
}
}
