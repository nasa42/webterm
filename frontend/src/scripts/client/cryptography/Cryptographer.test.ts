import { describe, expect, it } from "vitest";
import { Cryptographer } from "./Cryptographer";
import { Bits256Array } from "../types/BitsArray.ts";

describe("Cryptographer", () => {
  const salt = Bits256Array.random();
  const secretKey = "test-secret-key";

  it("should encrypt and decrypt small payload correctly", async () => {
    const { cryptographer } = await Cryptographer.new({ iterations: 1, salt, secretKey });
    const data = new TextEncoder().encode("Hello, World!");

    const { ciphertext, iv, compressed } = await cryptographer.encrypt(data);
    expect(compressed).toBe(false);

    const decrypted = await cryptographer.decrypt(ciphertext, iv, compressed);
    expect(new TextDecoder().decode(decrypted)).toBe("Hello, World!");
  });

  it("should compress large payload", async () => {
    const { cryptographer } = await Cryptographer.new({ iterations: 1, salt, secretKey });
    const data = new TextEncoder().encode("A".repeat(1000));

    const { ciphertext, iv, compressed } = await cryptographer.encrypt(data, true);
    expect(compressed).toBe(true);

    const decrypted = await cryptographer.decrypt(ciphertext, iv, compressed);
    expect(new TextDecoder().decode(decrypted)).toBe("A".repeat(1000));
  });

  it("should throw on incorrect decryption", async () => {
    const { cryptographer } = await Cryptographer.new({ iterations: 1, salt, secretKey });
    const data = new TextEncoder().encode("Test");

    const { ciphertext, iv } = await cryptographer.encrypt(data);
    // Modify ciphertext to cause decryption failure
    ciphertext[0]! ^= 1;

    await expect(cryptographer.decrypt(ciphertext, iv, false)).rejects.toThrow();
  });
});
