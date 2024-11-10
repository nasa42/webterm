import { Terminal } from "@xterm/xterm";
import { RelayConnection } from "./RelayConnection.ts";
import { TerminalConnection } from "./TerminalConnection.ts";
import { FitAddon } from "@xterm/addon-fit";

export class Session {
  private readonly socket: WebSocket;
  private readonly terminal: Terminal;
  private readonly fitAddon: FitAddon;
  private readonly relayConnection: RelayConnection;

  constructor(url: string, element: HTMLElement) {
    this.socket = new WebSocket(url);
    this.terminal = new Terminal();
    this.fitAddon = new FitAddon();
    this.initTerminal(element);
    this.relayConnection = new RelayConnection(this.socket, this.terminal);
    new TerminalConnection(this.relayConnection, this.terminal);
  }

  private initTerminal(element: HTMLElement) {
    this.terminal.loadAddon(this.fitAddon);
    this.terminal.open(element);
  }
}
