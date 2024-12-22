import { RelayConnection } from "./RelayConnection.ts";
import { TerminalConnection } from "./TerminalConnection.ts";
import { readR2fRoot } from "../parsers/readR2fRoot.ts";
import { SendPayload } from "./SendPayload.ts";
import { debounce } from "lodash-es";
import type { ActivityId } from "../types/BigIntLike.ts";
import { processR2f } from "../pipeline/processR2f.ts";
import { Cryptographer } from "../cryptography/Cryptographer.ts";
import type { Bits256Array } from "../types/BitsArray.ts";

export class Runner {
  private readonly socket: WebSocket;
  private readonly relayConnection: RelayConnection;
  private readonly terminalConnection: TerminalConnection;
  private currentActivityId?: ActivityId;
  private cryptographer_?: Cryptographer;

  constructor(
    url: string,
    element: HTMLElement,
    private readonly serverId: string,
    private readonly serverPassword: string,
  ) {
    this.socket = new WebSocket(url);
    this.relayConnection = new RelayConnection(
      this,
      this.socket,
      async (event) => await this.onWebsocketMessage(event),
    );
    this.terminalConnection = new TerminalConnection(element, (data) => this.onTerminalUserInput(data));
    window.addEventListener(
      "resize",
      debounce(() => this.onWindowResize(), 500),
    );
  }

  private async onWebsocketMessage(event: MessageEvent) {
    const r2fRoot = readR2fRoot(new Uint8Array(event.data));
    let send = new SendPayload(this);

    try {
      await processR2f(r2fRoot, send);
    } catch (error) {
      console.error(error);
      return;
    }

    if (send.toTerminal) {
      this.terminalConnection.write(send.toTerminal);
    }

    if (send.receivedActivityId) {
      if (this.currentActivityId?.int() != send.receivedActivityId.int()) {
        this.currentActivityId = send.receivedActivityId;
        await this.onActivityInit();
      }
    }

    if (send.toAgentPlain) {
      this.relayConnection.dispatchToAgentPlain(send.toAgentPlain);
    }

    if (send.toAgentEncrypted) {
      await this.relayConnection.dispatchToAgentEncrypted(send.toAgentEncrypted);
    }
  }

  private async onTerminalUserInput(data: string) {
    if (!this.currentActivityId) {
      throw new Error("Did not expect ActivityId to be undefined");
    }

    await this.relayConnection.dispatchTerminalUserInput(this.currentActivityId, data);
  }

  private async onWindowResize() {
    if (!this.currentActivityId) {
      throw new Error("Did not expect ActivityId to be undefined");
    }

    this.terminalConnection.resizeToFit();

    await this.relayConnection.dispatchResize(
      this.currentActivityId,
      this.terminalConnection.terminalCols(),
      this.terminalConnection.terminalRows(),
    );
  }

  private async onActivityInit() {
    console.log(`Initialising activity ${this.currentActivityId}...`);
    await this.onWindowResize();
  }

  async initCryptographer({ iterations, salt }: { iterations: number; salt: Bits256Array }) {
    const { cryptographer } = await Cryptographer.new({ iterations, salt, secretKey: this.serverPassword });
    this.cryptographer_ = cryptographer;
  }

  cryptographer(): Cryptographer {
    if (!this.cryptographer_) {
      throw new Error("Cryptographer not initialised");
    }
    return this.cryptographer_;
  }
}
