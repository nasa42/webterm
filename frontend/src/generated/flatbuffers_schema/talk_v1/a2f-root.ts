// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { A2fMessageFormat } from './a2f-message-format.js';
import { A2fPlainMessage, unionToA2fPlainMessage, unionListToA2fPlainMessage } from './a2f-plain-message.js';
import { Bits96 } from './bits96.js';


export class A2fRoot {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):A2fRoot {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsA2fRoot(bb:flatbuffers.ByteBuffer, obj?:A2fRoot):A2fRoot {
  return (obj || new A2fRoot()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsA2fRoot(bb:flatbuffers.ByteBuffer, obj?:A2fRoot):A2fRoot {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new A2fRoot()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

format():A2fMessageFormat {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : A2fMessageFormat.Plain;
}

iv(obj?:Bits96):Bits96|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new Bits96()).__init(this.bb_pos + offset, this.bb!) : null;
}

plainMessageType():A2fPlainMessage {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : A2fPlainMessage.NONE;
}

plainMessage<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

encryptedPayload(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

encryptedPayloadLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

encryptedPayloadArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

messageId():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

static startA2fRoot(builder:flatbuffers.Builder) {
  builder.startObject(6);
}

static addFormat(builder:flatbuffers.Builder, format:A2fMessageFormat) {
  builder.addFieldInt8(0, format, A2fMessageFormat.Plain);
}

static addIv(builder:flatbuffers.Builder, ivOffset:flatbuffers.Offset) {
  builder.addFieldStruct(1, ivOffset, 0);
}

static addPlainMessageType(builder:flatbuffers.Builder, plainMessageType:A2fPlainMessage) {
  builder.addFieldInt8(2, plainMessageType, A2fPlainMessage.NONE);
}

static addPlainMessage(builder:flatbuffers.Builder, plainMessageOffset:flatbuffers.Offset) {
  builder.addFieldOffset(3, plainMessageOffset, 0);
}

static addEncryptedPayload(builder:flatbuffers.Builder, encryptedPayloadOffset:flatbuffers.Offset) {
  builder.addFieldOffset(4, encryptedPayloadOffset, 0);
}

static createEncryptedPayloadVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startEncryptedPayloadVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addMessageId(builder:flatbuffers.Builder, messageId:bigint) {
  builder.addFieldInt64(5, messageId, BigInt('0'));
}

static endA2fRoot(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

}
