import { tauriStore } from "./tauri-store.ts";

// noinspection JSUnusedGlobalSymbols
class MixedStore {
  getItem(key: string): string | null {
    return localStorage.getItem(key);
  }

  setItem(key: string, value: string) {
    localStorage.setItem(key, value);
    tauriStore.set(key, value).then();
  }
}

export const mixedStore = new MixedStore();