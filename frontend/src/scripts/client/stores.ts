import { atom } from "nanostores";
import type { Relay } from "./models/Relay.ts";
import type { NotificationVariant } from "./ui/NotificationManager.ts";

export const activeNotificationStore = atom<{
  message: string;
  variant: NotificationVariant;
} | null>(null);

export const notificationListSignal = atom(0);

export const handshakeInitiateSignal = atom<{ deviceName: string } | null>(null);
export const handshakeCompleteSignal = atom<{ nonce: string; relay: Relay; deviceSubname: string } | null>(null);

export const currentTerminalStore = atom<{ deviceName: string; deviceSubname: string } | null>(null);
