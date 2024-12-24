import * as flatbuffers from "flatbuffers";
import {
  F2rError,
  F2rErrorType,
  F2rRoot,
  F2rRootPayload,
  F2rToAgent,
} from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { type F2aRootBlob, F2rRootBlob } from "../types/BinaryBlob.ts";

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface BuilderState {}

class Initial implements BuilderState {
  _type = "Initial" as const;
}

export class Built implements BuilderState {
  _type = "Built" as const;
}

export class F2rBuilder<State extends BuilderState> {
  private constructor(
    private readonly builder: flatbuffers.Builder,
    private _state: State,
    private payloadType: F2rRootPayload,
    private payloadOffset: number,
  ) {
    this.builder = builder;
  }

  static new(): F2rBuilder<Initial> {
    return new F2rBuilder<Initial>(new flatbuffers.Builder(), new Initial(), F2rRootPayload.NONE, 0);
  }

  buildError(errorType: F2rErrorType, errorMessage?: string): F2rBuilder<Built> {
    const errorMessageOffset = this.builder.createString(errorMessage || "");
    const errorOffset = F2rError.createF2rError(this.builder, errorType, errorMessageOffset);

    return new F2rBuilder<Built>(this.builder, new Built(), F2rRootPayload.Error, errorOffset);
  }

  buildToAgent(payload: F2aRootBlob): F2rBuilder<Built> {
    const payloadOffset = this.builder.createByteVector(payload.data());
    const toAgentOffset = F2rToAgent.createF2rToAgent(this.builder, payloadOffset);

    return new F2rBuilder<Built>(this.builder, new Built(), F2rRootPayload.ToAgent, toAgentOffset);
  }

  toFlatbuffers(): F2rRootBlob {
    const rootOffset = F2rRoot.createF2rRoot(this.builder, this.payloadType, this.payloadOffset);
    this.builder.finish(rootOffset);
    return new F2rRootBlob(this.builder.asUint8Array());
  }
}
