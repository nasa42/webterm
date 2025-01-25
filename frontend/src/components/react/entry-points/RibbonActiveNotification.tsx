import { useStore } from "@nanostores/react";
import React from "react";
import { activeNotificationStore } from "../../../scripts/client/stores.ts";

const clearNotification = () => {
  activeNotificationStore.set(null);
};

export const RibbonActiveNotification: React.FC = () => {
  const store = useStore(activeNotificationStore);
  return (
    <>
      <div className={`${store || "d-none"} flex-grow-1 mx-3`}>
        <div className={`alert ${store && `alert-${store.variant}`} alert-dismissible mb-0 d-flex align-items-center`}>
          <span className="flex-grow-1 text-center">{store?.message}</span>
          <button type="button" className="btn-close" aria-label="Close" onClick={clearNotification}></button>
        </div>
      </div>
    </>
  );
};
