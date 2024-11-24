import "@xterm/xterm/css/xterm.css";
import { Session } from "./Session.ts";
import { Relay } from "./relay.ts";

const initTerminal = (elementID: string) => {
  const $element = document.getElementById(elementID);

  if (!$element) {
    console.error("Webterm: Terminal element not found");
    return;
  }

  const urlParams = new URLSearchParams(window.location.search);
  const handshakeNonce = urlParams.get("handshake_nonce");
  const relay_host = urlParams.get("relay");

  if (!handshakeNonce) {
    alert("Handshake failed, please try again");
    return;
  }

  if (!relay_host) {
    alert("Relay not found");
    return;
  }

  const relay = new Relay(relay_host);

  console.log("Connecting to relay:", relay.websocketUrl(handshakeNonce));
  new Session(relay.websocketUrl(handshakeNonce), $element);
};

initTerminal("wt-terminal");
