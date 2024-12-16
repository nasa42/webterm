class SessionStoreClass {
  pushToList(key: string, value: string): number {
    const currentIndex = parseInt(sessionStorage.getItem(`wt.${key}.Index`) || "0");
    sessionStorage.setItem(`wt.${key}.${currentIndex}`, value);
    sessionStorage.setItem(`wt.${key}.Index`, `${currentIndex + 1}`);
    return currentIndex;
  }

  getFromList(key: string, index: number): string | null {
    return sessionStorage.getItem(`wt.${key}.${index}`);
  }

  clear() {
    for (let i = 0; i < sessionStorage.length; i++) {
      const key = sessionStorage.key(i);
      if (key?.startsWith('wt.')) {
        sessionStorage.removeItem(key);
        i--;
      }
    }
  }
}

export type SessionStore = InstanceType<typeof SessionStoreClass>;
export const sessionStore = new SessionStoreClass();
