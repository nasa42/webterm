import * as flatbuffers from "flatbuffers";
import { BinaryBlob } from "../types/BinaryBlob.ts";
import {
  F2rHandshakeError,
  F2rHandshakeErrorType,
  F2rHandshakeRequestConnection,
  F2rHandshakeRoot,
  F2rHandshakeRootPayload,
  Version,
} from "../../../generated/flatbuffers_schema/handshake_v1/handshake_v1.ts";
import { CONFIG } from "../config.ts";

export class F2rHandshakeRootBlob extends BinaryBlob {}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface BuilderState {}

class Initial implements BuilderState {
  _type = "Initial" as const;
}

export class Built implements BuilderState {
  _type = "Built" as const;
}

export class F2rHandshakeBuilder<State extends BuilderState> {
  private constructor(
    private readonly builder: flatbuffers.Builder,
    private _state: State,
    private payloadType: F2rHandshakeRootPayload,
    private payloadOffset: number,
  ) {
    this.builder = builder;
  }

  static new(): F2rHandshakeBuilder<Initial> {
    return new F2rHandshakeBuilder<Initial>(new flatbuffers.Builder(), new Initial(), F2rHandshakeRootPayload.NONE, 0);
  }

  buildError(errorType: F2rHandshakeErrorType, errorMessage?: string): F2rHandshakeBuilder<Built> {
    const errorMessageOffset = this.builder.createString(errorMessage || "");
    const errorOffset = F2rHandshakeError.createF2rHandshakeError(this.builder, errorType, errorMessageOffset);

    return new F2rHandshakeBuilder<Built>(this.builder, new Built(), F2rHandshakeRootPayload.Error, errorOffset);
  }

  buildRequestConnection(deviceName: string): F2rHandshakeBuilder<Built> {
    const deviceNameOffset = this.builder.createString(deviceName);
    const toAgentOffset = F2rHandshakeRequestConnection.createF2rHandshakeRequestConnection(
      this.builder,
      deviceNameOffset,
    );

    return new F2rHandshakeBuilder<Built>(
      this.builder,
      new Built(),
      F2rHandshakeRootPayload.RequestConnection,
      toAgentOffset,
    );
  }

  toFlatbuffers(): F2rHandshakeRootBlob {
    const versionOffset = Version.createVersion(
      this.builder,
      CONFIG.version.major,
      CONFIG.version.minor,
      CONFIG.version.patch,
    );
    const rootOffset = F2rHandshakeRoot.createF2rHandshakeRoot(
      this.builder,
      versionOffset,
      this.payloadType,
      this.payloadOffset,
    );
    this.builder.finish(rootOffset);
    return new F2rHandshakeRootBlob(this.builder.asUint8Array());
  }
}
