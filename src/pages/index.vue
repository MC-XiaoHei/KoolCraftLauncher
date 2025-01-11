<template>
  <div class="flex flex-col gap-3">
    <div>
      <canvas id="skin-viewer"></canvas>
    </div>
    <v-spacer />
    <div class="w-full text-center text-monocraft">
      <span v-if="accountStore.currentAccount === null">
        {{ t("pages.index.label.no-available-account") }}
      </span>
      <span v-else>
        [{{
          t(`pages.index.label.account-type.${ accountStore.currentAccountTypeLabel() }`)
        }}]&thinsp;{{ accountStore.currentAccount.name }}
      </span>
    </div>
    <div class="flex gap-3">
      <v-btn :prepend-icon="mdiCompassOutline"
             class="flex-grow !h-full justify-start"
             size="x-large"
             variant="tonal"
      >
        <span class="mt--1">
          {{ t("pages.index.label.explore-resources") }}
        </span>
      </v-btn>
      <v-btn
          :icon="mdiCog"
          rounded
          variant="tonal"
      />
      <v-btn
          :icon="mdiApps"
          rounded
          variant="tonal"
      />
    </div>
    <div class="flex rounded-1 overflow-clip h-18">
      <v-btn :rounded="false"
             class="flex-grow !h-18 justify-start"
             stacked
             variant="tonal"
      >
        <div class="items-start flex flex-col">
          <span class="text-h5">{{ t("pages.index.label.launch-game") }}</span>
          <span class="text-medium-emphasis text-transform-none">1.21.4-RE:Vanilla-1.0.0</span>
        </div>
      </v-btn>
      <v-btn :icon="mdiChevronDoubleUp"
             :rounded="false"
             class="!h-18 !w-12"
             size="x-large"
             variant="tonal"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { mdiApps, mdiChevronDoubleUp, mdiCog, mdiCompassOutline } from "@mdi/js";
import { useAccountStore } from "../store/account/account.ts";

const { t } = useI18n();
const accountStore = useAccountStore();
</script>

<style lang="scss" scoped>
.text-transform-none {
  text-transform: none;
}
</style>