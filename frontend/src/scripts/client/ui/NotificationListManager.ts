export interface Notification {
  id: string;
  message: string;
  timestamp: Date;
  read: boolean;
}

class NotificationListManager {
  private notifications: Notification[] = [];
  private $badge: HTMLElement | null = null;
  private $list: HTMLElement | null = null;

  constructor(
    private readonly badgeId: string,
    private readonly listId: string,
  ) {
    document.addEventListener("DOMContentLoaded", () => this.init());
  }

  private init(): void {
    this.$badge = document.getElementById(this.badgeId);
    this.$list = document.getElementById(this.listId);

    if (!this.$badge || !this.$list) {
      console.error(`Required elements not found: badge=${this.badgeId} list=${this.listId}`);
      return;
    }

    this.render();
    this.setupEventListeners();
  }

  private setupEventListeners(): void {
    if (!this.$list) return;

    this.$list.addEventListener("click", (e) => {
      const target = e.target;
      if (!(target instanceof HTMLElement)) return;

      const item = target.closest<HTMLElement>(".app-notification-item");
      if (!item) return;

      const id = item.dataset.id;
      if (!id) return;

      if (target.classList.contains("btn-close")) {
        this.removeNotification(id);
      } else {
        this.markAsRead(id);
      }
    });
  }

  addNotification(message: string): void {
    const notification: Notification = {
      id: crypto.randomUUID(),
      message,
      timestamp: new Date(),
      read: false,
    };
    this.notifications.unshift(notification);
    this.render();
  }

  removeNotification(id: string): void {
    this.notifications = this.notifications.filter((n) => n.id !== id);
    this.render();
  }

  markAsRead(id: string): void {
    const notification = this.notifications.find((n) => n.id === id);
    if (notification) {
      notification.read = true;
      this.render();
    }
  }

  private render(): void {
    if (!this.$badge || !this.$list) return;

    const unreadCount = this.notifications.filter((n) => !n.read).length;
    this.$badge.textContent = unreadCount.toString();
    this.$badge.classList.toggle("d-none", unreadCount === 0);

    if (this.notifications.length === 0) {
      this.$list.innerHTML = '<div class="text-center text-muted">No notifications</div>';
      return;
    }

    this.$list.innerHTML = this.notifications
      .map(
        (n) => `
          <div class="app-notification-item p-2 ${n.read ? "text-muted" : ""}" data-id="${n.id}">
            <div class="d-flex justify-content-between">
              <small>${new Date(n.timestamp).toLocaleTimeString()}</small>
              <button class="btn btn-sm btn-close" aria-label="Delete"></button>
            </div>
            <div>${n.message}</div>
          </div>
        `,
      )
      .join('<hr class="my-2">');
  }
}

export const notificationList = new NotificationListManager("app-notification-badge", "app-notifications-list");
