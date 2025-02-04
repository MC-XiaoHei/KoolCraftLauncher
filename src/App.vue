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

      <transition :name="routingBack ? 'transform-back':'transform-forward'" mode="out-in">
        <v-toolbar-title key="brand-title" v-if="isIndexPage || !router.currentRoute.value.name"
                         class="text-monocraft"
        >KCl
        </v-toolbar-title>
        <v-toolbar-title v-else :key="toolbarTitle" class="ml-3 cursor-default title">
          {{ toolbarTitle }}
        </v-toolbar-title>
      </transition>
      <v-spacer></v-spacer>
      <v-tooltip location="bottom" open-delay="300"
                 :text="t('layout.label.install-page')">
        <template v-slot:activator="{ props }">
          <v-btn class="no-drag mr-2 w-8 h-8"
                 size="32"
                 v-bind="props"
                 icon=""
                 @click="router.push('/install')"
                 variant="plain"
                 rounded
          >
            <v-icon size="24" v-if="taskGroupInfos.length > 0">
              <svg xmlns="http://www.w3.org/2000/svg"
                   width="24"
                   height="24"
                   viewBox="0 0 24 24"
                   fill="currentColor">
                <path
                    d="M13 15.6C13.3 15.8 13.7 15.8 14 15.6L19 12.7L19 13C19.7 13 20.4 13.1 21 13.4L21 11.6L22 11C22.5 10.7 22.6 10.1 22.4 9.6L20.9 7.1C20.8 6.9 20.7 6.7 20.5 6.6L12.6 2.2C12.4 2.1 12.2 2 12 2C11.8 2 11.6 2.1 11.4 2.2L3.6 6.6C3.4 6.7 3.2 6.8 3.1 7L1.6 9.6C1.3 10.1 1.5 10.7 2 11C2.3 11.2 2.7 11.2 3 11L3 16.5C3 16.9 3.2 17.2 3.5 17.4L11.4 21.8C11.6 21.9 11.8 22 12 22C12.2 22 12.4 21.9 12.6 21.8L13.5 21.3C13.2 20.7 13.1 20 13 19.3M11 19.3L5 15.9L5 9.2L11 12.6L11 19.3ZM20.1 9.7L13.8 13.3L13.2 12.3L19.5 8.7L20.1 9.7ZM12 10.8L12 4.2L18 7.5L12 10.8Z">
                </path>
                <path class="fade-transition-inf"
                      d="M19.9747 15.5843L17.9747 15.5843L17.9747 19.5843L15.9747 19.5843L18.9747 22.5843L21.9747 19.5843L19.9747 19.5843L19.9747 15.5843Z">
                </path>
              </svg>
            </v-icon>
            <v-icon v-else :icon="mdiPackageVariant" size="24" />
          </v-btn>
        </template>
      </v-tooltip>
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
          <transition :name="routingBack ? 'transform-back':'transform-forward'" mode="out-in">
            <component :is="Component" class="w-full h-full" />
          </transition>
        </router-view>
      </OverlayScrollbarsComponent>
    </div>
  </v-app>
</template>

<script lang="ts" setup>
import { useMinecraftVersionCache } from "@/data/cache/minecraft-version-cache";
import { taskGroupInfos, updateTaskGroupInfo } from "@/data/install/install.ts";
import { DarkMode } from "@/data/theme/models";
import { useThemeStore } from "@/data/theme/theme";
import { routingBack } from "@/plugins/router.ts";
import { mdiArrowLeft, mdiPackageVariant, mdiWindowClose } from "@mdi/js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { OverlayScrollbarsComponent } from "overlayscrollbars-vue";
import { useTheme } from "vuetify";

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
    translated = translated.replace(RegExp(`:${key}:`, "g"), value);
  });
  return translated;
});
const windowMinimizeIcon = "M20,13H4V11H20";

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

  updateTaskGroupInfo().then(() => setInterval(() => updateTaskGroupInfo(), 1000));
});
</script>

<style scoped lang="scss">
.title :deep(.v-toolbar-title__placeholder) {
  width: 256px;
}

.transform-forward-enter-active, .transform-forward-leave-active {
  transition: transform 100ms ease, opacity 100ms ease;
}

.transform-forward-enter, .transform-forward-leave-to {
  transform: translateX(-5%);
  opacity: 0;
}

.transform-back-enter-active, .transform-back-leave-active {
  transition: transform 100ms ease, opacity 100ms ease;
}

.transform-back-enter, .transform-back-leave-to {
  transform: translateX(5%);
  opacity: 0;
}

@keyframes fadeInOut {
  0% {
    opacity: 0.2;
  }
  50% {
    opacity: 1;
  }
  100% {
    opacity: 0.2;
  }
}

.fade-transition-inf {
  animation: fadeInOut 1.5s infinite;
}
</style>