<template>
  <div class="flex flex-col gap-3">
    <v-btn size="x-large"
           variant="outlined"
           disabled
           class="!h-16 custom-btn"
    >
      <template v-slot:default>
        <div class="items-start flex w-full">
          <v-icon class="ml--1" size="32" :icon="microsoftIcon" />
          <v-spacer />
          <span class="text-h6">{{ t("pages.account.add.label.microsoft") }}</span>
          <v-spacer />
        </div>
      </template>
    </v-btn>
    <v-btn size="x-large"
           variant="outlined"
           class="!h-16 custom-btn"
           @click="router.push('/account/add/offline')"
    >
      <template v-slot:default>
        <div class="items-start flex w-full">
          <v-icon class="ml--1" size="32" :icon="mdiCloudOffOutline" />
          <v-spacer />
          <span class="text-h6">{{ t("pages.account.add.label.offline") }}</span>
          <v-spacer />
        </div>
      </template>
    </v-btn>
    <v-btn size="x-large"
           variant="outlined"
           disabled
           class="!h-16 custom-btn"
    >
      <template v-slot:default>
        <div class="items-start flex w-full">
          <v-icon class="ml--1" size="32" :icon="mdiAccountCowboyHatOutline" />
          <v-spacer />
          <span class="text-h6">{{ t("pages.account.add.label.auth-lib") }}</span>
          <v-spacer />
        </div>
      </template>
    </v-btn>
    <v-btn size="x-large"
           variant="outlined"
           disabled
           class="!h-16 custom-btn"
    >
      <template v-slot:default>
        <div class="items-start flex w-full">
          <v-icon class="ml--1" size="32" :icon="mdiAccountTieHatOutline" />
          <v-spacer />
          <span class="text-h6">{{ t("pages.account.add.label.uni-pass") }}</span>
          <v-spacer />
        </div>
      </template>
    </v-btn>
  </div>
</template>

<script setup lang="ts">

import { mdiAccountCowboyHatOutline, mdiAccountTieHatOutline, mdiCloudOffOutline } from "@mdi/js";

const { t } = useI18n();
const router = useRouter();
const microsoftIcon = "M2,3H11V12H2V3M11,22H2V13H11V22M21,3V12H12V3H21M21,22H12V13H21V22Z";

</script>

<style scoped lang="scss">

</style>