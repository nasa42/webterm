import { CONFIG } from "./config.ts";
import { createF2rHandshake } from "./functions/createF2rHandshake.ts";
import { readR2fHandshake } from "./functions/readR2fHandshake.ts";

const init = () => {
  const $connectButton = document.getElementById("wt-connect-button");

  if (!$connectButton) {
    console.error("Webterm: Connect button not found");
    return;
  }

  const relay = CONFIG.defaultRelays[0];

  if (!relay) {
    console.error("Webterm: No relay found");
    return;
  }

  $connectButton.addEventListener("click", () => {
    const url = relay.handshakeUrl();
    console.log("Connecting to relay:", url);

    const message = createF2rHandshake();

    fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/octet-stream",
      },
      body: message,
    })
      .then((response) => response.arrayBuffer())
      .then((buffer) => {
        const handshakeResponse = readR2fHandshake(new Uint8Array(buffer));
        const nonce = handshakeResponse.relayAuthNonce();

        console.log("Handshake successful:", handshakeResponse.success());

        if (nonce) {
          window.location.href = `/terminal?handshake_nonce=${nonce}&relay=${relay.originalHost}`;
        } else {
          console.log(handshakeResponse);
          throw new Error("No handshake nonce received");
        }
      })
      .catch((error) => {
        console.error("Handshake failed:", error);
        alert("Handshake failed, please try again");
      });
  });
};

init();
