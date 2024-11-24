import { Relay } from "./relay.ts";

export const TEST_SERVER_ID = "test";

const defaultRelays = (): Relay[] => {
  const fromEnv: string | null | undefined = import.meta.env.PUBLIC_DEFAULT_RELAYS.toString();

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
