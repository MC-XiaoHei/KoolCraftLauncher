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
import { useMinecraftVersionCache } from "@/data/cache/minecraft-version-cache";
import { useMinecraftDirStore } from "@/data/game/minecraft-dir-store";
import { useMinecraftGameStore } from "@/data/game/minecraft-game-store";
import { getBuiltInGameIcon } from "@/data/game/models";
import { invoke } from "@tauri-apps/api/core";

const { t } = useI18n();
const router = useRouter();
const minecraftVersionCache = useMinecraftVersionCache();
const minecraftDirStore = useMinecraftDirStore();
const minecraftGameStore = useMinecraftGameStore();
// @ts-ignore
const game = minecraftVersionCache.buildGame(router.currentRoute.value.params.version, minecraftDirStore.currentDir);
const name = ref(game.id);
const idValid = computed(() =>
    !minecraftGameStore.getGames(minecraftDirStore.currentDir).some((game) => game.id === name.value),
);

function install() {
  invoke("install_vanilla", {
    verJsonUrl: minecraftVersionCache.getVersionInfo(game.id)?.url,
    versionName: name.value,
    minecraftDir: minecraftDirStore.currentDir.path,
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