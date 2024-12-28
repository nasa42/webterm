import { Cryptographer } from "../cryptography/Cryptographer.ts";
import { ensureBinary } from "../functions/ensureBinary.ts";
import { jsonParse, jsonStringify } from "../functions/jsonUtils.ts";
import { sessionStore } from "./SessionStore.ts";
import { Bits256Array, Bits96Array } from "../types/BitsArray.ts";

export class StoredCredential {
  static async store(serverId: string, serverPassword: string): Promise<{ index: number; storeKey: string }> {
    const storeKey = crypto.randomUUID();

    const encrypted = await Cryptographer.quickEncrypt({
      secretKey: storeKey,
      plaintext: ensureBinary(jsonStringify({ serverId, serverPassword })),
    });
    const payload = jsonStringify({
      ciphertext: encrypted.ciphertext,
      salt: encrypted.salt.data(),
      iv: encrypted.iv.data(),
    });

    const index = sessionStore.pushToList("auth", payload);
    return { index, storeKey };
  }

  static async retrieve(index: number, storeKey: string): Promise<StoredCredential> {
    const payload = sessionStore.getFromList("auth", index);
    if (!payload) {
      throw new Error(`No stored credentials for index ${index}`);
    }

    const { ciphertext, salt, iv }: { ciphertext: Uint8Array; salt: Uint8Array; iv: Uint8Array } = jsonParse(payload);
    const decrypted = await Cryptographer.quickDecrypt({
      secretKey: storeKey,
      salt: new Bits256Array(salt),
      iv: new Bits96Array(iv),
      ciphertext,
    });

    const { serverId, serverPassword } = jsonParse(decrypted) as { serverId: string; serverPassword: string };

    return new StoredCredential(serverId, serverPassword);
  }

  private constructor(
    public readonly serverId: string,
    public readonly serverPassword: string,
  ) {}
}
