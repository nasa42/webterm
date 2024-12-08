import * as flatbuffers from "flatbuffers";
import { VectorTable } from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { ActivityInputBlob } from "../types/BinaryBlob.ts";
import {
  TerminalInput,
  TerminalInputRoot,
  TerminalResize,
} from "../../../generated/flatbuffers_schema/talk_v1/activity.ts";

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
    private payloadType: TerminalInput,
    private payloadOffset: number,
  ) {
    this.builder = builder;
  }

  static new(): TerminalInputBuilder<Initial> {
    return new TerminalInputBuilder<Initial>(new flatbuffers.Builder(), new Initial(), TerminalInput.NONE, 0);
  }

  buildUserInput(raw_input: Uint8Array): TerminalInputBuilder<Built> {
    const raw_input_offset = this.builder.createByteVector(raw_input);
    const offset = VectorTable.createVectorTable(this.builder, raw_input_offset);

    return new TerminalInputBuilder<Built>(this.builder, new Built(), TerminalInput.UserInput, offset);
  }

  buildResize(cols: number, rows: number): TerminalInputBuilder<Built> {
    const offset = TerminalResize.createTerminalResize(this.builder, cols, rows);

    return new TerminalInputBuilder<Built>(this.builder, new Built(), TerminalInput.Resize, offset);
  }

  toFlatbuffers(): ActivityInputBlob {
    const rootOffset = TerminalInputRoot.createTerminalInputRoot(this.builder, this.payloadType, this.payloadOffset);
    this.builder.finish(rootOffset);
    return new ActivityInputBlob(this.builder.asUint8Array());
  }
}
