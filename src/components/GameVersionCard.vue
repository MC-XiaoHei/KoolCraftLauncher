<template>
  <v-btn class="justify-start">
    <template v-slot:prepend>
      <v-img class="ml--1" :src="iconUrl" width="32" height="32" />
    </template>
    <div class="flex flex-col items-start">
      <span class="text-body-1">{{ props.version }}</span>
      <span class="mt--2 text-caption text-disabled">{{ props.text }}</span>
    </div>
  </v-btn>
</template>

<script setup lang="ts">
const props = defineProps<{
  icon: string;
  version: string;
  text: string;
}>();

const iconUrl = computed(() => {
  switch (props.icon) {
    case "release":
      return "/minecraft-assets/grass-block.png";
    case "snapshot":
      return "/minecraft-assets/command-block.gif";
    case "old":
      return "/minecraft-assets/cobblestone.png";
    case "fool":
      return "/minecraft-assets/slime-block.png";
  }
});
</script>

<style scoped lang="scss">
</style>