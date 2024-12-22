function processString(input: string): string {
  return [...input]
    .map((char) => {
      switch (char) {
        case "\x08":
          return "^H";
        case "\x0A":
          return "\n";
        case "\x0C":
          return "^L";
        case "\x0D":
          return "\r";
        case "\x1B":
          return "^[ ";
        case "\x7F":
          return "^?";
        default:
          if (char <= "\x1F") {
            return "^" + String.fromCharCode(char.charCodeAt(0) + 0x40);
          }
          return char;
      }
    })
    .join("");
}

export function formatPtyOutput(bytes: Uint8Array): string {
  let result = "";

  let i = 0;
  while (i < bytes.length) {
    const byte = bytes[i]!;

    // Attempting to decode as UTF-8
    try {
      const decoder = new TextDecoder("utf-8", { fatal: true });
      const validString = decoder.decode(bytes.slice(i));
      result += processString(validString);
      break; // If it successfully decodes, we can break since it's all valid
    } catch (_) {
      // If invalid byte sequence, handle as a single byte
      result += "\\x" + byte.toString(16).padStart(2, "0");
      i++;
    }
  }

  return result;
}
