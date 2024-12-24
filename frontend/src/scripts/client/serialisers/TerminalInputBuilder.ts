import * as flatbuffers from "flatbuffers";
import { VectorTable } from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { ActivityInputBlob } from "../types/BinaryBlob.ts";
import { PtyInput, PtyInputRoot, PtyResize } from "../../../generated/flatbuffers_schema/talk_v1/activity.ts";

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface BuilderState {}

class Initial implements BuilderState {
  _type = "Initial" as const;
}

export class Built implements BuilderState {
  _type = "Built" as const;
}

export class TerminalInputBuilder<State extends BuilderState> {
  private constructor(
    private readonly builder: flatbuffers.Builder,
    private _state: State,
    private payloadType: PtyInput,
    private payloadOffset: number,
  ) {
    this.builder = builder;
  }

  static new(): TerminalInputBuilder<Initial> {
    return new TerminalInputBuilder<Initial>(new flatbuffers.Builder(), new Initial(), PtyInput.NONE, 0);
  }

  buildUserInput(raw_input: Uint8Array): TerminalInputBuilder<Built> {
    const raw_input_offset = this.builder.createByteVector(raw_input);
    const offset = VectorTable.createVectorTable(this.builder, raw_input_offset);

    return new TerminalInputBuilder<Built>(this.builder, new Built(), PtyInput.UserInput, offset);
  }

  buildResize(cols: number, rows: number): TerminalInputBuilder<Built> {
    const offset = PtyResize.createPtyResize(this.builder, cols, rows);

    return new TerminalInputBuilder<Built>(this.builder, new Built(), PtyInput.Resize, offset);
  }

  toFlatbuffers(): ActivityInputBlob {
    const rootOffset = PtyInputRoot.createPtyInputRoot(this.builder, this.payloadType, this.payloadOffset);
    this.builder.finish(rootOffset);
    return new ActivityInputBlob(this.builder.asUint8Array());
  }
}
