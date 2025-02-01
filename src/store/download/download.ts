import { DownloadGroup, DownloadGroups } from "@/store/download/models.ts";
import { invoke } from "@tauri-apps/api/core";
import { v4 } from "uuid";

export const totalDownloadSpeed = ref(0);
const totalDownloadSpeedHistory = ref<number[]>([]);

export const useDownloadManagerStore = defineStore("download-manager-store", () => {
  const downloadGroups = ref<{ [key: string]: DownloadGroup }>({});
  const groupNameMap = ref<{ [key: string]: string }>({});
  const downloading = computed(() => Object.entries(downloadGroups.value).length > 0);

  function createGroup(name: string): string {
    const randomId = v4();
    groupNameMap.value[randomId] = name;
    return randomId;
  }

  onMounted(async () => {
    async function update() {
      const groups = await invoke("get_download_groups") as DownloadGroups;
      let newGroups: { [key: string]: DownloadGroup } = {};
      let newTotalDownloadSpeed = 0;
      for (const [id, group] of Object.entries(groups)) {
        group.name = groupNameMap.value[id] || id;
        newTotalDownloadSpeed += group.downloadSpeed;
        newGroups[id] = group;
      }
      totalDownloadSpeedHistory.value.push(newTotalDownloadSpeed);
      if (totalDownloadSpeedHistory.value.length > 5) {
        totalDownloadSpeedHistory.value.shift();
      }
      totalDownloadSpeed.value = totalDownloadSpeedHistory.value
          .reduce((acc, current) => acc + current, 0) / totalDownloadSpeedHistory.value.length;
      downloadGroups.value = newGroups;
      console.log(newGroups);
    }

    update().then(() => setInterval(update, 500));
  });

  return {
    downloadGroups,
    groupNameMap,
    downloading,
    createGroup,
  };
}, {
  persist: true,
});