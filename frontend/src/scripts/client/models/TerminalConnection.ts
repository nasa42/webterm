import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";

export class TerminalConnection {
  private readonly terminal: Terminal;
  private readonly fitAddon: FitAddon;

  constructor(
    element: HTMLElement,
    private readonly onData: (data: string) => void,
  ) {
    this.terminal = new Terminal();
    this.fitAddon = new FitAddon();
    this.terminal.loadAddon(this.fitAddon);
    this.terminal.open(element);
    this.registerEventListeners();
  }

  private registerEventListeners() {
    this.terminal.onData((data) => this.onData(data));
  }

  write(data: BinaryLike) {
    this.terminal.write(data);
  }

  terminalCols(): number {
    return this.terminal.cols;
  }

  terminalRows(): number {
    return this.terminal.rows;
  }
}
