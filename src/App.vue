<template>
  <keep-alive>
    <v-app id="app"
           :class="{
              'bg-transparent': shouldTransparentBody,
              'border': shouldCustomWindow,
            }"
    >
      <v-app-bar id="navbar"
                 :class="{
                    'bg-transparent': shouldTransparentBody,
                    'border': shouldCustomWindow,
                  }"
                 density="compact"
                 flat
      >
        <template v-slot:prepend>
          <v-avatar v-show="isIndexPage" class="ml-2" image="/icon-32x.png" size="32" />
          <v-btn v-show="!isIndexPage"
                 class="ml-2"
                 size="32"
                 variant="plain"
                 rounded
                 :icon="mdiArrowLeft"
                 @click="router.back"
          />
        </template>

        <v-toolbar-title class="text-monocraft mt--1 ml-3 cursor-default">
          {{ toolbarTitle }}
        </v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn :icon="windowMinimizeIcon"
               :rounded="0"
               class="no-drag"
               variant="plain"
               @click="appWindow.minimize()"
        />
        <v-btn :icon="mdiWindowClose"
               :rounded="0"
               class="no-drag mr-0"
               variant="plain"
               @click="appWindow.close()"
        />
      </v-app-bar>
      <div class="pt-12 px-8 pb-6 h-full">
        <router-view class="w-full h-full"></router-view>
      </div>
    </v-app>
  </keep-alive>

</template>

<script lang="ts" setup>
import { mdiArrowLeft, mdiWindowClose } from "@mdi/js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useTheme } from "vuetify";
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
  if (isIndexPage.value || !router.currentRoute.value.name) {
    return "KCl";
  }
  let routeName = (router.currentRoute.value.name as string).replace(/\//g, ".");
  if (!routeName.startsWith(".")) {
    routeName = `.${ routeName }`;
  }
  if (!routeName.endsWith(".")) {
    routeName += ".";
  }
  return t(`pages${ routeName }title`);
});
const windowMinimizeIcon = "M20,13H4V11H20";

function getDarkModeMediaQuery() {
  return window.matchMedia("(prefers-color-scheme: dark)");
}

function setIsDark(isDark: boolean) {
  theme.global.name.value = isDark ? "dark" : "light";
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
});
</script>

<style lang="scss">
body {
  border-radius: 8px;
}

.border {
  border: 1px solid grey;
  border-radius: 8px;
}

#navbar {
  border-bottom: none !important;
  border-top-left-radius: 8px;
  border-top-right-radius: 8px;
}

body, html, #app {
  overflow: clip !important;
  width: 100%;
  height: 100%;
  margin: 0;
}

#app {
  background: rgb(var(--v-theme-surface));
}

@font-face {
  font-family: "monocraft";
  src: url("./fonts/Monocraft-Light.otf") format("opentype");
  font-weight: 300;
  font-style: normal;
}

@font-face {
  font-family: "monocraft";
  src: url("./fonts/Monocraft-Light-Italic.otf") format("opentype");
  font-weight: 300;
  font-style: italic;
}

.text-monocraft {
  font-family: "monocraft", sans-serif;
  font-weight: 300;
}

.text-transform-none {
  text-transform: none;
}

.custom-btn {
  display: block !important;
  position: relative;
}

.custom-btn :deep(.v-btn__content) {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
</style>