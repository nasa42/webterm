// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { R2fRootPayload, unionToR2fRootPayload, unionListToR2fRootPayload } from './r2f-root-payload.js';


export class R2fRoot {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):R2fRoot {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsR2fRoot(bb:flatbuffers.ByteBuffer, obj?:R2fRoot):R2fRoot {
  return (obj || new R2fRoot()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsR2fRoot(bb:flatbuffers.ByteBuffer, obj?:R2fRoot):R2fRoot {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new R2fRoot()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

rootPayloadType():R2fRootPayload {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : R2fRootPayload.NONE;
}

rootPayload<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startR2fRoot(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addRootPayloadType(builder:flatbuffers.Builder, rootPayloadType:R2fRootPayload) {
  builder.addFieldInt8(0, rootPayloadType, R2fRootPayload.NONE);
}

static addRootPayload(builder:flatbuffers.Builder, rootPayloadOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, rootPayloadOffset, 0);
}

static endR2fRoot(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createR2fRoot(builder:flatbuffers.Builder, rootPayloadType:R2fRootPayload, rootPayloadOffset:flatbuffers.Offset):flatbuffers.Offset {
  R2fRoot.startR2fRoot(builder);
  R2fRoot.addRootPayloadType(builder, rootPayloadType);
  R2fRoot.addRootPayload(builder, rootPayloadOffset);
  return R2fRoot.endR2fRoot(builder);
}
}