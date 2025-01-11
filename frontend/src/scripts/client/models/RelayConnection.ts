import { EncryptionReady, F2aBuilder, PlainReady } from "../serialisers/F2aBuilder.ts";
import { Built, F2rBuilder } from "../serialisers/F2rBuilder.ts";
import { ensureBinary } from "../functions/ensureBinary.ts";
import type { ActivityId } from "../types/BigIntLike.ts";
import { TerminalInputBuilder } from "../serialisers/TerminalInputBuilder.ts";
import type { Runner } from "./Runner.ts";
import type { BinaryLike } from "../types/BinaryLike.ts";

export class RelayConnection {
  // manage a queue of messages to ensure that they're processed sequentially in the order they arrive
  private messageQueue: MessageEvent[] = [];
  private processingQueue = false;

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
    this.socket.addEventListener("message", (event) => {
      this.messageQueue.push(event);
      this.processMessageQueue();
    });
    this.socket.addEventListener("close", (event) => this.onClose(event));
    this.socket.addEventListener("error", (error) => this.onError(error));
  }

  private async processMessageQueue() {
    if (this.processingQueue) return;

    try {
      this.processingQueue = true;
      while (this.messageQueue.length > 0) {
        const event = this.messageQueue.shift()!;
        await this.onMessage(event);
      }
    } finally {
      this.processingQueue = false;
    }
  }

  private onOpen(_event: Event) {
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
    const payload = f2r.toFlatbuffers();
    this.socket.send(payload.data());
  }

  async dispatchToAgentEncrypted(payload: F2aBuilder<EncryptionReady>) {
    console.log("Dispatching message to agent:");
    const f2a = await payload.toFlatbuffersEncrypted(this.runner.cryptographer());
    this.dispatchToRelay(F2rBuilder.new().buildToAgent(f2a));
  }

  dispatchToAgentPlain(payload: F2aBuilder<PlainReady>) {
    console.log("Dispatching message to agent:");
    this.dispatchToRelay(F2rBuilder.new().buildToAgent(payload.toFlatbuffersPlain()));
  }

  async dispatchTerminalUserInput(activityId: ActivityId, raw_data: BinaryLike) {
    const tb = TerminalInputBuilder.new();
    const input_ = tb.buildUserInput(ensureBinary(raw_data)).toFlatbuffers();
    const f2a = F2aBuilder.new();
    const payload = f2a.buildActivityInputMessage(activityId, input_);

    await this.dispatchToAgentEncrypted(payload);
  }

  async dispatchResize(activityId: ActivityId, cols: number, rows: number) {
    const tb = TerminalInputBuilder.new();
    const input_ = tb.buildResize(cols, rows).toFlatbuffers();
    const f2a = F2aBuilder.new();
    const payload = f2a.buildActivityInputMessage(activityId, input_);

    await this.dispatchToAgentEncrypted(payload);
  }

  initiateAuthentication() {
    console.debug("Initiating authentication...");
    const f2a = F2aBuilder.new();
    this.dispatchToAgentPlain(f2a.buildAuthRequestPreamble());
  }
}
