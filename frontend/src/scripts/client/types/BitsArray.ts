import * as flatbuffers from "flatbuffers";
import { Bits96 } from "../../../generated/flatbuffers_schema/talk_v1/bits96.ts";
import { Bits256 } from "../../../generated/flatbuffers_schema/talk_v1/bits256.ts";

class BitsArray {
  protected inner: Uint8Array;

  constructor(data: Uint8Array) {
    this.inner = data;
  }

  data(): Uint8Array {
    return this.inner;
  }

  toNumberArray(): number[] {
    return Array.from(this.inner);
  }

  toBigIntBE(): bigint {
    let result = 0n;
    for (const byte of this.inner) {
      result = (result << 8n) | BigInt(byte);
    }
    return result;
  }
}

const bigIntToArrayBE = (value: bigint, size: number): Uint8Array => {
  const maxValue = (1n << BigInt(size * 8)) - 1n;
  if (value < 0n || value > maxValue) {
    throw new Error(`Value ${value} is outside the valid range for ${size} bytes`);
  }
  return Uint8Array.from({ length: size }, (_, i) => Number((value >> BigInt(8 * (size - 1 - i))) & 0xffn));
};

export class Bits96Array extends BitsArray {
  static readonly LENGTH = 12;

  static random(): Bits96Array {
    return new Bits96Array(crypto.getRandomValues(new Uint8Array(12)));
  }

  static fromFbBits96(fbBits96: Bits96): Bits96Array {
    const result = new Uint8Array(this.LENGTH);
    for (let i = 0; i < this.LENGTH; i++) {
      result[i] = fbBits96.bytes(i)!;  // Use non-null assertion since we know the length
    }
    return new Bits96Array(result);
  }

  static fromBigIntBE(bigInt: bigint): Bits96Array {
    return new Bits96Array(bigIntToArrayBE(bigInt, this.LENGTH));
  }

  constructor(data: Uint8Array) {
    if (data.length !== Bits96Array.LENGTH) throw new Error(`Invalid length ${data.length} for Bits96Array`);
    super(data);
  }

  toFbBits96(builder: flatbuffers.Builder): flatbuffers.Offset {
    return Bits96.createBits96(builder, this.toNumberArray());
  }
}

export class Bits256Array extends BitsArray {
  static readonly LENGTH = 32;

  static random(): Bits256Array {
    return new Bits256Array(crypto.getRandomValues(new Uint8Array(32)));
  }

  static fromFbBits256(fbBits256: Bits256): Bits256Array {
    const result = new Uint8Array(this.LENGTH);
    for (let i = 0; i < this.LENGTH; i++) {
      result[i] = fbBits256.bytes(i)!;  // Use non-null assertion since we know the length
    }
    return new Bits256Array(result);
  }

  static fromBigIntBE(bigInt: bigint): Bits256Array {
    return new Bits256Array(bigIntToArrayBE(bigInt, this.LENGTH));
  }

  constructor(data: Uint8Array) {
    if (data.length !== Bits256Array.LENGTH) throw new Error(`Invalid length ${data.length} for Bits256Array`);
    super(data);
  }

  toFbBits256(builder: flatbuffers.Builder): flatbuffers.Offset {
    return Bits256.createBits256(builder, this.toNumberArray());
  }
}
