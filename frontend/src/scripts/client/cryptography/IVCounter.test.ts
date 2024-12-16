import { ivCounter } from "./IVCounter";
import { describe, expect, test } from "vitest";

describe("IVCounter", () => {
  test("next() returns 12-byte Uint8Array", () => {
    const result = ivCounter.next().data();
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(12);
  });

  test("consecutive calls increment by 2 each time", () => {
    const iterations = 5;
    const results = Array(iterations)
      .fill(0)
      .map(() => ivCounter.next().toBigIntBE());

    for (let i = 1; i < results.length; i++) {
      expect(results[i]! - results[i - 1]!).toBe(2n);
    }
  });

  test("maintains odd/even pattern", () => {
    const results = Array(4)
      .fill(0)
      .map(() => ivCounter.next().toBigIntBE());
    results.forEach((value) => {
      expect(value % 2n).toBe(1n); // Should always be odd for frontend
    });
  });
});
