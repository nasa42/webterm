import type { SendPayload } from "../models/SendPayload.ts";
import {
  A2fActivityOutput,
  A2fMessage,
  A2fMessageFormat,
  A2fPlainAuthPreamble,
  A2fPlainAuthResult,
  A2fPlainMessage,
  A2fRoot,
} from "../../../generated/flatbuffers_schema/talk_v1/talk_v1.ts";
import { F2aBuilder } from "../serialisers/F2aBuilder.ts";
import { readA2fEncryptedRoot } from "../parsers/readA2fEncryptedRoot.ts";
import { readTerminalOutput } from "../parsers/readTerminalOutput.ts";
import { processTerminalOutput } from "./processTerminalOutput.ts";
import { ActivityId } from "../types/BigIntLike.ts";
import { Bits256Array, Bits96Array } from "../types/BitsArray.ts";
import { activeNotification } from "../ui/ActiveNotificationManager.ts";

export const processA2f = async (agentRoot: A2fRoot, send: SendPayload) => {
  switch (agentRoot.format()) {
    case A2fMessageFormat.Plain:
      await processPlain(agentRoot, send);
      return;
    default:
      await processEncrypted(agentRoot, send);
      return;
  }
};

const processPlain = async (payload: A2fRoot, send: SendPayload) => {
  switch (payload.plainMessageType()) {
    case A2fPlainMessage.AuthPreamble:
      let preamble = payload.plainMessage(new A2fPlainAuthPreamble()) as A2fPlainAuthPreamble | null;
      const salt = preamble?.salt();
      if (!preamble || !preamble.pbkdf2Iterations() || !salt) {
        alert("Invalid preamble");
        return;
      }
      console.info("received preamble");
      let preambleResp = F2aBuilder.new();
      await send.runner.initCryptographer({
        iterations: preamble.pbkdf2Iterations(),
        salt: Bits256Array.fromFbBits256(salt),
      });
      const { ciphertext, iv } = await send.runner.cryptographer().encrypt(preamble.challengeNonceArray()!, false);
      send.toAgentPlain = preambleResp.buildAuthRequestVerification(iv, ciphertext, 0n);
      return;
    case A2fPlainMessage.AuthResult:
      let result = payload.plainMessage(new A2fPlainAuthResult()) as A2fPlainAuthResult | null;
      if (!result) return;
      console.info(`received auth result: ${result.successAuth()}`);
      activeNotification.clear();
      let resp = F2aBuilder.new();
      send.toAgentEncrypted = resp.buildActivityCreateTerminal();
      return;
    default:
      console.error("Unknown agent plain message type:", payload.plainMessageType());
      return;
  }
};

const processEncrypted = async (agentRoot: A2fRoot, send: SendPayload) => {
  const ciphertext = agentRoot.encryptedPayloadArray();
  const iv = agentRoot.iv();

  if (!ciphertext || !iv) {
    // TODO: Return an error to agent
    console.error("No ciphertext or iv found in agent root");
    return;
  }

  const compressed = agentRoot.format() === A2fMessageFormat.Aes256GcmDeflateRaw;

  let plaintext = await send.runner.cryptographer().decrypt(ciphertext, Bits96Array.fromFbBits96(iv), compressed);

  if (!plaintext) {
    // TODO: Return an error to agent
    console.error("No decrypted payload");
    return;
  }

  let message = readA2fEncryptedRoot(plaintext);

  switch (message.messageType()) {
    case A2fMessage.ActivityOutput:
      const activityOutput = message.message(new A2fActivityOutput()) as A2fActivityOutput | null;
      const terminalOutput = activityOutput?.outputArray();
      if (!activityOutput || !terminalOutput) return;
      const root = readTerminalOutput(terminalOutput);
      send.receivedActivityId = new ActivityId(activityOutput.activityId());
      processTerminalOutput(root, send);
  }
};
