<template>
  <div class="flex flex-col gap-3">
    <v-card v-for="(group, id) in taskGroupInfos" :key="id">
      <v-card-title>{{ group.name }}</v-card-title>
      <v-card-item class="mt--2">
        <v-progress-linear class="rounded-1 text-grey"
                           color="primary"
                           height="16"
                           v-for="(section, id) in group.sections"
                           :key="id"
                           :model-value="section.progressPercent">
          <span>{{ section.name }}</span>
        </v-progress-linear>
      </v-card-item>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { taskGroupInfos } from "@/data/install/install.ts";
</script>

<style scoped lang="scss">
span {
  font-size: 14px;
  mix-blend-mode: difference;
}
</style>