<template>
  <div class="flex flex-col gap-3 text-transform-none">
    <v-btn
        class="justify-start"
        v-for="category in categories"
        :key="category.route"
        :to="category.route"
    >
      <template v-slot:prepend>
        <v-icon class="ml--1 mb-1" :icon="category.icon" size="32" />
      </template>
      <span class="ml-2 text-body-1">
          {{ category.text }}
        </span>
    </v-btn>
  </div>
</template>

<script setup lang="ts">
import { mdiBrushVariant, mdiCodeJson, mdiCubeOutline, mdiGlasses, mdiPackageVariant, mdiPickaxe } from "@mdi/js";

const { t } = useI18n();

const categories = [
  { icon: mdiPickaxe, text: t("pages.discover.label.game"), route: "/discover/game" },
  { icon: mdiCubeOutline, text: t("pages.discover.label.mod"), route: "/discover/mod" },
  { icon: mdiBrushVariant, text: t("pages.discover.label.resource-pack"), route: "/discover/resource-pack" },
  { icon: mdiGlasses, text: t("pages.discover.label.shader"), route: "/discover/shader" },
  { icon: mdiCodeJson, text: t("pages.discover.label.data-pack"), route: "/discover/data-pack" },
  { icon: mdiPackageVariant, text: t("pages.discover.label.mod-pack"), route: "/discover/mod-pack" },
];
</script>

<style scoped lang="scss">
.text-transform-none :deep(.v-btn__content) {
  text-transform: none;
  font-size: xx-small;
}
</style>