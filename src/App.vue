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
            <ChevronLeft class="size-5 text-gray-500 transition-colors" />
          </Button>
        </div>
        <div class="flex flex-col items-start justify-center">
          <div class="font-minecraft pb-0.5" data-tauri-drag-region>
            KCl
          </div>
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
import { isRoutingBack, routeBack, routerRef } from "@/lib/router.ts";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ChevronLeft } from "lucide-vue-next";
import { onMounted } from "vue";

const appWindow = getCurrentWindow();
const router = useRouter();
const shouldTransparentBody = ref(false);
const shouldCustomWindow = ref(false);

const isRootRoute = computed(() => {
  return router.currentRoute.value.path === "/";
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