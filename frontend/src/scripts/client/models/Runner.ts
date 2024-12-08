import { RelayConnection } from "./RelayConnection.ts";
import { TerminalConnection } from "./TerminalConnection.ts";
import { readR2fRoot } from "../functions/readR2fRoot.ts";
import { SendPayload } from "./SendPayload.ts";
import { debounce } from "lodash";
import type { ActivityId } from "../types/BigIntLike.ts";
import { processR2f } from "../functions/processR2f.ts";

export class Runner {
  private readonly socket: WebSocket;
  private readonly relayConnection: RelayConnection;
  private readonly terminalConnection: TerminalConnection;
  private currentActivityId?: ActivityId;

  constructor(url: string, element: HTMLElement) {
    this.socket = new WebSocket(url);
    this.relayConnection = new RelayConnection(this.socket, (event) => this.onWebsocketMessage(event));
    this.terminalConnection = new TerminalConnection(element, (data) => this.onTerminalUserInput(data));
    window.addEventListener(
      "resize",
      debounce(() => this.onWindowResize(), 500),
    );
  }

  private onWebsocketMessage(event: MessageEvent) {
    const r2fRoot = readR2fRoot(new Uint8Array(event.data));
    let send = new SendPayload();

    processR2f(r2fRoot, send);

    if (send.toTerminal) {
      this.terminalConnection.write(send.toTerminal);
    }

    if (send.receivedActivityId) {
      this.currentActivityId = send.receivedActivityId;
    }

    if (send.toAgentPlain) {
      this.relayConnection.dispatchToAgentPlain(send.toAgentPlain);
    }

    if (send.toAgentEncrypted) {
      this.relayConnection.dispatchToAgentEncrypted(send.toAgentEncrypted);
    }
  }

  private onTerminalUserInput(data: string) {
    if (!this.currentActivityId) {
      throw new Error("Did not expect ActivityId to be undefined");
    }

    this.relayConnection.dispatchTerminalUserInput(this.currentActivityId, data);
  }

  private onWindowResize() {
    this.relayConnection.dispatchResize(this.terminalConnection.terminalCols(), this.terminalConnection.terminalRows());
  }
}
