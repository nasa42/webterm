import { Relay } from "./models/Relay.ts";

export const TEST_SERVER_ID = "test";
export const VERSION = {
  major: parseInt(import.meta.env.WEBTERM_VERSION_MAJOR || "0"),
  minor: parseInt(import.meta.env.WEBTERM_VERSION_MINOR || "0"),
  patch: parseInt(import.meta.env.WEBTERM_VERSION_PATCH || "0"),
};

export const ELLIPSIS = "…";

const defaultRelays = (): Relay[] => {
  const fromEnv: string | null | undefined = import.meta.env.PUBLIC_DEFAULT_RELAYS;

  if (!fromEnv || fromEnv === "") {
    return [];
  }

  return fromEnv
    .split(",")
    .map((relay) => relay.trim())
    .filter((relay) => relay !== "")
    .map((relay) => new Relay(relay));
};

export const CONFIG = {
  defaultRelays: defaultRelays(),
};
