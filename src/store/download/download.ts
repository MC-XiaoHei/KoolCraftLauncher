import { invoke } from "@tauri-apps/api/core";
import { v4 } from "uuid";

export const downloadSpeeds = new Map<string, number>();

export const useDownloadManagerStore = defineStore("download-manager-store", () => {
  const downloadGroups = ref<[string, string][]>([]);
  const forAdding = ref<[string, string][]>([]);
  const downloading = computed(() => downloadGroups.value.length > 0);

  function createGroup(name: string): string {
    const randomId = v4();
    forAdding.value.push([name, randomId]);
    return randomId;
  }

  onMounted(async () => {
    setInterval(async () => {
      let newDownloadGroups = [];
      for (let group of downloadGroups.value) {
        const response = await invoke("get_download_speed", {
          downloadGroup: group[1],
        });
        if (response) {
          console.log(`${group[0]}: ${(response as number / 1024 / 1024).toFixed(2)} MB/s`);
          downloadSpeeds.set(group[1], response as number);
          newDownloadGroups.push(group);
        } else {
          downloadSpeeds.delete(group[1]);
        }
      }
      for (let value of forAdding.value) {
        newDownloadGroups.push(value);
      }
      forAdding.value = [];
      downloadGroups.value = newDownloadGroups;
    }, 1000);
  });

  return {
    downloadGroups,
    downloading,
    createGroup,
  };
}, {
  persist: true,
});