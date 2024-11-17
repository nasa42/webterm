import { Relay } from "./relay.ts";

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
