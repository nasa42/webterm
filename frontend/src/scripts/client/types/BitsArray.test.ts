import { describe, expect, test } from "vitest";

import { Bits256Array, Bits96Array } from "./BitsArray";
import * as flatbuffers from "flatbuffers";
import { Bits96 } from "../../../generated/flatbuffers_schema/talk_v1/bits96";
import { Bits256 } from "../../../generated/flatbuffers_schema/talk_v1/bits256";

describe("BitsArray Classes", () => {
  describe("Bits96Array", () => {
    test("constructor validates length", () => {
      expect(() => new Bits96Array(new Uint8Array(11))).toThrow("Invalid length");
      expect(() => new Bits96Array(new Uint8Array(13))).toThrow("Invalid length");
      expect(() => new Bits96Array(new Uint8Array(12))).not.toThrow();
    });

    test("random() creates valid array", () => {
      const random = Bits96Array.random();
      expect(random.data().length).toBe(12);
      // Test that two random arrays are different
      const random2 = Bits96Array.random();
      expect(random.data()).not.toEqual(random2.data());
    });

    test("fromBigIntBE converts correctly", () => {
      const value = 0x0102030405060708090a0b0cn;
      const bits = Bits96Array.fromBigIntBE(value);
      expect(bits.data()).toEqual(
        new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]),
      );
    });

    test("fromBigIntBE handles smaller numbers", () => {
      const smallValue = 0x0102n;
      const bits = Bits96Array.fromBigIntBE(smallValue);
      expect(bits.data()).toEqual(
        new Uint8Array([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02]),
      );

      const zeroValue = 0n;
      const zeroBits = Bits96Array.fromBigIntBE(zeroValue);
      expect(zeroBits.data()).toEqual(new Uint8Array(12)); // all zeros
    });

    test("fromFbBits96 converts from flatbuffer object", () => {
      const builder = new flatbuffers.Builder(1024);
      const testData = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c];
      const offset = Bits96.createBits96(builder, testData);
      builder.finish(offset);
      const buf = builder.asUint8Array();
      const fbBits96 = new Bits96();
      const byteBuffer = new flatbuffers.ByteBuffer(buf);
      // For structs, we read directly from the root position
      fbBits96.__init(byteBuffer.readInt32(0), byteBuffer);
      const bits = Bits96Array.fromFbBits96(fbBits96);
      expect(Array.from(bits.data())).toEqual(testData);
    });

    test("toBits96 converts to flatbuffer object", () => {
      const testData = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
      const bits = new Bits96Array(testData);
      const builder = new flatbuffers.Builder(1024);
      const offset = bits.toFbBits96(builder);
      expect(offset).toBeGreaterThan(0);
    });

    test("fromBigIntBE throws on overflow", () => {
      const tooLarge = 1n << 96n;
      expect(() => Bits96Array.fromBigIntBE(tooLarge)).toThrow("outside the valid range");
      expect(() => Bits96Array.fromBigIntBE(-1n)).toThrow("outside the valid range");
    });
  });

  describe("Bits256Array", () => {
    test("constructor validates length", () => {
      expect(() => new Bits256Array(new Uint8Array(31))).toThrow("Invalid length");
      expect(() => new Bits256Array(new Uint8Array(33))).toThrow("Invalid length");
      expect(() => new Bits256Array(new Uint8Array(32))).not.toThrow();
    });

    test("random() creates valid array", () => {
      const random = Bits256Array.random();
      expect(random.data().length).toBe(32);
      // Test that two random arrays are different
      const random2 = Bits256Array.random();
      expect(random.data()).not.toEqual(random2.data());
    });

    test("fromBigIntBE converts correctly", () => {
      const value = 0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20n;
      const bits = Bits256Array.fromBigIntBE(value);
      expect(bits.data()).toEqual(
        new Uint8Array([
          0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
          0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
        ]),
      );
    });

    test("fromBigIntBE handles smaller numbers", () => {
      const smallValue = 0x0102030405n;
      const bits = Bits256Array.fromBigIntBE(smallValue);
      expect(bits.data()).toEqual(
        new Uint8Array([
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05
        ]),
      );

      const zeroValue = 0n;
      const zeroBits = Bits256Array.fromBigIntBE(zeroValue);
      expect(zeroBits.data()).toEqual(new Uint8Array(32)); // all zeros
    });

    test("fromFbBits256 converts from flatbuffer object", () => {
      const builder = new flatbuffers.Builder(1024);
      const testData = Array(32).fill(0).map((_, i) => i + 1);
      const offset = Bits256.createBits256(builder, testData);
      builder.finish(offset);
      const buf = builder.asUint8Array();
      const fbBits256 = new Bits256();
      const byteBuffer = new flatbuffers.ByteBuffer(buf);
      // For structs, we read directly from the root position
      fbBits256.__init(byteBuffer.readInt32(0), byteBuffer);
      const bits = Bits256Array.fromFbBits256(fbBits256);
      expect(Array.from(bits.data())).toEqual(testData);
    });

    test("toBits256 converts to flatbuffer object", () => {
      const testData = new Uint8Array(
        Array(32)
          .fill(0)
          .map((_, i) => i + 1),
      );
      const bits = new Bits256Array(testData);
      const builder = new flatbuffers.Builder(1024);
      const offset = bits.toFbBits256(builder);
      expect(offset).toBeGreaterThan(0);
    });

    test("fromBigIntBE throws on overflow", () => {
      const tooLarge = 1n << 256n;
      expect(() => Bits256Array.fromBigIntBE(tooLarge)).toThrow("outside the valid range");
      expect(() => Bits256Array.fromBigIntBE(-1n)).toThrow("outside the valid range");
    });
  });

  describe("Common BitsArray functionality", () => {
    test("data() returns original array", () => {
      const testData = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
      const bits = new Bits96Array(testData);
      expect(bits.data()).toEqual(testData);
    });

    test("toNumberArray() converts to number array", () => {
      const testData = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
      const bits = new Bits96Array(testData);
      expect(bits.toNumberArray()).toEqual(Array.from(testData));
    });

    test("toBigIntBE converts correctly", () => {
      const value96 = 0x0102030405060708090a0b0cn;
      const bits96 = Bits96Array.fromBigIntBE(value96);
      expect(bits96.toBigIntBE()).toBe(value96);

      const value256 = 0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20n;
      const bits256 = Bits256Array.fromBigIntBE(value256);
      expect(bits256.toBigIntBE()).toBe(value256);
    });

    test("toBigIntBE handles zero", () => {
      const bits96 = Bits96Array.fromBigIntBE(0n);
      expect(bits96.toBigIntBE()).toBe(0n);

      const bits256 = Bits256Array.fromBigIntBE(0n);
      expect(bits256.toBigIntBE()).toBe(0n);
    });
  });
});
