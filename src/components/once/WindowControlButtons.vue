<template>
  <Button
      class="rounded-none hover:bg-transparent [&:hover>svg]:text-gray-900 dark:[&:hover>svg]:text-gray-200"
      @click="minimize"
      variant="ghost"
      size="icon"
  >
    <Minus class="text-gray-500 transition-colors" />
  </Button>

  <Button
      class="rounded-none hover:bg-transparent [&:hover>svg]:text-gray-900 dark:[&:hover>svg]:text-gray-200"
      @click="close"
      variant="ghost"
      size="icon"
  >
    <X class="text-gray-500 transition-colors" />
  </Button>
</template>
<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-vue-next";

const appWindow = getCurrentWindow();

function minimize() {
  appWindow.minimize();
}

function close() {
  appWindow.close();
}
</script>