// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

export class F2aActivityInput {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):F2aActivityInput {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsF2aActivityInput(bb:flatbuffers.ByteBuffer, obj?:F2aActivityInput):F2aActivityInput {
  return (obj || new F2aActivityInput()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsF2aActivityInput(bb:flatbuffers.ByteBuffer, obj?:F2aActivityInput):F2aActivityInput {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new F2aActivityInput()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

activityId():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

input(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

inputLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

inputArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

static startF2aActivityInput(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addActivityId(builder:flatbuffers.Builder, activityId:bigint) {
  builder.addFieldInt64(0, activityId, BigInt('0'));
}

static addInput(builder:flatbuffers.Builder, inputOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, inputOffset, 0);
}

static createInputVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startInputVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static endF2aActivityInput(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createF2aActivityInput(builder:flatbuffers.Builder, activityId:bigint, inputOffset:flatbuffers.Offset):flatbuffers.Offset {
  F2aActivityInput.startF2aActivityInput(builder);
  F2aActivityInput.addActivityId(builder, activityId);
  F2aActivityInput.addInput(builder, inputOffset);
  return F2aActivityInput.endF2aActivityInput(builder);
}
}