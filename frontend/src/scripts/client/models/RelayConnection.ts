import { EncryptionReady, F2aBuilder, PlainReady } from "../serialisers/F2aBuilder.ts";
import { Built, F2rBuilder } from "../serialisers/F2rBuilder.ts";
import { ensureBinary } from "../functions/ensureBinary.ts";
import type { ActivityId } from "../types/BigIntLike.ts";
import { TerminalInputBuilder } from "../serialisers/TerminalInputBuilder.ts";

export class RelayConnection {
  constructor(
    private readonly socket: WebSocket,
    private readonly onMessage: (event: MessageEvent) => void,
  ) {
    this.socket = socket;
    this.socket.binaryType = "arraybuffer";
    this.registerEventListeners();
  }

  private registerEventListeners() {
    this.socket.addEventListener("open", (event) => this.onOpen(event));
    this.socket.addEventListener("message", (event) => this.onMessage(event));
    this.socket.addEventListener("close", (event) => this.onClose(event));
    this.socket.addEventListener("error", (error) => this.onError(error));
  }

  private onOpen(event: Event) {
    console.log("Connected to WebSocket connection...");
    this.initiateAuthentication();
  }

  private onClose(_event: CloseEvent) {
    console.info("Relay connection closed");
  }

  private onError(error: Event) {
    console.error(error);
  }

  private dispatchToRelay(f2r: F2rBuilder<Built>) {
    let payload = f2r.toFlatbuffers();
    this.socket.send(payload.data());
  }

  dispatchToAgentEncrypted(payload: F2aBuilder<EncryptionReady>) {
    console.log("Dispatching message to agent:");
    let f2a = payload.toFlatbuffersEncrypted();
    this.dispatchToRelay(F2rBuilder.new().buildToAgent(f2a));
  }

  dispatchToAgentPlain(payload: F2aBuilder<PlainReady>) {
    console.log("Dispatching message to agent:");
    this.dispatchToRelay(F2rBuilder.new().buildToAgent(payload.toFlatbuffersPlain()));
  }

  dispatchTerminalUserInput(activityId: ActivityId, raw_data: BinaryLike) {
    let tb = TerminalInputBuilder.new();
    let input_ = tb.buildUserInput(ensureBinary(raw_data)).toFlatbuffers();
    let f2a = F2aBuilder.new();
    let payload = f2a.buildActivityInputMessage(activityId, input_);

    this.dispatchToAgentEncrypted(payload);
  }

  dispatchResize(_cols: number, _rows: number) {
    throw new Error("Not implemented");
  }

  initiateAuthentication() {
    console.debug("Initiating authentication...");
    let f2a = F2aBuilder.new();
    this.dispatchToAgentPlain(f2a.buildAuthRequestPreamble());
  }
}
