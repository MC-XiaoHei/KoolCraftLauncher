import { MinecraftDir } from "@/data/game/models.ts";

export const useMinecraftDirStore = defineStore("minecraft-dir-store", () => {
  const dirs = ref<MinecraftDir[]>([{
    path: "./.minecraft",
    name: useI18n().t("defaults.current-minecraft-dir-name"),
  }]);
  const currentDir = ref<MinecraftDir>(dirs.value[0]);

  watch(() => dirs.value.length, (length) => {
    if (length === 1) {
      currentDir.value = dirs.value[0];
    }
  });

  return {
    dirs,
    currentDir,
  };
});
