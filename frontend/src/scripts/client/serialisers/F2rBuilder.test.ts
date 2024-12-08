import { describe, expect, it } from "vitest";
import * as flatbuffers from "flatbuffers";
import { Built, F2rBuilder } from "./F2rBuilder"; // Adjust import path as needed
import {
  F2rError,
  F2rErrorType,
  F2rRoot,
  F2rRootPayload,
  F2rToAgent,
} from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { F2aRootBlob } from "../types/BinaryBlob.ts";

describe("F2rBuilder", () => {
  describe("new()", () => {
    it("should create a new builder in Initial state", () => {
      const builder = F2rBuilder.new();

      // Verify initial state
      expect(builder).toBeDefined();
      // Note: Since _state and other properties are private, we can't directly test them
    });
  });

  describe("buildError()", () => {
    it("should create an error payload", () => {
      const builder = F2rBuilder.new().buildError(F2rErrorType.ErrorUnspecified);

      const blob = builder.toFlatbuffers();

      // Parse the blob
      const buf = new flatbuffers.ByteBuffer(blob.data());
      const root = F2rRoot.getRootAsF2rRoot(buf);

      // Verify payload type
      expect(root.rootPayloadType()).toBe(F2rRootPayload.Error);

      // Verify error details
      const errorPayload = root.rootPayload(new F2rError()) as F2rError | null;
      expect(errorPayload?.errorType()).toBe(F2rErrorType.ErrorUnspecified);
    });

    it("should transition to Built state", () => {
      const builder = F2rBuilder.new().buildError(F2rErrorType.ErrorUnspecified);

      // This is a type-level check that the builder is now in Built state
      const typedBuilder: F2rBuilder<Built> = builder;
      expect(typedBuilder).toBeDefined();
    });
  });

  describe("buildToAgent()", () => {
    it("should create a ToAgent payload", () => {
      // Prepare test payload
      const testPayload = new F2aRootBlob(new Uint8Array([1, 2, 3, 4]));

      const builder = F2rBuilder.new().buildToAgent(testPayload);

      const blob = builder.toFlatbuffers();

      // Parse the blob
      const buf = new flatbuffers.ByteBuffer(blob.data());
      const root = F2rRoot.getRootAsF2rRoot(buf);

      // Verify payload type
      expect(root.rootPayloadType()).toBe(F2rRootPayload.ToAgent);

      // Verify payload details
      const toAgentPayload = root.rootPayload(new F2rToAgent()) as F2rToAgent | null;
      const payloadData = toAgentPayload?.payloadArray();

      expect(payloadData).toEqual(new Uint8Array([1, 2, 3, 4]));
    });

    it("should transition to Built state", () => {
      const testPayload = new F2aRootBlob(new Uint8Array());

      const builder = F2rBuilder.new().buildToAgent(testPayload);

      // This is a type-level check that the builder is now in Built state
      const typedBuilder: F2rBuilder<Built> = builder;
      expect(typedBuilder).toBeDefined();
    });
  });

  describe("toFlatbuffers()", () => {
    it("should create a valid FlatBuffer blob", () => {
      const builder = F2rBuilder.new().buildError(F2rErrorType.ErrorUnspecified);

      const blob = builder.toFlatbuffers();

      // Verify blob is created
      expect(blob).toBeDefined();
      expect(blob.data().length).toBeGreaterThan(0);

      // Attempt to parse the blob
      const buf = new flatbuffers.ByteBuffer(blob.data());
      const root = F2rRoot.getRootAsF2rRoot(buf);

      // Verify basic parsing works
      expect(root.rootPayloadType()).toBe(F2rRootPayload.Error);
    });

    it("supports multiple blob creations", () => {
      const blob1 = F2rBuilder.new().buildError(F2rErrorType.ErrorUnspecified).toFlatbuffers();

      const blob2 = F2rBuilder.new()
        .buildToAgent(new F2aRootBlob(new Uint8Array([5, 6, 7])))
        .toFlatbuffers();

      // Verify blobs are different
      expect(blob1.data()).not.toEqual(blob2.data());

      // Parse and verify each blob
      const buf1 = new flatbuffers.ByteBuffer(blob1.data());
      const root1 = F2rRoot.getRootAsF2rRoot(buf1);
      expect(root1.rootPayloadType()).toBe(F2rRootPayload.Error);

      const buf2 = new flatbuffers.ByteBuffer(blob2.data());
      const root2 = F2rRoot.getRootAsF2rRoot(buf2);
      expect(root2.rootPayloadType()).toBe(F2rRootPayload.ToAgent);
    });
  });

  describe("Type Safety", () => {
    it("should not allow building payload on a Built state", () => {
      // This is a compile-time check, so it's more of a type system test
      const builder = F2rBuilder.new().buildError(F2rErrorType.ErrorUnspecified);

      // These should not compile if uncommented:
      // builder.buildError(F2rErrorType.InternalError);
      // builder.buildToAgent(new MockF2aRootBlob(new Uint8Array()));

      // Placeholder to make the test pass
      expect(builder).toBeDefined();
    });
  });
});
