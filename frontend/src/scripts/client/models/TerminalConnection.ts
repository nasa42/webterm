import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import type { BinaryLike } from "../types/BinaryLike.ts";

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
    // console.log(`writing: "${formatPtyOutput(ensureBinary(data))}"`);

    this.terminal.write(data);
  }

  resizeToFit() {
    this.fitAddon.fit();
    console.log(`Resized to ${this.terminal.cols}x${this.terminal.rows}`);
  }

  terminalCols(): number {
    return this.terminal.cols;
  }

  terminalRows(): number {
    return this.terminal.rows;
  }
}
