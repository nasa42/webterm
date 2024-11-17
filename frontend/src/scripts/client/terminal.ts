import "@xterm/xterm/css/xterm.css";
import { Session } from "./Session.ts";
import { CONFIG } from "./config.ts";

const initTerminal = (elementID: string) => {
  const $element = document.getElementById(elementID);

  if (!$element) {
    console.error("Webterm: Terminal element not found");
    return;
  }

  const relay = CONFIG.defaultRelays[0];

  if (!relay) {
    console.error("Webterm: No relay found");
    return;
  }

  const urlParams = new URLSearchParams(window.location.search);
  const handshakeNonce = urlParams.get("handshake_nonce");

  if (!handshakeNonce) {
    alert("Handshake failed, please try again");
    return;
  }

  console.log("Connecting to relay:", relay.websocketUrl(handshakeNonce));
  new Session(relay.websocketUrl(handshakeNonce), $element);
};

initTerminal("wt-terminal");
