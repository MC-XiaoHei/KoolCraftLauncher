<template>
  <div class="flex flex-col gap-3" v-if="taskGroupInfos.length != 0">
    <v-card v-for="(group, id) in taskGroupInfos" :key="id">
      <v-card-title>
        <v-icon class="" size="24" :icon="getTaskIcon(parseInstallTaskName(group.name).type)" />
        <span class="text-subtitle-1 ml-1 text-high-emphasis text-truncate">
          {{ t('pages.install.label.install') + '&thinsp;' + parseInstallTaskName(group.name).versionName }}
        </span>
      </v-card-title>
      <v-card-item class="mt--2">
        <div class="flex w-full text-body2" v-for="(section, id) in group.sections"
             :key="id">
          <div class="w-12 text-medium-emphasis">{{ section.progressPercent }}%</div>
          <div class="flex-grow">{{ section.name }}</div>
        </div>
      </v-card-item>
    </v-card>
  </div>
  <v-empty-state v-else :title="t('pages.install.label.no-task')" />
</template>

<script setup lang="ts">
import { taskGroupInfos } from "@/data/install/install.ts";
import { parseInstallTaskName } from "@/utils/install.ts";
import { mdiHelpBoxOutline } from "@mdi/js";

const { t } = useI18n();

function getTaskIcon(taskType: string) {
  if (taskType.startsWith("install-")) {
    const type = taskType.replace("install-", "");
    switch (type) {
      case "vanilla":
        return "M4,2H20A2,2 0 0,1 22,4V20A2,2 0 0,1 20,22H4A2,2 0 0,1 2,20V4A2,2 0 0,1 4,2M6,6V10H10V12H8V18H10V16H14V18H16V12H14V10H18V6H14V10H10V6H6Z";
      default:
        return mdiHelpBoxOutline;
    }
  }
}
</script>

<style scoped lang="scss">
span {
  font-size: 14px;
  mix-blend-mode: difference;
}
</style>