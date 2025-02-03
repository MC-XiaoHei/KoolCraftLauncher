import { DownloadGroup } from "@/store/download/models.ts";
import { invoke } from "@tauri-apps/api/core";
import { v4 } from "uuid";

export const totalDownloadSpeed = ref(0);

export const useDownloadManagerStore = defineStore("install-manager-store", () => {
  const downloadGroups = ref<{ [key: string]: DownloadGroup }>({});
  const groupNameMap = ref<{
    [key: string]: {
      name: string;
      time: Date;
    }
  }>({});
  const downloading = computed(() => Object.entries(downloadGroups.value).length > 0);

  function createGroup(name: string): string {
    const randomId = v4();
    groupNameMap.value[randomId] = {
      name,
      time: new Date(),
    };
    return randomId;
  }

  onMounted(async () => {
    async function update() {
      const groups = await invoke("get_task_group_infos");
      console.log(groups);
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