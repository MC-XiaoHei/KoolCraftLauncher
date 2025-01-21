import { fetch } from "@tauri-apps/plugin-http";

export interface VersionData {
  id: string;
  type: string;
  url: string;
  time: string;
  releaseTime: string;
}

export interface VersionListData {
  latest: {
    release: string;
    snapshot: string;
  };
  versions: VersionData[];
}

export enum VersionCacheStatus {
  Fetching,
  Ok,
  Error,
}

export const useMinecraftVersionCache = defineStore("minecraft-version-cache", () => {
  const data = ref<VersionListData | null>(null);
  const status = ref(VersionCacheStatus.Fetching);

  async function updateData() {
    try {
      if (data === null) {
        status.value = VersionCacheStatus.Fetching;
      } else {
        status.value = VersionCacheStatus.Ok;
      }
      const response = await fetch("https://piston-meta.mojang.com/mc/game/version_manifest.json", {
        method: "GET",
      });
      if (response.ok) {
        data.value = await response.json();
        status.value = VersionCacheStatus.Ok
      } else {
        status.value = VersionCacheStatus.Error;
      }
    } catch (_) {
        status.value = VersionCacheStatus.Error;
    }
  }

  return {
    data,
    status,
    updateData,
  };
}, {
  persist: true,
});