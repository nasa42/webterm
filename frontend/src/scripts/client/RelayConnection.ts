import {
  createF2aMessage,
  createF2rMessage,
  createResizeData,
  readA2fMessage,
  readR2fMessage,
} from "./talk_v1_helpers.ts";
import { Terminal } from "@xterm/xterm";
import { F2aMessageType, F2rMessageType } from "../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";

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

    const fromRelay = readR2fMessage(new Uint8Array(event.data));
    if (!fromRelay) {
      return;
    }

    const fromAgent = readA2fMessage(fromRelay.dataArray() ?? new Uint8Array());
    if (!fromAgent) {
      return;
    }

    console.log("Received message from agent:", fromAgent);
    this.terminal.write(fromAgent.dataArray() ?? new Uint8Array());
  }

  private onClose() {
    this.terminal.writeln("Connection closed.");
    this.terminal.dispose();
  }

  private onError(error: Event) {
    console.error(error);
    this.terminal.writeln(`WebSocket error: ${error}`);
  }

  dispatchToAgent(type: F2aMessageType, data: string | Uint8Array) {
    console.log("Dispatching message to agent:", type);
    if (typeof data === "string") {
      data = this.encoder.encode(data);
    }

    const innerMessage = createF2aMessage(type, data);

    const outerMessage = createF2rMessage(F2rMessageType.ToAgent, innerMessage);
    this.socket.send(outerMessage);
  }

  dispatchData(data: string) {
    this.dispatchToAgent(F2aMessageType.ActivityInput, data);
  }

  dispatchResize(cols: number, rows: number) {
    const resizeMessage = createResizeData(cols, rows);
    this.dispatchToAgent(F2aMessageType.TerminalResize, resizeMessage);
  }

  spawnTerminalOnAgent() {
    console.debug("Spawning terminal on agent...");
    this.dispatchToAgent(F2aMessageType.ActivityCreateTerminal, "");
  }
}
