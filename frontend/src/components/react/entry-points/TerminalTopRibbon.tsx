import React, { useEffect } from "react";
import { RibbonActiveNotification } from "../RibbonActiveNotification";
import { RibbonNotificationBell } from "../RibbonNotificationBell";
import { useStore } from "@nanostores/react";
import { currentTerminalStore } from "../../../scripts/client/stores.ts";

export const TerminalTopRibbon: React.FC = () => {
  const currentTerminal = useStore(currentTerminalStore);

  useEffect(() => {
    if (!currentTerminal) return;
    document.title = `${currentTerminal.deviceName}/${currentTerminal.deviceSubname} | Webterm`;
  }, [currentTerminal]);

  return (
    <>
      <div className="row gx-2">
        <nav className="navbar navbar-expand-lg bg-body-tertiary py-0">
          <div className="container-fluid">
            <a className="navbar-brand" href="/">
              Webterm
            </a>
            <RibbonActiveNotification />
            <button
              className="navbar-toggler"
              type="button"
              data-bs-toggle="collapse"
              data-bs-target="#app-navbar-dropdown"
              aria-controls="app-navbar-dropdown"
              aria-expanded="false"
              aria-label="Toggle navigation"
            >
              <span className="navbar-toggler-icon"></span>
            </button>
            <div className="collapse navbar-collapse justify-content-end" id="app-navbar-dropdown">
              <ul className="navbar-nav">
                <li className="nav-item me-2">
                  <RibbonNotificationBell />
                </li>
                <li className="nav-item dropdown">
                  <a
                    className="nav-link dropdown-toggle"
                    href="#"
                    id="app-navbar-menu-link"
                    role="button"
                    data-bs-toggle="dropdown"
                    aria-expanded="false"
                  >
                    Menu
                  </a>
                  <ul className="dropdown-menu dropdown-menu-end" aria-labelledby="app-navbar-menu-link">
                    <li>
                      <a className="dropdown-header text-decoration-none">
                        Connected to: <br />
                        {currentTerminal?.deviceName}/
                        <strong className="text-black">{currentTerminal?.deviceSubname}</strong>
                      </a>
                    </li>
                  </ul>
                </li>
              </ul>
            </div>
          </div>
        </nav>
      </div>
    </>
  );
};
