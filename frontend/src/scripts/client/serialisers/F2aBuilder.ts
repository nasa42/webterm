import * as flatbuffers from "flatbuffers";
import {
  Bits256,
  Bits96,
  EmptyTable,
  F2aActivityInput,
  F2aEncryptedRoot,
  F2aMessage,
  F2aMessageFormat,
  F2aPlainAuthPresentVerification,
  F2aPlainAuthRequestPreamble,
  F2aPlainMessage,
  F2aRoot,
  Version,
} from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { type ActivityInputBlob, F2aRootBlob } from "../types/BinaryBlob.ts";
import type { ActivityId } from "../types/BigIntLike.ts";
import { VERSION } from "../config.ts";

interface BuilderState {}

class Initial implements BuilderState {
  _type = "Initial" as const;
}

export class EncryptionReady implements BuilderState {
  _type = "EncryptionReady" as const;
}

export class PlainReady implements BuilderState {
  _type = "PlainReady" as const;
}

export class F2aBuilder<State extends BuilderState> {
  constructor(
    private readonly builder: flatbuffers.Builder,
    private _state: State,
    private plainMessageType: F2aPlainMessage | null,
    private encryptedMessageType: F2aMessage | null,
    private payloadOffset: number,
  ) {}

  static new(): F2aBuilder<Initial> {
    return new F2aBuilder<Initial>(new flatbuffers.Builder(), new Initial(), null, null, 0);
  }

  buildAuthRequestPreamble(): F2aBuilder<PlainReady> {
    const versionOffset = Version.createVersion(this.builder, VERSION.major, VERSION.minor, VERSION.patch);
    const offset = F2aPlainAuthRequestPreamble.createF2aPlainAuthRequestPreamble(this.builder, versionOffset);

    return new F2aBuilder(this.builder, new PlainReady(), F2aPlainMessage.AuthRequestPreamble, null, offset);
  }

  buildAuthRequestVerification(): F2aBuilder<PlainReady> {
    const solutionOffset = Bits256.createBits256(this.builder, []);
    const offset = F2aPlainAuthPresentVerification.createF2aPlainAuthPresentVerification(
      this.builder,
      solutionOffset,
      0n,
    );
    return new F2aBuilder(this.builder, new PlainReady(), F2aPlainMessage.AuthPresentVerification, null, offset);
  }

  buildActivityCreateTerminal(): F2aBuilder<EncryptionReady> {
    const offset = EmptyTable.createEmptyTable(this.builder);
    return new F2aBuilder(this.builder, new EncryptionReady(), null, F2aMessage.ActivityCreateTerminal, offset);
  }

  buildActivityInputMessage(activityId: ActivityId, input: ActivityInputBlob): F2aBuilder<EncryptionReady> {
    const inputOffset = this.builder.createByteVector(input.data());
    const offset = F2aActivityInput.createF2aActivityInput(this.builder, activityId.int(), inputOffset);
    return new F2aBuilder(this.builder, new EncryptionReady(), null, F2aMessage.ActivityInput, offset);
  }

  toFlatbuffersPlain(this: F2aBuilder<PlainReady>): F2aRootBlob {
    F2aRoot.startF2aRoot(this.builder);
    F2aRoot.addFormat(this.builder, F2aMessageFormat.Plain);
    F2aRoot.addPlainMessageType(this.builder, this.plainMessageType || F2aPlainMessage.NONE);
    F2aRoot.addPlainMessage(this.builder, this.payloadOffset);
    const rootOffset = F2aRoot.endF2aRoot(this.builder);

    this.builder.finish(rootOffset);
    return new F2aRootBlob(this.builder.asUint8Array());
  }

  toFlatbuffersEncrypted(this: F2aBuilder<EncryptionReady>): F2aRootBlob {
    F2aEncryptedRoot.startF2aEncryptedRoot(this.builder);
    F2aEncryptedRoot.addMessageType(this.builder, this.encryptedMessageType || F2aMessage.NONE);
    F2aEncryptedRoot.addMessage(this.builder, this.payloadOffset);
    const payloadOffset = F2aEncryptedRoot.endF2aEncryptedRoot(this.builder);
    this.builder.finish(payloadOffset);
    const payload = this.builder.asUint8Array();

    // TODO: ADD ENCRYPTION

    const encryptedPayload = payload;

    const builder = new flatbuffers.Builder();

    let encryptedPayloadOffset = builder.createByteVector(encryptedPayload);
    F2aRoot.startF2aRoot(builder);
    F2aRoot.addFormat(builder, F2aMessageFormat.AesGcm256DeflateRaw);
    F2aRoot.addIv(builder, Bits96.createBits96(builder, [1, 2, 3, 4]));
    F2aRoot.addEncryptedPayload(builder, encryptedPayloadOffset);
    const rootOffset = F2aRoot.endF2aRoot(builder);

    builder.finish(rootOffset);
    return new F2aRootBlob(builder.asUint8Array());
  }
}
