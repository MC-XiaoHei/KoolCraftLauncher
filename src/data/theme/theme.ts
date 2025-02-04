import { DarkMode, WindowVibrancyEffect } from "./models.ts";

export const useThemeStore = defineStore("theme", () => {
  const windowVibrancyEffect = ref(WindowVibrancyEffect.AUTO);
  const darkMode = ref(DarkMode.AUTO);

  return {
    windowVibrancyEffect,
    darkMode,
  };
}, {
  persist: true,
});