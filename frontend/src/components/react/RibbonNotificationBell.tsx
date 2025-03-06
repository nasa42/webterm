/** @jsxImportSource @emotion/react */
import React, { useEffect, useState } from "react";
import { css } from "@emotion/react";
import BellIcon from "../../../node_modules/@tabler/icons/icons/outline/bell.svg?react";
import { useStore } from "@nanostores/react";
import { notificationListSignal } from "../../scripts/client/stores.ts";
import { type Notification, notificationManager } from "../../scripts/client/ui/NotificationManager.ts";

export const RibbonNotificationBell: React.FC = () => {
  const signal = useStore(notificationListSignal);

  const [list, setList] = useState<Notification[]>([]);

  useEffect(() => {
    setList(notificationManager.list());
  }, [signal]);

  useEffect(() => {
    const dropdownElement = document.getElementById("app-notification-dropdown");

    const handleDropdownHide = () => {
      notificationManager.markAllSeen();
    };

    dropdownElement?.addEventListener("hide.bs.dropdown", handleDropdownHide);

    return () => {
      dropdownElement?.removeEventListener("hide.bs.dropdown", handleDropdownHide);
    };
  }, []);

  return (
    <div
      css={css`
        cursor: pointer;

        &:hover {
          background-color: rgba(0, 0, 0, 0.05);
        }
      `}
    >
      <button className="btn btn-link position-relative" data-bs-toggle="dropdown">
        <BellIcon className="text-secondary" />
        <span className="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger d-none">
          {list.length}
        </span>
      </button>
      <div className="dropdown-menu dropdown-menu-end p-2" id="app-notification-dropdown" style={{ minWidth: "300px" }}>
        <div>
          {list.length > 0 ? (
            <>
              {list.map((n, index) => (
                <>
                  {index !== 0 && <hr className="my-2" />}
                  <div className={`p-2 ${n.muted ? "text-muted" : ""}`}>
                    <div className="d-flex justify-content-between">
                      <small>${new Date(n.timestamp).toLocaleTimeString()}</small>
                      <button
                        className="btn btn-sm btn-close"
                        aria-label="Delete"
                        onClick={() => notificationManager.removeNotification(n.id)}
                      ></button>
                    </div>
                    <div>${n.message}</div>
                  </div>
                </>
              ))}
            </>
          ) : (
            <div className="text-center text-muted">No notifications</div>
          )}
        </div>
      </div>
    </div>
  );
};
