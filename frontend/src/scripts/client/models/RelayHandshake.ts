import { F2rHandshakeBuilder } from "../serialisers/F2rHandshakeBuilder.ts";
import { CONFIG } from "../config.ts";
import { readR2fHandshakeRoot } from "../parsers/readR2fHandshake.ts";
import type { R2fHandshakeRoot } from "../../../generated/flatbuffers_schema/handshake_v1/r2f-handshake-root.ts";
import { R2fHandshakeRootPayload } from "../../../generated/flatbuffers_schema/handshake_v1/r2f-handshake-root-payload.ts";
import { R2fHandshakeSuccess } from "../../../generated/flatbuffers_schema/handshake_v1/r2f-handshake-success.ts";
import { HandshakeError } from "../errors.ts";
import type { Relay } from "./Relay.ts";
import { R2fHandshakeError } from "../../../generated/flatbuffers_schema/handshake_v1/r2f-handshake-error.ts";
import type { R2fHandshakeDevice } from "../../../generated/flatbuffers_schema/handshake_v1/handshake_v1.ts";
import { compact } from "lodash-es";

export class RelayHandshake {
  static async new(): Promise<RelayHandshake> {
    const relay = CONFIG.randomRelay();
    return new RelayHandshake(relay);
  }

  private constructor(private relay: Relay) {}

  async initiateConnectionRequest(deviceName: string): Promise<{ nonce: string; relay: Relay; subnames: string[] }> {
    const url = this.relay.handshakeUrl();
    const message = F2rHandshakeBuilder.new().buildRequestConnection(deviceName).toFlatbuffers();

    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/octet-stream",
      },
      body: message.data(),
    });

    const buffer = await response.arrayBuffer();
    const handshakeResponse = readR2fHandshakeRoot(new Uint8Array(buffer));

    const result = await this.processResponse(handshakeResponse);
    return { ...result, relay: this.relay };
  }

  private async processResponse(handshakeResponse: R2fHandshakeRoot): Promise<{ nonce: string; subnames: string[] }> {
    switch (handshakeResponse.rootPayloadType()) {
      case R2fHandshakeRootPayload.Success:
        const payload = handshakeResponse.rootPayload(new R2fHandshakeSuccess()) as R2fHandshakeSuccess | null;
        const nonce = payload?.relayAuthNonce();
        if (!payload || !nonce) {
          throw new HandshakeError("Relay responded with an empty nonce");
        }
        const devices: R2fHandshakeDevice[] = [];
        for (let i = 0; i < payload.devicesLength(); i++) {
          const device = payload.devices(i);
          if (device) {
            devices.push(device);
          }
        }

        return {
          nonce,
          subnames: compact(devices.map((device) => device.subname())),
        };
      case R2fHandshakeRootPayload.Error:
        const payload2 = handshakeResponse.rootPayload(new R2fHandshakeError()) as R2fHandshakeError | null;
        throw new HandshakeError(`Handshake Error: ${payload2?.errorType()}: ${payload2?.errorMessage()}`);
      case R2fHandshakeRootPayload.NONE:
        throw new HandshakeError("Invalid NONE response from Relay");
    }
  }
}
