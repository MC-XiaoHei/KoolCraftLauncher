<template>
  <Button
      @click="minimize"
      variant="highlight"
      size="icon"
  >
    <Icon :path="mdiMinus" />
  </Button>

  <Button
      @click="close"
      variant="highlight"
      size="icon"
  >
    <Icon :path="mdiClose" />
  </Button>
</template>

<script setup lang="ts">
import { mdiClose, mdiMinus } from "@mdi/js";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

function minimize() {
  appWindow.minimize();
}

function close() {
  appWindow.close();
}
</script>