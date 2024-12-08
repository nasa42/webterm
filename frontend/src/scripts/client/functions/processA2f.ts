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
import { readA2fEncryptedRoot } from "./readA2fEncryptedRoot.ts";
import { readTerminalOutput } from "./readTerminalOutput.ts";
import { processTerminalOutput } from "./processTerminalOutput.ts";
import { ActivityId } from "../types/BigIntLike.ts";

export const processA2f = (agentRoot: A2fRoot, send: SendPayload) => {
  switch (agentRoot.format()) {
    case A2fMessageFormat.Plain:
      processPlain(agentRoot, send);
      return;
    default:
      processEncrypted(agentRoot, send);
      return;
  }
};

const processPlain = (payload: A2fRoot, send: SendPayload) => {
  switch (payload.plainMessageType()) {
    case A2fPlainMessage.AuthPreamble:
      let preamble = payload.plainMessage(new A2fPlainAuthPreamble()) as A2fPlainAuthPreamble | null;
      if (!preamble) return;
      console.info("received preamble");
      let preambleResp = F2aBuilder.new();
      send.toAgentPlain = preambleResp.buildAuthRequestVerification();
      return;
    case A2fPlainMessage.AuthResult:
      let result = payload.plainMessage(new A2fPlainAuthResult()) as A2fPlainAuthResult | null;
      if (!result) return;
      console.info("received auth result");
      let resp = F2aBuilder.new();
      send.toAgentEncrypted = resp.buildActivityCreateTerminal();
      return;
    default:
      console.error("Unknown agent plain message type:", payload.plainMessageType());
      return;
  }
};

const processEncrypted = (agentRoot: A2fRoot, send: SendPayload) => {
  // TODO: DECRYPT MESSAGE HERE
  const decrypted = agentRoot.encryptedPayloadArray();

  if (!decrypted) {
    console.error("No decrypted payload");
    return;
  }

  let message = readA2fEncryptedRoot(decrypted);

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
