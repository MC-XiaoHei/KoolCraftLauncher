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
        <v-btn :disabled="isIndexPage"
               class="ml-2 no-drag"
               size="32"
               variant="plain"
               rounded
               :icon="mdiArrowLeft"
               @click="router.back()"
        />
      </template>

      <transition :name="routeBack ? 'transform-back':'transform-forward'" mode="out-in">
        <v-toolbar-title key="brand-title" v-if="isIndexPage || !router.currentRoute.value.name"
                         class="text-monocraft"
        >KCl
        </v-toolbar-title>
        <v-toolbar-title v-else :key="toolbarTitle" class="ml-3 cursor-default title">
          {{ toolbarTitle }}
        </v-toolbar-title>
      </transition>
      <v-spacer></v-spacer>
      <v-btn class="no-drag mr-2 w-8 h-8"
             size="32"
             icon=""
             variant="plain"
             rounded
      >
        <v-icon :icon="downloadingIcons[downloadingIconFrame]" size="24" />
      </v-btn>
      <v-btn :icon="windowMinimizeIcon"
             class="no-drag mr-2"
             size="32"
             variant="plain"
             rounded
             @click="appWindow.minimize()"
      />
      <v-btn :icon="mdiWindowClose"
             class="no-drag mr-2"
             size="32"
             variant="plain"
             rounded
             @click="appWindow.close()"
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

        <router-view v-slot="{ Component }">
          <transition :name="routeBack ? 'transform-back':'transform-forward'" mode="out-in">
            <component :is="Component" class="w-full h-full" />
          </transition>
        </router-view>
      </OverlayScrollbarsComponent>
    </div>
  </v-app>
</template>

<script lang="ts" setup>
import { useMinecraftVersionCache } from "@/store/cache/minecraft-version-cache";
import { useDownloadManagerStore } from "@/store/download/download.ts";
import { DarkMode } from "@/store/theme/models";
import { useThemeStore } from "@/store/theme/theme";
import { mdiArrowLeft, mdiWindowClose } from "@mdi/js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { OverlayScrollbarsComponent } from "overlayscrollbars-vue";
import { useTheme } from "vuetify";

const { t } = useI18n();
const appWindow = getCurrentWindow();
const theme = useTheme();
const themeSettings = useThemeStore();
const downloadManagerStore = useDownloadManagerStore();
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
    translated = translated.replace(RegExp(`:${key}:`, "g"), value);
  });
  return translated;
});
const windowMinimizeIcon = "M20,13H4V11H20";
const notDownloadingKeyFrameIndex = 10;
const downloadingIconFrame = ref(notDownloadingKeyFrameIndex);
const downloadingIcons = [
  "M5.671 4.257A9.95 9.95 0 0 1 11 2.049v2.013A7.96 7.96 0 0 0 7.094 5.68zM13 4.062a7.96 7.96 0 0 1 3.906 1.618l1.423-1.423A9.95 9.95 0 0 0 13 2.05zM4.062 11A7.96 7.96 0 0 1 5.68 7.094L4.257 5.671A9.95 9.95 0 0 0 2.05 11zM18.32 7.094A7.96 7.96 0 0 1 19.938 11h2.013a9.95 9.95 0 0 0-2.208-5.329zM7.5 12l4.5 4.5 4.5-4.5H13V8h-2v4zm-1.82 4.906A7.96 7.96 0 0 1 4.062 13H2.049a9.95 9.95 0 0 0 2.208 5.329zM19.938 13a7.96 7.96 0 0 1-1.618 3.906l1.423 1.423A9.95 9.95 0 0 0 21.951 13zm-3.032 5.32A7.96 7.96 0 0 1 13 19.938v2.013a9.96 9.96 0 0 0 5.329-2.208zM11 19.938a7.96 7.96 0 0 1-3.906-1.618l-1.423 1.423A9.95 9.95 0 0 0 11 21.951z",
  "M5.671 4.257A9.95 9.95 0 0 1 11 2.049v2.013A7.96 7.96 0 0 0 7.094 5.68zM13 4.062a7.96 7.96 0 0 1 3.906 1.618l1.423-1.423A9.95 9.95 0 0 0 13 2.05zM4.062 11A7.96 7.96 0 0 1 5.68 7.094L4.257 5.671A9.95 9.95 0 0 0 2.05 11zM18.32 7.094A7.96 7.96 0 0 1 19.938 11h2.013a9.95 9.95 0 0 0-2.208-5.329zM7.5 12l4.5 4.5 4.5-4.5H13V8h-2v4zm3.5 7.938A8.004 8.004 0 0 1 4.062 13H2.049A10 10 0 0 0 11 21.95zM19.938 13a7.96 7.96 0 0 1-1.618 3.906l1.423 1.423A9.95 9.95 0 0 0 21.951 13zm-3.032 5.32A7.96 7.96 0 0 1 13 19.938v2.013a9.96 9.96 0 0 0 5.329-2.208z",
  "M5.671 4.257A9.95 9.95 0 0 1 11 2.049v2.013A7.96 7.96 0 0 0 7.094 5.68zM13 4.062a7.96 7.96 0 0 1 3.906 1.618l1.423-1.423A9.95 9.95 0 0 0 13 2.05zM4 12c0-1.849.627-3.551 1.68-4.906L4.257 5.671A9.96 9.96 0 0 0 2 12c0 5.185 3.947 9.449 9 9.95v-2.012A8 8 0 0 1 4 12m14.32-4.906A7.96 7.96 0 0 1 19.938 11h2.013a9.95 9.95 0 0 0-2.208-5.329zM7.5 12l4.5 4.5 4.5-4.5H13V8h-2v4zm12.438 1a7.96 7.96 0 0 1-1.618 3.906l1.423 1.423A9.95 9.95 0 0 0 21.95 13zm-3.032 5.32A7.96 7.96 0 0 1 13 19.938v2.013a9.95 9.95 0 0 0 5.329-2.208z",
  "M2 12c0-5.185 3.947-9.449 9-9.95v2.012a8.001 8.001 0 0 0 0 15.876v2.013C5.947 21.449 2 17.185 2 12m11-7.938a7.96 7.96 0 0 1 3.906 1.618l1.423-1.423A9.95 9.95 0 0 0 13 2.05zm5.32 3.032A7.96 7.96 0 0 1 19.938 11h2.013a9.95 9.95 0 0 0-2.208-5.329zM7.5 12l4.5 4.5 4.5-4.5H13V8h-2v4zm12.438 1a7.96 7.96 0 0 1-1.618 3.906l1.423 1.423A9.95 9.95 0 0 0 21.95 13zm-3.032 5.32A7.96 7.96 0 0 1 13 19.938v2.013a9.95 9.95 0 0 0 5.329-2.208z",
  "M2 12c0 5.185 3.947 9.449 9 9.95v-2.012A8.001 8.001 0 1 1 16.906 5.68l1.423-1.423A9.96 9.96 0 0 0 12 2C6.477 2 2 6.477 2 12m19.95-1h-2.012a7.96 7.96 0 0 0-1.618-3.906l1.423-1.423A9.95 9.95 0 0 1 21.95 11M7.5 12l4.5 4.5 4.5-4.5H13V8h-2v4zm12.243 6.329-1.423-1.423A7.96 7.96 0 0 0 19.938 13h2.013a9.95 9.95 0 0 1-2.208 5.329m-1.414 1.414A9.95 9.95 0 0 1 13 21.95v-2.013a7.96 7.96 0 0 0 3.906-1.618z",
  "M2 12c0 5.185 3.947 9.449 9 9.95v-2.012A8.001 8.001 0 0 1 12 4a8 8 0 0 1 7.938 7h2.013C21.449 5.947 17.185 2 12 2 6.477 2 2 6.477 2 12m5.5 0 4.5 4.5 4.5-4.5H13V8h-2v4zm12.243 6.329-1.423-1.423A7.96 7.96 0 0 0 19.938 13h2.013a9.95 9.95 0 0 1-2.208 5.329m-1.414 1.414A9.95 9.95 0 0 1 13 21.95v-2.013a7.96 7.96 0 0 0 3.906-1.618z",
  "M2 12c0 5.185 3.947 9.449 9 9.95v-2.012A8.001 8.001 0 0 1 12 4a8 8 0 0 1 6.32 12.906l1.423 1.423A9.96 9.96 0 0 0 22 12c0-5.523-4.477-10-10-10S2 6.477 2 12m5.5 0 4.5 4.5 4.5-4.5H13V8h-2v4zm10.829 7.743A9.95 9.95 0 0 1 13 21.95v-2.013a7.96 7.96 0 0 0 3.906-1.618z",
  "M12 16.5 7.5 12H11V8h2v4h3.5zm-1 3.438v2.013a9.94 9.94 0 0 1-5.329-2.208l1.423-1.423a8 8 0 0 0 1.713 1.017 8 8 0 0 0 2.193.601M2 12a9.95 9.95 0 0 0 2.257 6.329l1.423-1.423A7.96 7.96 0 0 1 4.062 13a8 8 0 0 1 0-2A7.96 7.96 0 0 1 5.68 7.094 8 8 0 0 1 7.094 5.68 7.96 7.96 0 0 1 11 4.062a8 8 0 0 1 2 0 7.96 7.96 0 0 1 3.906 1.618 8.05 8.05 0 0 1 2.422 3.105C19.76 9.769 20 10.856 20 12a7.99 7.99 0 0 1-3.094 6.32A7.96 7.96 0 0 1 13 19.938v2.013q.237-.024.47-.058a9.93 9.93 0 0 0 4.859-2.15q.777-.636 1.414-1.414A9.97 9.97 0 0 0 21.951 11l-.003-.022a9.94 9.94 0 0 0-2.205-5.307 10 10 0 0 0-2.792-2.361A9.95 9.95 0 0 0 12 2C6.477 2 2 6.477 2 12",
  "M12 16.5 7.5 12H11V8h2v4h3.5zm-6.91-.466q.267.456.59.872l-1.423 1.423A9.95 9.95 0 0 1 2.049 13h2.013a7.95 7.95 0 0 0 1.028 3.034M11 19.938v2.013a9.94 9.94 0 0 1-5.329-2.208l1.423-1.423a8 8 0 0 0 1.393.87c.775.38 1.621.637 2.513.748M12 2C6.815 2 2.55 5.947 2.05 11h2.012A7.96 7.96 0 0 1 5.68 7.094 8 8 0 0 1 7.094 5.68 7.96 7.96 0 0 1 11 4.062a8 8 0 0 1 2 0 7.95 7.95 0 0 1 3.906 1.618 8.05 8.05 0 0 1 2.312 2.866A8.001 8.001 0 0 1 13 19.938v2.013q.237-.024.47-.058a9.94 9.94 0 0 0 4.859-2.15q.777-.636 1.414-1.414A10 10 0 0 0 21.95 13a10 10 0 0 0 0-2l-.003-.022a9.94 9.94 0 0 0-2.205-5.307 10 10 0 0 0-1.414-1.414A9.96 9.96 0 0 0 11.999 2",
  "M2.05 11a9.95 9.95 0 0 1 2.207-5.329L5.68 7.094A7.96 7.96 0 0 0 4.062 11zM12 16.5 7.5 12H11V8h2v4h3.5zm-6.91-.466q.267.456.59.872l-1.423 1.423A9.95 9.95 0 0 1 2.049 13h2.013a7.95 7.95 0 0 0 1.028 3.034M11 19.938v2.013a9.94 9.94 0 0 1-5.329-2.208l1.423-1.423a8 8 0 0 0 1.393.87c.775.38 1.621.637 2.513.748M12 2A9.96 9.96 0 0 0 5.67 4.257L7.094 5.68A7.96 7.96 0 0 1 11 4.062a8 8 0 0 1 2 0 7.95 7.95 0 0 1 3.906 1.618 8.05 8.05 0 0 1 2.312 2.866A8.001 8.001 0 0 1 13 19.938v2.013q.237-.024.47-.058a9.94 9.94 0 0 0 4.859-2.15q.777-.636 1.414-1.414A10 10 0 0 0 21.95 13a10 10 0 0 0 0-2l-.003-.022a9.94 9.94 0 0 0-2.205-5.307 10 10 0 0 0-1.414-1.414A9.96 9.96 0 0 0 11.999 2",
  "M5.671 4.257A9.95 9.95 0 0 1 11 2.049v2.013A7.96 7.96 0 0 0 7.094 5.68zm.01 2.837A7.96 7.96 0 0 0 4.06 11H2.05a9.95 9.95 0 0 1 2.208-5.329zM12 16.5 7.5 12H11V8h2v4h3.5zM4.062 13H2.049a9.95 9.95 0 0 0 2.208 5.329l1.423-1.423A7.946 7.946 0 0 1 4.062 13m3.032 5.32-1.423 1.423A9.935 9.935 0 0 0 11 21.951v-2.013a7.94 7.94 0 0 1-3.906-1.618M20 12a8 8 0 0 1-7 7.938v2.013q.237-.024.47-.058a9.94 9.94 0 0 0 4.859-2.15q.777-.636 1.414-1.414A10 10 0 0 0 21.95 13a10 10 0 0 0-.053-2.432 9.94 9.94 0 0 0-2.155-4.897 10 10 0 0 0-1.414-1.414 9.95 9.95 0 0 0-5.33-2.208v2.013a7.95 7.95 0 0 1 3.907 1.618 8.05 8.05 0 0 1 2.302 2.845C19.716 9.576 20 10.755 20 12",
  "M16.034 5.09A7.95 7.95 0 0 0 13 4.062V2.049c2.011.2 3.847.996 5.329 2.208L16.906 5.68a8 8 0 0 0-.872-.59M5.67 4.257A9.95 9.95 0 0 1 11 2.049v2.013A7.96 7.96 0 0 0 7.094 5.68zm.01 2.837A7.96 7.96 0 0 0 4.06 11H2.05a9.95 9.95 0 0 1 2.208-5.329zM12 16.5 7.5 12H11V8h2v4h3.5zM4.062 13H2.049a9.95 9.95 0 0 0 2.208 5.329l1.423-1.423A7.947 7.947 0 0 1 4.062 13m3.032 5.32-1.423 1.423A9.933 9.933 0 0 0 11 21.951v-2.013a7.94 7.94 0 0 1-3.906-1.618M20 12a8 8 0 0 1-7 7.938v2.013q.237-.024.47-.058a9.94 9.94 0 0 0 4.859-2.15q.777-.636 1.414-1.414A10 10 0 0 0 21.95 13a10 10 0 0 0-.038-2.328 9.94 9.94 0 0 0-2.17-5L18.32 7.093a8 8 0 0 1 .873 1.401A8 8 0 0 1 20 12",
  "M16.034 5.09A7.95 7.95 0 0 0 13 4.062V2.049c2.011.2 3.847.996 5.329 2.208L16.906 5.68a8 8 0 0 0-.872-.59M5.67 4.257A9.95 9.95 0 0 1 11 2.049v2.013A7.96 7.96 0 0 0 7.094 5.68zm.01 2.837A7.96 7.96 0 0 0 4.06 11H2.05a9.95 9.95 0 0 1 2.208-5.329zm14.062-1.423L18.32 7.094a8 8 0 0 1 .884 1.424c.372.767.624 1.602.734 2.482h2.013a9.94 9.94 0 0 0-2.208-5.329M12 16.5 7.5 12H11V8h2v4h3.5zM4.062 13H2.049a9.95 9.95 0 0 0 2.208 5.329l1.423-1.423A7.947 7.947 0 0 1 4.062 13m3.032 5.32-1.423 1.423A9.933 9.933 0 0 0 11 21.951v-2.013a7.94 7.94 0 0 1-3.906-1.618M21.951 13h-2.013A8.004 8.004 0 0 1 13 19.938v2.013a9.9 9.9 0 0 0 4.12-1.36 10.065 10.065 0 0 0 2.624-2.263h-.001A10 10 0 0 0 21.95 13",
];
const routeBack = ref(false);

function getDarkModeMediaQuery() {
  return window.matchMedia("(prefers-color-scheme: dark)");
}

function setIsDark(isDark: boolean) {
  theme.global.name.value = isDark ? "defaultDarkTheme" : "defaultDarkTheme"; // TODO
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

router.beforeEach((to, from, next) => {
  if (to.name === "/[...path]") {
    routeBack.value = true;
  } else if (from.name === "/[...path]") {
    routeBack.value = false;
  } else {
    const fromPath = from.name?.toString().replace(/\./g, "/") ?? "/";
    const toPath = to.name?.toString().replace(/\./g, "/") ?? "/";
    routeBack.value = fromPath.split("/").length <= toPath.split("/").length;
  }
  next();
});

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

onMounted(async () => {
  document.getElementById("navbar")?.addEventListener("mousedown", (event) => {
    const isLeftClick = event.buttons === 1;
    const isDraggable = event.target instanceof HTMLElement && event.target.closest(".no-drag") === null;
    if (isDraggable && isLeftClick) {
      appWindow.startDragging();
    }
  });
  useMinecraftVersionCache().updateData().then();
  setInterval(() => {
    if (!downloadManagerStore.downloading && downloadingIconFrame.value === notDownloadingKeyFrameIndex) {
      return;
    }
    downloadingIconFrame.value = (downloadingIconFrame.value + 1) % downloadingIcons.length;
  }, 500);
});
</script>

<style scoped lang="scss">
.title :deep(.v-toolbar-title__placeholder) {
  width: 256px;
}

.transform-back-enter-active, .transform-back-leave-active {
  transition: transform 100ms ease, opacity 100ms ease;
}

.transform-back-enter, .transform-back-leave-to {
  transform: translateX(-5%);
  opacity: 0;
}

.transform-forward-enter-active, .transform-forward-leave-active {
  transition: transform 100ms ease, opacity 100ms ease;
}

.transform-forward-enter, .transform-forward-leave-to {
  transform: translateX(5%);
  opacity: 0;
}
</style>