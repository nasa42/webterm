import {
  createFrontendToAgentMessage,
  createFrontendToRelayMessage,
  createResizeData,
  readAgentToFrontendMessage,
  readRelayToFrontendMessage,
} from "./flatbuffers_helpers.ts";
import { Terminal } from "@xterm/xterm";
import { FrontendToAgentMessageType, FrontendToRelayMessageType } from "../../generated/flatbuffers_schema/schema.ts";

export class RelayConnection {
  private readonly socket: WebSocket;
  private readonly terminal;
  private readonly encoder = new TextEncoder();

  constructor(socket: WebSocket, terminal: Terminal) {
    this.terminal = terminal;
    this.socket = socket;
    this.socket.binaryType = "arraybuffer";
    this.registerEventListeners();
  }

  private registerEventListeners() {
    this.socket.addEventListener("open", () => this.onOpen());
    this.socket.addEventListener("message", (event) => this.onMessage(event));
    this.socket.addEventListener("close", () => this.onClose());
    this.socket.addEventListener("error", (error) => this.onError(error));
  }

  private onOpen() {
    console.log("Connected to WebSocket connection...");
    this.spawnTerminalOnAgent();
  }

  private onMessage(event: MessageEvent<ArrayBuffer | string>) {
    if (typeof event.data === "string") {
      this.terminal.write(event.data);
      return;
    }

    const fromRelay = readRelayToFrontendMessage(new Uint8Array(event.data));
    if (!fromRelay) {
      return;
    }

    const fromAgent = readAgentToFrontendMessage(fromRelay.data);
    if (!fromAgent) {
      return;
    }

    console.log("Received message from agent:", fromAgent);
    this.terminal.write(fromAgent.data);
  }

  private onClose() {
    this.terminal.writeln("Connection closed.");
    this.terminal.dispose();
  }

  private onError(error: Event) {
    console.error(error);
    this.terminal.writeln(`WebSocket error: ${error}`);
  }

  dispatchToAgent(type: FrontendToAgentMessageType, data: string | Uint8Array) {
    if (typeof data === "string") {
      data = this.encoder.encode(data);
    }

    const innerMessage = createFrontendToAgentMessage(type, data);

    const outerMessage = createFrontendToRelayMessage(FrontendToRelayMessageType.ToAgent, innerMessage);
    this.socket.send(outerMessage);
  }

  dispatchData(data: string) {
    this.dispatchToAgent(FrontendToAgentMessageType.Data, data);
  }

  dispatchResize(cols: number, rows: number) {
    const resizeMessage = createResizeData(cols, rows);
    this.dispatchToAgent(FrontendToAgentMessageType.Resize, resizeMessage);
  }

  spawnTerminalOnAgent() {
    this.dispatchToAgent(FrontendToAgentMessageType.SpawnTerminal, "");
  }
}
