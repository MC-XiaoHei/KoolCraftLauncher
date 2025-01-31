<template>
  <div class="flex flex-col">
    <v-card>
      <template v-slot:prepend>
        <v-img class="ml--1" :src="getBuiltInGameIcon(game)" width="48" height="48" />
      </template>
      <template v-slot:title>
        <v-text-field class="mb--6"
                      label="版本名"
                      variant="underlined"
                      v-model="name"
                      autofocus
                      :rules="[(_: string) => idValid]"
        />
      </template>
      <div class="mb-2 w-full flex">
        <span class="text-disabled ml-4 text-body-2">
          {{ t("pages.discover.game.install.dynamic.label.no-other-install") }}
        </span>
        <v-spacer />
        <div class="mr-2 h-2">
          <v-btn :disabled="!idValid"
                 size="small"
                 variant="text"
                 color="primary"
                 :text="t('pages.discover.game.install.dynamic.label.install')"
                 class="mt--2"
                 @click="install"
          />
        </div>
      </div>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { useMinecraftVersionCache } from "@/store/cache/minecraft-version-cache.ts";
import { useDownloadManagerStore } from "@/store/download/download.ts";
import { useMinecraftDirStore } from "@/store/game/minecraft-dir-store.ts";
import { useMinecraftGameStore } from "@/store/game/minecraft-game-store.ts";
import { getBuiltInGameIcon } from "@/store/game/models.ts";
import { invoke } from "@tauri-apps/api/core";

const { t } = useI18n();
const router = useRouter();
const minecraftVersionCache = useMinecraftVersionCache();
const minecraftDirStore = useMinecraftDirStore();
const minecraftGameStore = useMinecraftGameStore();
const downloadManagerStore = useDownloadManagerStore();
// @ts-ignore
const game = minecraftVersionCache.buildGame(router.currentRoute.value.params.version, minecraftDirStore.currentDir);
const name = ref(game.id);
const idValid = computed(() =>
    !minecraftGameStore.getGames(minecraftDirStore.currentDir).some((game) => game.id === name.value),
);

function install() {
  const downloadGroupId = downloadManagerStore.createGroup(`install@vanilla@${ name.value }`);
  invoke("install_vanilla", {
    verJsonUrl: minecraftVersionCache.getVersionInfo(game.id)?.url,
    versionName: name.value,
    minecraftDir: minecraftDirStore.currentDir.path,
    downloadGroup: downloadGroupId,
  }).then();
  router.push("/");
}

watch(() => name.value, (value) => {
  if (!value) {
    name.value = game.id;
  }
});
</script>

<style scoped lang="scss">

</style>