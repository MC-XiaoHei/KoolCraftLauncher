<template>
  <div v-if="versionCache.status == VersionCacheStatus.Ok">
    <v-card class="rounded-md card mb-3" flat>
      <v-card-title class="text-subtitle-1">{{ t("pages.discover.game.label.latest") }}</v-card-title>
      <v-card-text>
        <div class="flex flex-col">
          <game-version-card
              icon="release"
              :version="latestReleaseData?.id ?? ''"
              :text="formatDate(latestReleaseData?.releaseTime ?? '')"
              :on-click="() => router.push(`/discover/game/install/${latestReleaseData?.id ?? ''}`)"
          />
          <game-version-card
              icon="snapshot"
              :version="latestSnapshotData?.id ?? ''"
              :text="formatDate(latestSnapshotData?.releaseTime ?? '')"
              :on-click="() => router.push(`/discover/game/install/${latestSnapshotData?.id ?? ''}`)"
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
              height: `${(versionCache.data?.versions.filter((v) => v.type === 'release').length ?? 0) * 52}px`,
            }">
            <intersection
                v-for="version in versionCache.data?.versions.filter((v) => v.type === 'release')"
                :key="version.id"
                class="h-13"
            >
              <game-version-card
                  class="w-full"
                  icon="release"
                  :version="version.id"
                  :text="formatDate(version.releaseTime)"
                  :on-click="() => router.push(`/discover/game/install/${version.id}`)"
              />
            </intersection>
          </div>
        </template>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-expansion-panels class="rounded-md card mb-3" flat static>
      <v-expansion-panel :expand-icon="mdiChevronDown" :collapse-icon="mdiChevronDown">
        <template v-slot:title>
          <span>{{ t("pages.discover.game.label.snapshots") }}</span>
        </template>
        <template v-slot:text>
          <div :style="{
              height: `${(versionCache.data?.versions.filter((v) => v.type === 'snapshot').length ?? 0) * 52}px`,
            }">
            <intersection
                v-for="version in versionCache.data?.versions.filter((v) => v.type === 'snapshot')"
                :key="version.id"
                class="h-13"
            >
              <game-version-card
                  class="w-full"
                  icon="snapshot"
                  :version="version.id"
                  :text="formatDate(version.releaseTime)"
                  :on-click="() => router.push(`/discover/game/install/${version.id}`)"
              />
            </intersection>
          </div>
        </template>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-expansion-panels class="rounded-md card" flat static>
      <v-expansion-panel :expand-icon="mdiChevronDown" :collapse-icon="mdiChevronDown">
        <template v-slot:title>
          <span>{{ t("pages.discover.game.label.fools-day") }}</span>
        </template>
        <template v-slot:text>
          <div :style="{
              height: `${versionCache.foolsDayVersions.length * 52}px`,
            }">
            <intersection
                v-for="version in versionCache.foolsDayVersions"
                :key="version.id"
                class="h-13"
            >
              <game-version-card
                  class="w-full"
                  icon="fools-day"
                  :version="version.id"
                  :text="t(
                      `pages.discover.game.label.fools-day-tooltips.${
                        version.id.replace(/ /g,'-')
                                  .replace(/\./g, '-')
                      }`,
                       formatDate(version.releaseTime)
                  )"
                  :on-click="() => router.push(`/discover/game/install/${version.id}`)"
              />
            </intersection>
          </div>
        </template>
      </v-expansion-panel>
    </v-expansion-panels>
  </div>
  <div v-else-if="versionCache.status == VersionCacheStatus.Fetching"
       class="flex flex-col justify-center items-center h-full">
    <span class="text-body-1 pb-2">{{ t("pages.discover.game.label.fetching") }}</span>
    <v-progress-linear rounded indeterminate />
  </div>
  <div v-else class="flex flex-col justify-center items-center h-full">
    <v-empty-state
        :title="t('pages.discover.game.label.error')"
        :action-text="t('pages.discover.game.label.retry')"
        @click:action="versionCache.updateData()"
    ></v-empty-state>
  </div>
</template>

<script setup lang="ts">
import { useMinecraftVersionCache } from "@/store/cache/minecraft-version-cache";
import { VersionCacheStatus } from "@/store/cache/models";
import { formatDate } from "@/utils/date";
import { mdiChevronDown } from "@mdi/js";

const { t } = useI18n();
const router = useRouter();
const versionCache = useMinecraftVersionCache();
const latestReleaseData = computed(() =>
    versionCache.data?.versions.find((v) => v.id === versionCache.data?.latest.release),
);
const latestSnapshotData = computed(() =>
    versionCache.data?.versions.find((v) => v.id === versionCache.data?.latest.snapshot),
);

onMounted(() => {
  versionCache.updateData().then();
});
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