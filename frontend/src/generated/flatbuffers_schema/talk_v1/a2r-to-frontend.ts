// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

export class A2rToFrontend {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):A2rToFrontend {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsA2rToFrontend(bb:flatbuffers.ByteBuffer, obj?:A2rToFrontend):A2rToFrontend {
  return (obj || new A2rToFrontend()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsA2rToFrontend(bb:flatbuffers.ByteBuffer, obj?:A2rToFrontend):A2rToFrontend {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new A2rToFrontend()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

frontendId():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

payload(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

payloadLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

payloadArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

static startA2rToFrontend(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addFrontendId(builder:flatbuffers.Builder, frontendId:bigint) {
  builder.addFieldInt64(0, frontendId, BigInt('0'));
}

static addPayload(builder:flatbuffers.Builder, payloadOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, payloadOffset, 0);
}

static createPayloadVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startPayloadVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static endA2rToFrontend(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createA2rToFrontend(builder:flatbuffers.Builder, frontendId:bigint, payloadOffset:flatbuffers.Offset):flatbuffers.Offset {
  A2rToFrontend.startA2rToFrontend(builder);
  A2rToFrontend.addFrontendId(builder, frontendId);
  A2rToFrontend.addPayload(builder, payloadOffset);
  return A2rToFrontend.endA2rToFrontend(builder);
}
}