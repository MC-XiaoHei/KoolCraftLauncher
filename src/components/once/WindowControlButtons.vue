<template>
  <Button
      @click="minimize"
      variant="highlight"
      size="icon"
  >
    <Minus class="text-gray-500 transition-colors" />
  </Button>

  <Button
      @click="close"
      variant="highlight"
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