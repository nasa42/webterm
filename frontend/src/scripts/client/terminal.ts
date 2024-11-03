import "@xterm/xterm/css/xterm.css";
import { Terminal } from "@xterm/xterm";

(() => {
  const $element = document.getElementById("terminal");

  if (!$element) {
    return;
  }

  const term = new Terminal();
  term.open($element);
  term.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ");
})();
