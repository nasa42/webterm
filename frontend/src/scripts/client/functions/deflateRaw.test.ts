import { describe, expect, it } from "vitest";
import { deflateRawCompress, deflateRawDecompress } from "./deflateRaw.ts";

// Test vectors shared between Rust and TypeScript implementations
const COMPATIBILITY_TEST_CASES: Array<[Uint8Array, Uint8Array]> = [
  [new Uint8Array([]), new Uint8Array([3, 0])],
  [
    new TextEncoder().encode("Hello, world!"),
    new Uint8Array([243, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 81, 4, 0]),
  ],
  [new TextEncoder().encode("AAAAAAAA"), new Uint8Array([115, 116, 132, 0, 0])],
];

describe("deflateRawDecompress compression functions", () => {
  it("should compress and decompress data correctly", async () => {
    const original = new TextEncoder().encode("A".repeat(1000));
    const compressed = await deflateRawCompress(original);
    const decompressed = await deflateRawDecompress(compressed);

    expect(compressed.length).toBeLessThan(original.length);
    expect(decompressed).toEqual(original);
  });

  it("should handle empty input", async () => {
    const empty = new Uint8Array();
    const compressed = await deflateRawCompress(empty);
    const decompressed = await deflateRawDecompress(compressed);

    expect(decompressed).toEqual(empty);
  });

  it("should handle small input", async () => {
    const small = new TextEncoder().encode("Hello");
    const compressed = await deflateRawCompress(small);
    const decompressed = await deflateRawDecompress(compressed);

    expect(new TextDecoder().decode(decompressed)).toBe("Hello");
  });

  describe("cross-implementation compatibility", () => {
    for (const [input, expected] of COMPATIBILITY_TEST_CASES) {
      it(`should match reference output for: ${new TextDecoder().decode(input) || "empty"}`, async () => {
        const compressed = await deflateRawCompress(input);
        expect([...compressed]).toEqual([...expected]);

        const decompressed = await deflateRawDecompress(compressed);
        expect([...decompressed]).toEqual([...input]);
      });
    }
  });
});
