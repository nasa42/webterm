import { EncryptionReady, F2aBuilder, type PlainReady } from "../serialisers/F2aBuilder.ts";
import type { ActivityId } from "../types/BigIntLike.ts";

export class SendPayload {
  public toTerminal?: Uint8Array | null;
  public receivedActivityId?: ActivityId;
  public toAgentPlain?: F2aBuilder<PlainReady>;
  public toAgentEncrypted?: F2aBuilder<EncryptionReady>;
}
