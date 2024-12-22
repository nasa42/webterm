import { ivCounter, type IVCounter } from "./IVCounter";
import { deflateRawCompress, deflateRawDecompress } from "../functions/deflateRaw.ts";
import { Bits256Array, Bits96Array } from "../types/BitsArray.ts";

export class Cryptographer {
  private key?: CryptoKey;
  private static readonly COMPRESSION_THRESHOLD = 512;

  private constructor(private readonly ivCounter: IVCounter) {}

  static async new({
    ivCounter_ = ivCounter,
    salt,
    secretKey,
    iterations = 100_000,
  }: {
    ivCounter_?: IVCounter;
    salt?: Bits256Array;
    secretKey: string;
    iterations?: number;
  }): Promise<{ cryptographer: Cryptographer; salt: Bits256Array; iterations: number }> {
    if (!salt) {
      salt = Bits256Array.random();
    }

    const record = new Cryptographer(ivCounter_);
    await record.initKey(salt.data(), secretKey, iterations);
    return {
      salt,
      iterations,
      cryptographer: record,
    };
  }

  private async initKey(salt: Uint8Array, secretKey: string, iterations: number) {
    const encoder = new TextEncoder();
    const keyMaterial = encoder.encode(secretKey);
    const importKey = await crypto.subtle.importKey("raw", keyMaterial, "PBKDF2", false, ["deriveBits", "deriveKey"]);
    this.key = await crypto.subtle.deriveKey(
      {
        name: "PBKDF2",
        salt: salt,
        iterations,
        hash: "SHA-256",
      },
      importKey,
      { name: "AES-GCM", length: 256 },
      false,
      ["encrypt", "decrypt"],
    );
  }

  static async quickEncrypt({ secretKey, plaintext }: { secretKey: string; plaintext: Uint8Array }): Promise<{
    ciphertext: Uint8Array;
    salt: Bits256Array;
    iv: Bits96Array;
  }> {
    const { cryptographer, salt } = await Cryptographer.new({
      secretKey,
    });
    const { ciphertext, iv } = await cryptographer.encrypt(plaintext);
    return {
      ciphertext,
      salt,
      iv,
    };
  }

  static async quickDecrypt({
    secretKey,
    ciphertext,
    salt,
    iv,
  }: {
    secretKey: string;
    ciphertext: Uint8Array;
    salt: Bits256Array;
    iv: Bits96Array;
  }): Promise<string> {
    const { cryptographer } = await Cryptographer.new({
      secretKey,
      salt,
    });
    const decrypted = await cryptographer.decrypt(ciphertext, iv, false);
    return new TextDecoder().decode(decrypted);
  }

  async encrypt(
    data: Uint8Array,
    mayCompress: boolean = false,
  ): Promise<{
    ciphertext: Uint8Array;
    iv: Bits96Array;
    compressed: boolean;
  }> {
    if (!this.key) {
      throw new Error("Key not initialised");
    }

    const iv = this.ivCounter.next();
    let compressed = false;
    let payload = data;

    if (mayCompress && data.length > Cryptographer.COMPRESSION_THRESHOLD) {
      payload = await deflateRawCompress(data);
      compressed = true;
    }

    const ciphertext = new Uint8Array(
      await crypto.subtle.encrypt({ name: "AES-GCM", iv: iv.data() }, this.key, payload),
    );

    return { ciphertext, iv, compressed };
  }

  async decrypt(ciphertext: Uint8Array, iv: Bits96Array, compressed: boolean): Promise<Uint8Array> {
    if (!this.key) {
      throw new Error("Key not initialised");
    }

    let decrypted = new Uint8Array(
      await crypto.subtle.decrypt(
        {
          name: "AES-GCM",
          iv: iv.data(),
        },
        this.key,
        ciphertext,
      ),
    );

    if (compressed) {
      try {
        decrypted = await deflateRawDecompress(decrypted);
      } catch (error) {
        console.error("failed to decompress");
        throw error;
      }
    }

    return decrypted;
  }
}
