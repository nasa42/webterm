import React, { useEffect, useState } from "react";
import { ELLIPSIS } from "../../../scripts/client/config.ts";
import { useStore } from "@nanostores/react";
import {
  currentTerminalStore,
  handshakeCompleteSignal,
  handshakeInitiateSignal,
} from "../../../scripts/client/stores.ts";
import { RelayHandshake } from "../../../scripts/client/models/RelayHandshake.ts";
import { Modal } from "bootstrap";
import { alertAndThrow } from "../../../scripts/client/functions/alertAndThrow.ts";
import AlertTriangleIcon from "../../../../node_modules/@tabler/icons/icons/outline/alert-triangle.svg?react";
import type { Relay } from "../../../scripts/client/models/Relay.ts";
import { pluralise } from "../../../scripts/client/functions/pluralise.ts";

const MODAL_ID = "app-handshake-modal";

export const HandshakeModal: React.FC = () => {
  const [bsModal, setBsModal] = useState<Modal | null>(null);
  const [showLoading, setShowLoading] = useState(true);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const initiateSignal = useStore(handshakeInitiateSignal);
  const [currentInput, setCurrentInput] = useState<{ deviceName: string } | null>(null);
  const [receivedNonce, setReceivedNonce] = useState<string | null>(null);
  const [receivedRelay, setReceivedRelay] = useState<Relay | null>(null);
  const [subnames, setSubnames] = useState<string[]>([]);

  useEffect(() => {
    const $modal = document.getElementById(MODAL_ID);
    if (!$modal) {
      alertAndThrow("Handshake modal not found");
      return;
    }
    const bsModal = new Modal($modal);
    setBsModal(bsModal);
  }, []);

  useEffect(() => {
    if (!initiateSignal || !bsModal) {
      return;
    }
    bsModal.show();
    handshakeInitiateSignal.set(null);
    setCurrentInput(initiateSignal);
  }, [initiateSignal, bsModal]);

  useEffect(() => {
    (async () => {
      await initiate();
    })();
  }, [currentInput]);

  const initiate = async () => {
    if (!currentInput) {
      return;
    }
    setErrorMessage(null);
    setShowLoading(true);
    try {
      const record = await RelayHandshake.new();
      const { nonce, relay, subnames } = await record.initiateConnectionRequest(currentInput?.deviceName);
      setShowLoading(false);
      setSubnames(subnames);
      setReceivedNonce(nonce);
      setReceivedRelay(relay);
    } catch (error) {
      if (error instanceof Error) {
        setErrorMessage(error.message);
      } else {
        setErrorMessage("An unknown error occurred.");
      }
      setShowLoading(false);
    }
  };

  const onRetryClick = async () => {
    await initiate();
  };

  const onSubnameSelect = async (subname: string) => {
    bsModal?.hide();
    handshakeCompleteSignal.set({
      deviceSubname: subname,
      relay: receivedRelay!,
      nonce: receivedNonce!,
    });
    currentTerminalStore.set({
      deviceSubname: subname,
      deviceName: currentInput?.deviceName || "",
    });
  };

  return (
    <div id={MODAL_ID} className="modal fade" data-bs-backdrop="static" tabIndex={-1} aria-hidden="true">
      <div className="modal-dialog modal-dialog-centered">
        <div className="modal-content">
          <div className="modal-header">
            <h5 className="modal-title">Handshake</h5>
          </div>

          <div className="modal-body">
            {showLoading && (
              <div className="text-center">
                <div className="spinner-border m-2" role="status">
                  <span className="visually-hidden">Loading...</span>
                </div>
                <p>Initiating Handshake with Relay{ELLIPSIS}</p>
              </div>
            )}

            {errorMessage && (
              <div className="text-center">
                <AlertTriangleIcon className="text-danger m-2" />
                <p>{errorMessage}</p>
                <button type="button" className="btn btn-link" onClick={onRetryClick}>
                  Retry
                </button>
              </div>
            )}

            {receivedNonce && (
              <>
                <p>
                  Found {pluralise("device", subnames.length)} with name <strong>{currentInput?.deviceName}</strong>
                </p>

                <table className="table table-striped">
                  <tbody>
                    {subnames.map((subname) => (
                      <tr key={subname}>
                        <td>
                          <a href="#" onClick={() => onSubnameSelect(subname)}>
                            {subname}
                          </a>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
