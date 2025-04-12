<template>
  <ScrollablePage>
    <div v-for="i in 100" :key="i">
      {{ i }}
    </div>
  </ScrollablePage>
</template>

<script setup lang="ts">
import { path } from "@/lib/router.ts";

onMounted(() => {
  path.value = "version.index";
});
</script>