import { activeNotificationStore, notificationListSignal } from "../stores.ts";

const MAX_NOTIFICATIONS = 100;

export type NotificationVariant =
  | "primary"
  | "secondary"
  | "success"
  | "danger"
  | "warning"
  | "info"
  | "light"
  | "dark";

export interface Notification {
  id: string;
  message: string;
  timestamp: Date;
  muted: boolean;
}

class NotificationManagerClass {
  private notifications: Notification[] = [];

  list(): Notification[] {
    return this.notifications;
  }

  setActive(message: string, variant: NotificationVariant) {
    activeNotificationStore.set({
      message,
      variant,
    });
  }

  clearActive() {
    activeNotificationStore.set(null);
  }

  addNotification(message: string, muted: boolean = false): void {
    const notification: Notification = {
      id: crypto.randomUUID(),
      message,
      timestamp: new Date(),
      muted,
    };
    this.notifications.unshift(notification);
    this.notifications = this.notifications.slice(0, MAX_NOTIFICATIONS);
    this.signal();
  }

  markAllSeen(): void {
    this.notifications.forEach((n) => (n.muted = true));
  }

  removeNotification(id: string): void {
    this.notifications = this.notifications.filter((n) => n.id !== id);
    this.signal();
  }

  private signal() {
    notificationListSignal.set(notificationListSignal.get() + 1);
  }
}

export type NotificationManager = InstanceType<typeof NotificationManagerClass>;
export const notificationManager = new NotificationManagerClass();
