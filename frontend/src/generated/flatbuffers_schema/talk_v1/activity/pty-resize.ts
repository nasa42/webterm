// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

export class PtyResize {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):PtyResize {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsPtyResize(bb:flatbuffers.ByteBuffer, obj?:PtyResize):PtyResize {
  return (obj || new PtyResize()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsPtyResize(bb:flatbuffers.ByteBuffer, obj?:PtyResize):PtyResize {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new PtyResize()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

cols():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

rows():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

static startPtyResize(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addCols(builder:flatbuffers.Builder, cols:number) {
  builder.addFieldInt16(0, cols, 0);
}

static addRows(builder:flatbuffers.Builder, rows:number) {
  builder.addFieldInt16(1, rows, 0);
}

static endPtyResize(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createPtyResize(builder:flatbuffers.Builder, cols:number, rows:number):flatbuffers.Offset {
  PtyResize.startPtyResize(builder);
  PtyResize.addCols(builder, cols);
  PtyResize.addRows(builder, rows);
  return PtyResize.endPtyResize(builder);
}
}