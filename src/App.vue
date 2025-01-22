<template>
  <v-app id="app"
         :class="{
              'bg-transparent': shouldTransparentBody,
              'border': shouldCustomWindow,
            }"
  >
    <v-app-bar id="navbar"
               :class="{
                    'bg-transparent': shouldTransparentBody,
                  }"
               density="compact"
               flat
    >
      <template v-slot:prepend>
        <v-btn v-show="!isIndexPage"
               class="ml-2 no-drag"
               size="32"
               variant="plain"
               rounded
               :icon="mdiArrowLeft"
               @click="() => router.back()"
        />
      </template>

      <v-toolbar-title v-if="isIndexPage || !router.currentRoute.value.name"
                       class="text-monocraft"
      >KCl
      </v-toolbar-title>
      <v-toolbar-title v-else class="mt--1 ml-3 cursor-default title">
        {{ toolbarTitle }}
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn :icon="windowMinimizeIcon"
             class="no-drag mr-2"
             size="32"
             variant="plain"
             rounded
             @click="() => appWindow.minimize()"
      />
      <v-btn :icon="mdiWindowClose"
             class="no-drag mr-2"
             size="32"
             variant="plain"
             rounded
             @click="() => appWindow.close()"
      />
    </v-app-bar>
    <div class="pt-12 pb-2 w-full h-full overflow-hidden">
      <OverlayScrollbarsComponent class="w-full h-full px-8" defer :options="{
          scrollbars: {
            clickScroll: true,
            dragScroll: true,
          },
          overflow: {
            x: 'hidden',
          },
        }">
        <router-view class="w-full h-full" />
      </OverlayScrollbarsComponent>
    </div>
  </v-app>
</template>

<script lang="ts" setup>
import { mdiArrowLeft, mdiWindowClose } from "@mdi/js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { OverlayScrollbarsComponent } from "overlayscrollbars-vue";
import { useTheme } from "vuetify";
import { useMinecraftVersionCache } from "./store/cache/minecraft-version-cache.ts";
import { DarkMode } from "./store/theme/models.ts";
import { useThemeStore } from "./store/theme/theme.ts";

const { t } = useI18n();
const appWindow = getCurrentWindow();
const theme = useTheme();
const themeSettings = useThemeStore();
const router = useRouter();
const shouldTransparentBody = ref(false);
const shouldCustomWindow = ref(false);
const isIndexPage = computed(() => router.currentRoute.value.name === "/");
const toolbarTitle = computed(() => {
  const route = router.currentRoute.value;
  let routeName = (route.name as string).replace(/\//g, ".");
  if (!routeName.startsWith(".")) {
    routeName = `.${ routeName }`;
  }
  if (routeName.includes("[") && routeName.includes("]")) {
    routeName = routeName.replace(/\[.*]/g, "dynamic");
  }
  if (!routeName.endsWith(".")) {
    routeName += ".";
  }
  let translated = t(`pages${ routeName }title`);
  Object.entries(route.params).forEach(([key, value]) => {
    translated = translated.replace(`<${ key }>`, value);
  });
  return translated;
});
const windowMinimizeIcon = "M20,13H4V11H20";

function getDarkModeMediaQuery() {
  return window.matchMedia("(prefers-color-scheme: dark)");
}

function setIsDark(isDark: boolean) {
  theme.global.name.value = isDark ? "defaultDarkTheme" : "defaultLightTheme";
}

function detectDarkMode() {
  switch (themeSettings.darkMode) {
    case DarkMode.AUTO:
      setIsDark(getDarkModeMediaQuery().matches);
      break;
    case DarkMode.DARK:
      setIsDark(true);
      break;
    case DarkMode.LIGHT:
      setIsDark(false);
      break;
  }
}

getDarkModeMediaQuery().addEventListener("change", detectDarkMode);
detectDarkMode();

invoke("get_vibrancy_state").then((vibrancyState) => {
  if (vibrancyState !== "none") {
    shouldTransparentBody.value = true;
  }
});

invoke("should_custom_window").then((shouldCustom) => {
  if (shouldCustom) {
    shouldCustomWindow.value = true;
  }
});

onMounted(() => {
  document.getElementById("navbar")?.addEventListener("mousedown", (event) => {
    const isLeftClick = event.buttons === 1;
    const isDraggable = event.target instanceof HTMLElement && event.target.closest(".no-drag") === null;
    if (isDraggable && isLeftClick) {
      appWindow.startDragging();
    }
  });
  useMinecraftVersionCache().updateData().then();
});
</script>

<style scoped lang="scss">
.title :deep(.v-toolbar-title__placeholder) {
  width: 256px;
}
</style>