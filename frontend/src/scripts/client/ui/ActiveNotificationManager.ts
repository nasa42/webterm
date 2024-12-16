interface ActiveNotification {
  message: string;
  variant: "primary" | "secondary" | "success" | "danger" | "warning" | "info" | "light" | "dark";
}

class ActiveNotificationManager {
  private notification: ActiveNotification | null = null;
  private $container: HTMLElement | null = null;
  private $message: HTMLElement | null = null;
  private $alert: HTMLElement | null = null;

  constructor(private readonly containerId: string) {
    document.addEventListener("DOMContentLoaded", () => this.init());
  }

  private init(): void {
    this.$container = document.getElementById(this.containerId);
    if (!this.$container) {
      console.error(`Container element not found: ${this.containerId}`);
      return;
    }

    this.$message = this.$container.querySelector("#app-active-notification-message");
    this.$alert = this.$container.querySelector(".alert");

    if (!this.$message || !this.$alert) {
      console.error("Required notification elements not found");
      return;
    }

    this.$container.querySelector(".btn-close")?.addEventListener("click", () => {
      this.clear();
    });

    this.render();
  }

  show(message: string, variant: ActiveNotification["variant"]): void {
    this.notification = {
      message,
      variant,
    };
    this.render();
  }

  clear(): void {
    this.notification = null;
    this.render();
  }

  get(): ActiveNotification | null {
    return this.notification;
  }

  private render(): void {
    if (!this.$container || !this.$message || !this.$alert) return;

    if (this.notification) {
      this.$message.textContent = this.notification.message;
      this.$alert.className = `alert alert-${this.notification.variant} alert-dismissible mb-0 d-flex align-items-center`;
      this.$container.classList.remove("d-none");
    } else {
      this.$container.classList.add("d-none");
    }
  }
}

export const activeNotification = new ActiveNotificationManager("app-active-notification");
