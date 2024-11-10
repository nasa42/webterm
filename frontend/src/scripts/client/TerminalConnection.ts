import type { Terminal } from "@xterm/xterm";
import type { RelayConnection } from "./RelayConnection.ts";
import { debounce } from "lodash";

export class TerminalConnection {
  constructor(
    private readonly relayConnection: RelayConnection,
    private readonly terminal: Terminal,
    private readonly fitAddon,
  ) {
    this.registerEventListeners();
  }

  private registerEventListeners() {
    this.terminal.onData((data) => this.onData(data));
    window.addEventListener(
      "resize",
      debounce(() => this.onWindowResize(), 500),
    );
  }

  private onData(data: string) {
    this.relayConnection.dispatchData(data);
  }

  private onWindowResize() {
    this.relayConnection.dispatchResize(this.terminal.cols, this.terminal.rows);
  }
}
