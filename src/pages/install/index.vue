<template>
  <div class="flex flex-col gap-3">
    <v-card v-for="(group, id) in downloadManagerStore.downloadGroups" :key="id">
      <v-card-title>{{ group.name }}</v-card-title>
      <v-card-item class="mt--2">
        <v-progress-linear class="rounded-1 text-grey" color="primary" height="16" :model-value="Math.max(group.totalDownloaded / group.totalSize * 100, 1.00)">
          <span class="mt-1px">{{ formatBytes(group.totalDownloaded) }} / {{ formatBytes(group.totalSize) }} , {{ formatBytes(group.downloadSpeed) }}/s</span>
        </v-progress-linear>
      </v-card-item>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { useDownloadManagerStore } from "@/store/download/download.ts";
import { formatBytes } from "@/utils/size.ts";

const downloadManagerStore = useDownloadManagerStore();
</script>

<style scoped lang="scss">
span {
  font-size: 14px;
  mix-blend-mode: difference;
}
</style>