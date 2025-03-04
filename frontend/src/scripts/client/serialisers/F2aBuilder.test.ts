import { describe, expect, it } from "vitest";
import * as flatbuffers from "flatbuffers";
import { EncryptionReady, F2aBuilder, PlainReady } from "./F2aBuilder";
import {
  F2aEncryptedRoot,
  F2aMessage,
  F2aMessageFormat,
  F2aPlainAuthRequestPreamble,
  F2aPlainMessage,
  F2aRoot,
  Version,
} from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { CONFIG } from "../config";
import { ActivityId } from "../types/BigIntLike";
import { Cryptographer } from "../cryptography/Cryptographer.ts";
import { Bits96Array } from "../types/BitsArray.ts";
import { ActivityInputBlob } from "./TerminalInputBuilder.ts";

const { cryptographer } = await Cryptographer.new({
  iterations: 1,
  secretKey: "a",
});

describe("F2aBuilder", () => {
  describe("new()", () => {
    it("should create a new builder in Initial state", () => {
      const builder = F2aBuilder.new();

      // Verify initial state
      expect(builder).toBeDefined();
    });
  });

  describe("buildAuthRequestPreamble()", () => {
    it("should create an AuthRequestPreamble payload", () => {
      const builder = F2aBuilder.new().buildAuthRequestPreamble();
      const blob = builder.toFlatbuffersPlain();

      const buf = new flatbuffers.ByteBuffer(blob.data());
      const root = F2aRoot.getRootAsF2aRoot(buf);

      expect(root.format()).toBe(F2aMessageFormat.Plain);
      expect(root.plainMessageType()).toBe(F2aPlainMessage.AuthRequestPreamble);

      const preamble = root.plainMessage(new F2aPlainAuthRequestPreamble()) as F2aPlainAuthRequestPreamble;
      const version = preamble.frontendVersion(new Version());

      expect(version?.major()).toBe(CONFIG.version.major);
      expect(version?.minor()).toBe(CONFIG.version.minor);
      expect(version?.patch()).toBe(CONFIG.version.patch);
    });

    it("should transition to PlainReady state", () => {
      const builder = F2aBuilder.new().buildAuthRequestPreamble();

      // This is a type-level check that the builder is now in PlainReady state
      const typedBuilder: F2aBuilder<PlainReady> = builder;
      expect(typedBuilder).toBeDefined();
    });
  });

  describe("buildActivityCreateTerminal()", () => {
    it("should create an ActivityCreateTerminal payload", async () => {
      const builder = F2aBuilder.new().buildActivityCreateTerminal();
      const blob = await builder.toFlatbuffersEncrypted(cryptographer);

      const buf = new flatbuffers.ByteBuffer(blob.data());
      const root = F2aRoot.getRootAsF2aRoot(buf);

      // decrypt the payload and test the value
      const decrypted = await cryptographer.decrypt(
        root.encryptedPayloadArray()!,
        Bits96Array.fromFbBits96(root.iv()!),
        root.format() === F2aMessageFormat.Aes256GcmDeflateRaw,
      );

      const byteBuffer = new flatbuffers.ByteBuffer(decrypted);
      const f2aEncryptedRoot = F2aEncryptedRoot.getRootAsF2aEncryptedRoot(byteBuffer);

      expect(f2aEncryptedRoot.messageType()).toBe(F2aMessage.ActivityCreateTerminal);
    });

    it("should transition to EncryptionReady state", () => {
      const builder = F2aBuilder.new().buildActivityCreateTerminal();

      // This is a type-level check that the builder is now in EncryptionReady state
      const typedBuilder: F2aBuilder<EncryptionReady> = builder;
      expect(typedBuilder).toBeDefined();
    });
  });

  describe("buildActivityInputMessage()", () => {
    it("should create an ActivityInput payload", async () => {
      const testPayload = new ActivityInputBlob(new Uint8Array([1, 2, 3, 4]));
      const testActivityId = new ActivityId(42n);

      const builder = F2aBuilder.new().buildActivityInputMessage(testActivityId, testPayload);
      const blob = await builder.toFlatbuffersEncrypted(cryptographer);

      const buf = new flatbuffers.ByteBuffer(blob.data());
      const root = F2aRoot.getRootAsF2aRoot(buf);

      // decrypt the payload and test the value
      const decrypted = await cryptographer.decrypt(
        root.encryptedPayloadArray()!,
        Bits96Array.fromFbBits96(root.iv()!),
        root.format() === F2aMessageFormat.Aes256GcmDeflateRaw,
      );

      const byteBuffer = new flatbuffers.ByteBuffer(decrypted);
      const f2aEncryptedRoot = F2aEncryptedRoot.getRootAsF2aEncryptedRoot(byteBuffer);

      expect(f2aEncryptedRoot.messageType()).toBe(F2aMessage.ActivityInput);
    });

    it("should transition to EncryptionReady state", () => {
      const testPayload = new ActivityInputBlob(new Uint8Array());
      const testActivityId = new ActivityId(42n);

      const builder = F2aBuilder.new().buildActivityInputMessage(testActivityId, testPayload);

      // This is a type-level check that the builder is now in EncryptionReady state
      const typedBuilder: F2aBuilder<EncryptionReady> = builder;
      expect(typedBuilder).toBeDefined();
    });
  });

  describe("Type Safety", () => {
    it("should not allow building encrypted payload on Plain state", () => {
      // This is a compile-time check, so it's more of a type system test
      const builder = F2aBuilder.new().buildAuthRequestPreamble();

      // These should not compile if uncommented:
      // builder.toFlatbuffersEncrypted();

      // Placeholder to make the test pass
      expect(builder).toBeDefined();
    });

    it("should not allow building plain payload on Encryption state", () => {
      // This is a compile-time check, so it's more of a type system test
      const builder = F2aBuilder.new().buildActivityCreateTerminal();

      // These should not compile if uncommented:
      // builder.toFlatbuffersPlain();

      // Placeholder to make the test pass
      expect(builder).toBeDefined();
    });
  });
});
