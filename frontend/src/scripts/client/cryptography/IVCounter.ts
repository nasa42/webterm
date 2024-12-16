import { Bits96Array } from "../types/BitsArray.ts";

class IVCounterClass {
  private counter: bigint;

  constructor() {
    const buffer = new BigUint64Array(1);
    crypto.getRandomValues(buffer);
    const value = buffer[0]!;
    // IV is always "odd" from frontend, and always "even" from agent
    this.counter = value % 2n === 0n ? value + 1n : value;
  }

  next(): Bits96Array {
    const result = Bits96Array.fromBigIntBE(this.counter);
    this.counter += 2n;
    return result;
  }
}

export type IVCounter = InstanceType<typeof IVCounterClass>;
export const ivCounter = new IVCounterClass();
