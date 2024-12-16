// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { R2aErrorType } from './r2a-error-type.js';


export class R2aError {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):R2aError {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsR2aError(bb:flatbuffers.ByteBuffer, obj?:R2aError):R2aError {
  return (obj || new R2aError()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsR2aError(bb:flatbuffers.ByteBuffer, obj?:R2aError):R2aError {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new R2aError()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

errorType():R2aErrorType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : R2aErrorType.ErrorUnspecified;
}

errorMessage():string|null
errorMessage(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
errorMessage(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startR2aError(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addErrorType(builder:flatbuffers.Builder, errorType:R2aErrorType) {
  builder.addFieldInt8(0, errorType, R2aErrorType.ErrorUnspecified);
}

static addErrorMessage(builder:flatbuffers.Builder, errorMessageOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, errorMessageOffset, 0);
}

static endR2aError(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createR2aError(builder:flatbuffers.Builder, errorType:R2aErrorType, errorMessageOffset:flatbuffers.Offset):flatbuffers.Offset {
  R2aError.startR2aError(builder);
  R2aError.addErrorType(builder, errorType);
  R2aError.addErrorMessage(builder, errorMessageOffset);
  return R2aError.endR2aError(builder);
}
}
