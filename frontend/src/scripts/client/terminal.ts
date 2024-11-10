import "@xterm/xterm/css/xterm.css";
import { Session } from "./Session.ts";

(() => {
  const $element = document.getElementById("wt-terminal");

  if (!$element) {
    return;
  }

  new Session("ws://localhost:3000/ws", $element);
})();
