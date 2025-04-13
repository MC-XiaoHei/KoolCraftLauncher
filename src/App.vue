<template>
  <main class="select-none">
    <div
        :class="{
          'custom-window': shouldCustomWindow,
        }"
        class="size-full"
    >
      <div class="h-9 flex" data-tauri-drag-region>
        <div class="flex items-center justify-center">
          <Button
              variant="highlight"
              size="icon"
              :disabled="isRootRoute"
              @click="routeBack"
          >
            <ChevronLeft />
          </Button>
        </div>
        <div class="flex flex-col items-start justify-center">
          <transition :name="isRoutingBack() ? 'back' : 'to'" mode="out-in">
            <div class="font-minecraft pb-0.5" data-tauri-drag-region v-if="isRootRoute" key="index">
              KCl
            </div>
            <div class="pb-0.5" data-tauri-drag-region v-else :key="title">
              {{ title }}
            </div>
          </transition>
        </div>
        <Spacer data-tauri-drag-region />
        <ThemeChangeButton />
        <WindowControlButtons />
      </div>
      <RouterView v-slot="{ Component, route }">
        <transition :name="isRoutingBack() ? 'back' : 'to'">
          <KeepAlive include="IndexPage">
            <component :is="Component" :key="route.path" />
          </KeepAlive>
        </transition>
      </RouterView>
    </div>
  </main>
</template>

<!--suppress CssUnusedSymbol -->
<style>
.to-enter-active, .to-leave-active {
  transition: transform 100ms ease, opacity 100ms ease;
}

.to-enter, .to-leave-to {
  transform: translateX(-5%);
  opacity: 0;
}

.back-enter-active, .back-leave-active {
  transition: transform 100ms ease, opacity 100ms ease;
}

.back-enter, .back-leave-to {
  transform: translateX(5%);
  opacity: 0;
}
</style>

<script setup lang="ts">
import { isRoutingBack, path, routeBack, routerRef } from "@/lib/router.ts";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ChevronLeft } from "lucide-vue-next";
import { onMounted } from "vue";
import { useI18n } from "vue-i18n";

const {t} = useI18n();
const appWindow = getCurrentWindow();
const router = useRouter();
const shouldTransparentBody = ref(false);
const shouldCustomWindow = ref(false);
const isRootRoute = computed(() => {
  return router.currentRoute.value.path === "/";
});
const title = computed(() => {
  if (path.value !== "index") {
    return t(`pages.${ path.value }.title`);
  } else {
    return "";
  }
});

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
  appWindow.show();
  routerRef.value = router;
});
</script>