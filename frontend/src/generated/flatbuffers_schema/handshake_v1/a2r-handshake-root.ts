// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { A2rHandshakeRootPayload, unionToA2rHandshakeRootPayload, unionListToA2rHandshakeRootPayload } from './a2r-handshake-root-payload.js';
import { Version } from './version.js';


export class A2rHandshakeRoot {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):A2rHandshakeRoot {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsA2rHandshakeRoot(bb:flatbuffers.ByteBuffer, obj?:A2rHandshakeRoot):A2rHandshakeRoot {
  return (obj || new A2rHandshakeRoot()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsA2rHandshakeRoot(bb:flatbuffers.ByteBuffer, obj?:A2rHandshakeRoot):A2rHandshakeRoot {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new A2rHandshakeRoot()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

agentVersion(obj?:Version):Version|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new Version()).__init(this.bb_pos + offset, this.bb!) : null;
}

rootPayloadType():A2rHandshakeRootPayload {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : A2rHandshakeRootPayload.NONE;
}

rootPayload<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startA2rHandshakeRoot(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addAgentVersion(builder:flatbuffers.Builder, agentVersionOffset:flatbuffers.Offset) {
  builder.addFieldStruct(0, agentVersionOffset, 0);
}

static addRootPayloadType(builder:flatbuffers.Builder, rootPayloadType:A2rHandshakeRootPayload) {
  builder.addFieldInt8(1, rootPayloadType, A2rHandshakeRootPayload.NONE);
}

static addRootPayload(builder:flatbuffers.Builder, rootPayloadOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, rootPayloadOffset, 0);
}

static endA2rHandshakeRoot(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createA2rHandshakeRoot(builder:flatbuffers.Builder, agentVersionOffset:flatbuffers.Offset, rootPayloadType:A2rHandshakeRootPayload, rootPayloadOffset:flatbuffers.Offset):flatbuffers.Offset {
  A2rHandshakeRoot.startA2rHandshakeRoot(builder);
  A2rHandshakeRoot.addAgentVersion(builder, agentVersionOffset);
  A2rHandshakeRoot.addRootPayloadType(builder, rootPayloadType);
  A2rHandshakeRoot.addRootPayload(builder, rootPayloadOffset);
  return A2rHandshakeRoot.endA2rHandshakeRoot(builder);
}
}
