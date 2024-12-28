import "@xterm/xterm/css/xterm.css";
import { Runner } from "../models/Runner.ts";
import { activeNotification } from "../ui/ActiveNotificationManager.ts";
import { CONFIG, ELLIPSIS } from "../config.ts";
import { createF2rHandshake } from "../functions/createF2rHandshake.ts";
import { readR2fHandshake } from "../parsers/readR2fHandshake.ts";
import { StoredCredential } from "../models/StoredCredential.ts";

const init = async (elementID: string) => {
  const relay = CONFIG.defaultRelays[0];
  if (!relay) {
    console.error("Webterm: Define default relays with environment variable PUBLIC_DEFAULT_RELAYS in .env");
    return;
  }

  const searchParams = new URLSearchParams(window.location.search);
  const secretKey = searchParams.get("store_key");
  const storeIndex = searchParams.get("store_index");

  if (!secretKey || !storeIndex) {
    alert("Handshake failed: Invalid params, please try again");
    window.location.href = "/";
    return;
  }

  const creds = await StoredCredential.retrieve(parseInt(storeIndex), secretKey);

  const url = relay.handshakeUrl();
  const message = createF2rHandshake(creds.serverId);

  let handshakeNonce;

  try {
    activeNotification.show(`Establishing handshake${ELLIPSIS}`, "info");
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/octet-stream",
      },
      body: message,
    });

    const buffer = await response.arrayBuffer();
    const handshakeResponse = readR2fHandshake(new Uint8Array(buffer));
    handshakeNonce = handshakeResponse.relayAuthNonce();

    if (!handshakeResponse.success() || !handshakeNonce) {
      throw new Error("Handshake failed");
    }
  } catch (error) {
    console.error("Connection failed:", error);
    alert("Connection failed, please try again");
    return;
  }

  if (!handshakeNonce) {
    alert("Handshake failed, please try again");
    return;
  }

  const $element = document.getElementById(elementID);

  if (!$element) {
    console.error("Webterm: Terminal element not found");
    return;
  }

  console.log("Connecting to relay:", relay.websocketUrl(handshakeNonce));
  activeNotification.show(`Connecting${ELLIPSIS}`, "info");
  new Runner(relay.websocketUrl(handshakeNonce), $element, creds.serverId, creds.serverPassword);
};

await init("app-terminal");
