import { beforeEach, describe, expect, it } from "vitest";
import { StoredCredential } from "./StoredCredential";
import { sessionStore } from "./SessionStore";

describe("StoredCredential", () => {
  const testServerId = "test-server";
  const testPassword = "test-password";

  beforeEach(() => {
    sessionStore.clear();
  });

  describe("store and retrieve", () => {
    it("should store and retrieve credentials correctly", async () => {
      const { index, secretKey } = await StoredCredential.store(testServerId, testPassword);

      const credential = await StoredCredential.retrieve(index, secretKey);

      expect(credential.serverId).toBe(testServerId);
      expect(credential.serverPassword).toBe(testPassword);
    });

    it("should throw error when no credentials found", async () => {
      await expect(StoredCredential.retrieve(999, "invalid-key")).rejects.toThrow(
        "No stored credentials for index 999",
      );
    });

    it("should throw error with invalid secret key", async () => {
      const { index } = await StoredCredential.store(testServerId, testPassword);

      await expect(StoredCredential.retrieve(index, "wrong-secret-key")).rejects.toThrow();
    });
  });
});
