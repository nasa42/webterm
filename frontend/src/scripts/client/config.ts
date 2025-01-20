import { Relay } from "./models/Relay.ts";

// should be same as in agent/src/config.rs
const DEFAULT_RELAYS = [
  "r1.relays.webterm.run",
  "r2.relays.webterm.run",
  "r4.relays.webterm.run",
  "r5.relays.webterm.run",
];

const VERSION = {
  major: parseInt(import.meta.env.WEBTERM_VERSION_MAJOR || "0"),
  minor: parseInt(import.meta.env.WEBTERM_VERSION_MINOR || "0"),
  patch: parseInt(import.meta.env.WEBTERM_VERSION_PATCH || "0"),
};

const repoURL = "https://github.com/nasa42/webterm";

export const ELLIPSIS = "…";

const defaultRelays = (): Relay[] => {
  const fromEnv: string | null | undefined = import.meta.env.PUBLIC_DEFAULT_RELAYS;
  let relayStrings: string[] = [];

  if (fromEnv) {
    relayStrings = fromEnv
      .split(",")
      .map((relay) => relay.trim())
      .filter((relay) => relay !== "");
  }

  if (relayStrings.length === 0) {
    relayStrings = DEFAULT_RELAYS;
  }

  return relayStrings.map((relay) => new Relay(relay));
};

const gitCommit = import.meta.env.CF_PAGES_COMMIT_SHA;

export const CONFIG = {
  defaultRelays: defaultRelays(),
  version: VERSION,
  gitCommit,
  deploymentCommitURL: () => `${repoURL}/commit/${gitCommit}`,
};
