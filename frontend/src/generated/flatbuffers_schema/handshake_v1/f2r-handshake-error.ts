// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { F2rHandshakeErrorType } from './f2r-handshake-error-type.js';


export class F2rHandshakeError {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):F2rHandshakeError {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsF2rHandshakeError(bb:flatbuffers.ByteBuffer, obj?:F2rHandshakeError):F2rHandshakeError {
  return (obj || new F2rHandshakeError()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsF2rHandshakeError(bb:flatbuffers.ByteBuffer, obj?:F2rHandshakeError):F2rHandshakeError {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new F2rHandshakeError()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

errorType():F2rHandshakeErrorType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : F2rHandshakeErrorType.ErrorUnspecified;
}

errorMessage():string|null
errorMessage(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
errorMessage(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startF2rHandshakeError(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addErrorType(builder:flatbuffers.Builder, errorType:F2rHandshakeErrorType) {
  builder.addFieldInt8(0, errorType, F2rHandshakeErrorType.ErrorUnspecified);
}

static addErrorMessage(builder:flatbuffers.Builder, errorMessageOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, errorMessageOffset, 0);
}

static endF2rHandshakeError(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createF2rHandshakeError(builder:flatbuffers.Builder, errorType:F2rHandshakeErrorType, errorMessageOffset:flatbuffers.Offset):flatbuffers.Offset {
  F2rHandshakeError.startF2rHandshakeError(builder);
  F2rHandshakeError.addErrorType(builder, errorType);
  F2rHandshakeError.addErrorMessage(builder, errorMessageOffset);
  return F2rHandshakeError.endF2rHandshakeError(builder);
}
}
