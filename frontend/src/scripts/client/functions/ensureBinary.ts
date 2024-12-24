import type { BinaryLike } from "../types/BinaryLike.ts";

const textEncoder = new TextEncoder();

export const ensureBinary = (data: BinaryLike): Uint8Array => {
  if (typeof data === "string") {
    data = textEncoder.encode(data);
  }

  return data;
};
