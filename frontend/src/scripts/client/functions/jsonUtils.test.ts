import { describe, expect, it } from "vitest";
import { jsonParse, jsonStringify } from "./jsonUtils";

describe("JSON Utils", () => {
  it("should handle primitive types", () => {
    const testCases = [123, "hello", true, null, ["a", "b", "c"], { name: "test", value: 42 }];

    testCases.forEach((testCase) => {
      const serialised = jsonStringify(testCase);
      const deserialised: typeof testCase = jsonParse(serialised);
      expect(deserialised).toEqual(testCase);
    });
  });

  it("should handle Uint8Array", () => {
    const original = new Uint8Array([1, 2, 3, 4, 5]);
    const serialised = jsonStringify(original);
    const deserialised: typeof original = jsonParse(serialised);

    expect(deserialised).toBeInstanceOf(Uint8Array);
    expect(Array.from(deserialised)).toEqual(Array.from(original));
  });

  it("should handle nested objects with Uint8Array", () => {
    const original = {
      name: "test",
      data: new Uint8Array([1, 2, 3]),
      nested: {
        buffer: new Uint8Array([4, 5, 6]),
      },
    };

    const serialised = jsonStringify(original);
    const deserialised: typeof original = jsonParse(serialised);

    expect(deserialised.data).toBeInstanceOf(Uint8Array);
    expect(deserialised.nested.buffer).toBeInstanceOf(Uint8Array);
    expect(Array.from(deserialised.data)).toEqual([1, 2, 3]);
    expect(Array.from(deserialised.nested.buffer)).toEqual([4, 5, 6]);
  });

  it("should handle arrays of Uint8Array", () => {
    const original = [new Uint8Array([1, 2]), new Uint8Array([3, 4]), new Uint8Array([5, 6])];

    const serialised = jsonStringify(original);
    const deserialised: typeof original = jsonParse(serialised);

    deserialised.forEach((arr: Uint8Array, index: number) => {
      expect(arr).toBeInstanceOf(Uint8Array);
      expect(Array.from(arr)).toEqual(Array.from(original[index]!));
    });
  });

  it("should handle empty Uint8Array", () => {
    const original = new Uint8Array();
    const serialised = jsonStringify(original);
    const deserialised: typeof original = jsonParse(serialised);

    expect(deserialised).toBeInstanceOf(Uint8Array);
    expect(deserialised.length).toBe(0);
  });

  it("should throw on invalid JSON", () => {
    expect(() => jsonParse("invalid json")).toThrow();
  });
});
