import { invoke } from "@tauri-apps/api/core";
import { v4 } from "uuid";

export const useDownloadManagerStore = defineStore("download-manager-store", () => {
  const downloadGroups = ref<[string, string][]>([]);

  function createGroup(name: string): string {
    const randomId = v4();
    downloadGroups.value.push([name, randomId]);
    return randomId;
  }


  onMounted(async () => {
    const result: [string, string][] = [];
    for (let value of downloadGroups.value) {
      if (await invoke("is_download_group_exists", {
        downloadGroup: value[1],
      })) {
        result.push(value);
      }
    }
    downloadGroups.value = result;

    setInterval(() => {
      const forRemoval: string[] = [];
      downloadGroups.value.forEach((group) => {
        invoke("get_download_speed", {
          downloadGroup: group[1],
        }).then((speed) => {
          if (!speed) {
            forRemoval.push(group[1]);
          } else {
            console.log(`Download speed(${ group[0] }): ${ (speed as number / 1024 / 1024).toFixed(2) } MB/s`);
          }
        });
      });
      downloadGroups.value = downloadGroups.value.filter((group) => !forRemoval.includes(group[1]));
    }, 1000);
  });

  return {
    downloadGroups,
    createGroup,
  };
}, {
  persist: true,
});