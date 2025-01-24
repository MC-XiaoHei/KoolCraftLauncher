import { VersionCacheStatus, VersionData, VersionListData } from "@/store/cache/models.ts";
import { Game, GameLoader, GameType, MinecraftDir } from "@/store/game/models.ts";
import { isFoolsDay } from "@/utils/date-utils.ts";
import { fetch } from "@tauri-apps/plugin-http";

const unexpectedFoolsDayVersions = [
  "1.RV-Pre1",
];

const unlistedFoolsDayVersions: VersionData[] = [
  {
    id: "2.0 - Blue",
    type: "snapshot",
    url: "https://archive.org/download/2point-0-blue/2point0_blue.json",
    time: "2013-04-01",
    releaseTime: "2013-04-01T11:45:14+00:00",
  },
  {
    id: "2.0 - Red",
    type: "snapshot",
    url: "https://archive.org/download/2point-0-red/2point0_red.json",
    time: "2013-04-01",
    releaseTime: "2013-04-01T11:45:14+00:00",
  },
  {
    id: "2.0 - Purple",
    type: "snapshot",
    url: "https://archive.org/download/2point-0-purple/2point0_purple.json",
    time: "2013-04-01",
    releaseTime: "2013-04-01T11:45:14+00:00",
  },
];

export const useMinecraftVersionCache = defineStore("minecraft-version-cache", () => {
  const data = ref<VersionListData | null>(null);
  const foolsDayVersions: Ref<VersionData[]> = computed(() => {
    if (data.value === null) return [];
    let versions = data.value.versions
        .filter((version) => version.type === "snapshot")
        .filter((version) =>
            unexpectedFoolsDayVersions.includes(version.id) || isFoolsDay(version.releaseTime),
        );
    unlistedFoolsDayVersions.forEach((version) => {
      versions.push(version);
    });
    return versions;
  });
  const status = ref(VersionCacheStatus.Fetching);

  async function updateData() {
    try {
      if (data === null || status.value === VersionCacheStatus.Error) {
        status.value = VersionCacheStatus.Fetching;
      } else {
        status.value = VersionCacheStatus.Ok;
      }
      const response = await fetch("https://piston-meta.mojang.com/mc/game/version_manifest.json", {
        method: "GET",
      });
      if (response.ok) {
        data.value = await response.json();
        status.value = VersionCacheStatus.Ok;
      } else {
        status.value = VersionCacheStatus.Error;
      }
    } catch (_) {
      status.value = VersionCacheStatus.Error;
    }
  }

  function buildGame(id: string, dir: MinecraftDir): Game {
    let type = GameType.Release;
    let loader = GameLoader.Vanilla;
    if (!/\d+\.\d+\.\d/.test(id)) {
      type = GameType.Snapshot;
    }
    if (foolsDayVersions.value.some((version) => version.id === id)) {
      type = GameType.FoolsDay;
    }
    // TODO: detect old versions
    // TODO: detect loader
    return {
      id,
      type,
      loader,
      dir,
    };
  }

  function getVersionInfo(id: string): VersionData | null {
    if (data.value === null) return null;
    return data.value.versions.find((version) => version.id === id) || null;
  }

  return {
    data,
    foolsDayVersions,
    status,
    updateData,
    buildGame,
    getVersionInfo,
  };
}, {
  persist: true,
});