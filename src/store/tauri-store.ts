import { load } from "@tauri-apps/plugin-store";

export const tauriStore = await load("kcl-data.json", { autoSave: true });
