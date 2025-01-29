import { invoke } from "@tauri-apps/api/core";
import { v4 } from "uuid";

export const useDownloadManagerStore = defineStore("download-manager-store", () => {
  const downloadGroups = ref<[string, string][]>([]);

  function createGroup(name: string): string {
    const randomId = v4();
    downloadGroups.value.push([name, randomId]);
    return randomId;
  }

  async function refresh() {
    const result: [string, string][] = [];

    for (let value of downloadGroups.value) {
      if (await invoke("is_download_group_exists", {
        downloadGroup: value[1],
      })) {
        result.push(value);
      }
    }

    downloadGroups.value = result;
  }

  return {
    downloadGroups,
    createGroup,
    refresh,
  };
}, {
  persist: true,
});