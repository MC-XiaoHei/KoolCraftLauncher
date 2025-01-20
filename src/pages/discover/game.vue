<template>
  <div v-if="status == FetchVerListStatus.Ok">
    <div class="pb-6">
      <v-card class="rounded-md card mb-3" flat>
        <v-card-title class="text-subtitle-1">{{ t("pages.discover.game.label.latest") }}</v-card-title>
        <v-card-text>
          <div class="flex flex-col">
            <game-version-card
                icon="release"
                :version="latestReleaseData?.id ?? ''"
                :text="formatDate(latestReleaseData?.releaseTime ?? '')"
            />
            <game-version-card
                icon="snapshot"
                :version="latestSnapshotData?.id ?? ''"
                :text="formatDate(latestSnapshotData?.releaseTime ?? '')"
            />
          </div>
        </v-card-text>
      </v-card>

      <v-expansion-panels class="rounded-md card mb-3" flat static>
        <v-expansion-panel :expand-icon="mdiChevronDown" :collapse-icon="mdiChevronDown">
          <template v-slot:title>
            <span>{{ t("pages.discover.game.label.releases") }}</span>
          </template>
          <template v-slot:text>
            <div :style="{
              height: `${(verListData?.versions.filter((v) => v.type === 'release').length ?? 0) * 52}px`,
            }">
              <v-lazy
                  v-for="version in verListData?.versions.filter((v) => v.type === 'release')"
                  :min-height="52"
                  :key="version.id"
                  :options="{'threshold':0.1}"
                  transition="fade-transition"
              >
                <game-version-card
                    class="w-full"
                    icon="release"
                    :version="version.id"
                    :text="formatDate(version.releaseTime)"
                />
              </v-lazy>
            </div>
          </template>
        </v-expansion-panel>
      </v-expansion-panels>

      <v-expansion-panels class="rounded-md card" flat static>
        <v-expansion-panel :expand-icon="mdiChevronDown" :collapse-icon="mdiChevronDown">
          <template v-slot:title>
            <span>{{ t("pages.discover.game.label.snapshots") }}</span>
          </template>
          <template v-slot:text>
            <div :style="{
              height: `${(verListData?.versions.filter((v) => v.type === 'snapshot').length ?? 0) * 52}px`,
            }">
              <intersection
                  v-for="version in verListData?.versions.filter((v) => v.type === 'snapshot')"
                  :key="version.id"
                  class="h-13"
              >
                <game-version-card
                    class="w-full"
                    icon="snapshot"
                    :version="version.id"
                    :text="formatDate(version.releaseTime)"
                />
              </intersection>
            </div>
          </template>
        </v-expansion-panel>
      </v-expansion-panels>
    </div>
  </div>
  <div v-else-if="status == FetchVerListStatus.Fetching" class="flex flex-col justify-center items-center h-full">
    <span class="text-body-1 pb-2">{{ t("pages.discover.game.label.fetching") }}</span>
    <v-progress-linear indeterminate />
  </div>
  <div v-else class="flex flex-col justify-center items-center h-full">
    <span class="text-body-1 pb-2">{{ t("pages.discover.game.label.error") }}</span>
    <v-btn @click="() => fetchVersion()" variant="text">{{ t("pages.discover.game.label.retry") }}</v-btn>
  </div>
</template>

<script setup lang="ts">
import { mdiChevronDown } from "@mdi/js";
import { fetch } from "@tauri-apps/plugin-http";
import GameVersionCard from "../../components/GameVersionCard.vue";
import Intersection from "../../components/Intersection.vue";
import { formatDate } from "../../utils/date-utils.ts";

enum FetchVerListStatus {
  Fetching,
  Ok,
  Error,
}

interface VersionData {
  id: string;
  type: string;
  url: string;
  time: string;
  releaseTime: string;
}

interface VerListData {
  latest: {
    release: string;
    snapshot: string;
  };
  versions: VersionData[];
}

const { t } = useI18n();
const status = ref(FetchVerListStatus.Fetching);
const verListData = ref<VerListData | null>(null);
const latestReleaseData = computed(() =>
    verListData.value?.versions.find((v) => v.id === verListData.value?.latest.release),
);
const latestSnapshotData = computed(() =>
    verListData.value?.versions.find((v) => v.id === verListData.value?.latest.snapshot),
);

onMounted(() => {
  fetchVersion().then();
});

async function fetchVersion() {
  try {
    status.value = FetchVerListStatus.Fetching;
    const response = await fetch("https://piston-meta.mojang.com/mc/game/version_manifest.json", {
      method: "GET",
    });
    if (!response.ok) {
      status.value = FetchVerListStatus.Error;
      return;
    }
    status.value = FetchVerListStatus.Ok;
    verListData.value = await response.json();
    console.log(verListData.value);
  } catch (e) {
    status.value = FetchVerListStatus.Error;
  }
}
</script>

<style scoped lang="scss">
.card {
  background-color: rgb(var(--v-theme-surface));
}


:deep(.v-expansion-panel-text__wrapper) {
  padding: 0 16px 16px;
}

:deep(.v-expansion-panel-title__icon) {
  transition: transform 0.3s ease;
}

:deep(.v-expansion-panel-title--active .v-expansion-panel-title__icon) {
  transform: rotate(180deg);
}
</style>