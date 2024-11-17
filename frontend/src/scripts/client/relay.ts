export class RelayParseError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "RelayParseError";
  }
}

export class Relay {
  host: string;
  useHttp: boolean;

  constructor(host: string) {
    let hostWithScheme = host;
    if (!host.includes("://")) {
      hostWithScheme = "https://" + host;
    }

    const parsedUrl = new URL(hostWithScheme);
    this.useHttp = parsedUrl.protocol === "http:";

    if (parsedUrl.protocol !== "http:" && parsedUrl.protocol !== "https:") {
      throw new RelayParseError(`Invalid relay URL scheme: ${parsedUrl.protocol}`);
    }

    this.host = parsedUrl.hostname;
    if (!this.host) {
      throw new RelayParseError(`Couldn't extract host from relay URL: ${host}`);
    }

    if (parsedUrl.port) {
      this.host += `:${parsedUrl.port}`;
    }
  }

  websocketUrl(handshakeNonce: string): string {
    const scheme = this.useHttp ? "ws" : "wss";
    return `${scheme}://${this.host}/ws/frontend?handshake_nonce=${handshakeNonce}`;
  }

  authUrl(): string {
    const scheme = this.useHttp ? "http" : "https";
    return `${scheme}://${this.host}/auth/frontend`;
  }
}
