import "@xterm/xterm/css/xterm.css";
import { Runner } from "../models/Runner.ts";
import { ELLIPSIS } from "../config.ts";
import { StoredCredential } from "../models/StoredCredential.ts";
import { handshakeCompleteSignal, handshakeInitiateSignal } from "../stores.ts";
import { alertAndThrow } from "../functions/alertAndThrow.ts";
import { notificationManager } from "../ui/NotificationManager.ts";

const init = async (elementID: string) => {
  const searchParams = new URLSearchParams(window.location.search);
  const secretKey = searchParams.get("store_key");
  const storeIndex = searchParams.get("store_index");

  if (!secretKey || !storeIndex) {
    alert("Handshake failed: Invalid params, please try again");
    window.location.href = "/";
    return;
  }

  const creds = await StoredCredential.retrieve(parseInt(storeIndex), secretKey);

  handshakeInitiateSignal.set({ deviceName: creds.serverId });

  handshakeCompleteSignal.subscribe((result) => {
    if (!result) return;

    const { nonce, relay, deviceSubname } = result;
    const $element = document.getElementById(elementID);

    if (!$element) {
      alertAndThrow("Webterm: Terminal element not found");
      return;
    }

    const url = relay.websocketUrl(nonce, deviceSubname);
    console.log("Connecting to relay:", url);
    notificationManager.setActive(`Connecting${ELLIPSIS}`, "info");
    new Runner(url, $element, creds.serverId, creds.serverPassword);
  });
};

await init("app-terminal");
