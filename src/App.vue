<template>
  <main>
    <div
        :class="{
          'custom-window': shouldCustomWindow,
        }"
        class="size-full"
    >
      <div class="h-8" data-tauri-drag-region>

      </div>
      <router-view />
    </div>
  </main>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

const appWindow = getCurrentWindow();
const router = useRouter();
const shouldTransparentBody = ref(false);
const shouldCustomWindow = ref(false);

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
});
</script>