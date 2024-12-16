import { EncryptionReady, F2aBuilder, PlainReady } from "../serialisers/F2aBuilder.ts";
import { Built, F2rBuilder } from "../serialisers/F2rBuilder.ts";
import { ensureBinary } from "../functions/ensureBinary.ts";
import type { ActivityId } from "../types/BigIntLike.ts";
import { TerminalInputBuilder } from "../serialisers/TerminalInputBuilder.ts";
import type { Runner } from "./Runner.ts";

export class RelayConnection {
  constructor(
    private readonly runner: Runner,
    private readonly socket: WebSocket,
    private readonly onMessage: (event: MessageEvent) => Promise<void>,
  ) {
    this.socket = socket;
    this.socket.binaryType = "arraybuffer";
    this.registerEventListeners();
  }

  private registerEventListeners() {
    this.socket.addEventListener("open", (event) => this.onOpen(event));
    this.socket.addEventListener("message", async (event) => await this.onMessage(event));
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

  async dispatchToAgentEncrypted(payload: F2aBuilder<EncryptionReady>) {
    console.log("Dispatching message to agent:");
    let f2a = await payload.toFlatbuffersEncrypted(this.runner.cryptographer());
    this.dispatchToRelay(F2rBuilder.new().buildToAgent(f2a));
  }

  dispatchToAgentPlain(payload: F2aBuilder<PlainReady>) {
    console.log("Dispatching message to agent:");
    this.dispatchToRelay(F2rBuilder.new().buildToAgent(payload.toFlatbuffersPlain()));
  }

  async dispatchTerminalUserInput(activityId: ActivityId, raw_data: BinaryLike) {
    let tb = TerminalInputBuilder.new();
    let input_ = tb.buildUserInput(ensureBinary(raw_data)).toFlatbuffers();
    let f2a = F2aBuilder.new();
    let payload = f2a.buildActivityInputMessage(activityId, input_);

    await this.dispatchToAgentEncrypted(payload);
  }

  async dispatchResize(activityId: ActivityId, cols: number, rows: number) {
    let tb = TerminalInputBuilder.new();
    let input_ = tb.buildResize(cols, rows).toFlatbuffers();
    let f2a = F2aBuilder.new();
    let payload = f2a.buildActivityInputMessage(activityId, input_);

    await this.dispatchToAgentEncrypted(payload);
  }

  initiateAuthentication() {
    console.debug("Initiating authentication...");
    let f2a = F2aBuilder.new();
    this.dispatchToAgentPlain(f2a.buildAuthRequestPreamble());
  }
}
