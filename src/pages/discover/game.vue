<template>
  <div v-if="status == FetchVerListStatus.Ok" class="flex flex-col">
    <v-lazy
        :options="{'threshold':0.5}"
        transition="fade-transition"
    >
      <v-card class="rounded-lg card" flat>
        <v-card-title class="text-h6">{{ t("pages.discover.game.label.latest") }}</v-card-title>
        <v-card-text>
          <div class="flex flex-col">
            <v-btn
                class="justify-start"
                size="x-large"
                variant="text"
                :text="verListData?.latest.release"
            />
          </div>
        </v-card-text>
      </v-card>
    </v-lazy>
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
import { fetch } from "@tauri-apps/plugin-http";

enum FetchVerListStatus {
  Fetching,
  Ok,
  Error,
}

interface VerListData {
  latest: {
    release: string;
    snapshot: string;
  };
  versions: {
    id: string;
    type: string;
    url: string;
    time: string;
    releaseTime: string;
  }[];
}

const { t } = useI18n();
const status = ref(FetchVerListStatus.Fetching);
const verListData = ref<VerListData | null>(null);

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
  background-color: rgb(var(--v-theme-surface-light));
}
</style>