<template>
  <ScrollablePage class="flex flex-col gap-3">
    <Button variant="ghost" v-for="(category, index) in categories" :key="index" class="flex w-full h-12">
      <Icon class="size-8" v-if="category.iconPath" :path="category.iconPath" />
      <component :size="32"
                 :strokeWidth="1.5"
                 class="size-8 opacity-80"
                 :is="category.iconComponent"
                 v-else-if="category.iconComponent"
      />
      <div class="flex-grow flex flex-col items-start">
        <span>
          {{ t(`pages.discover.index.categories.${ category.id }.title`) }}
        </span>
        <span class="opacity-50 text-xs -mt-1">
          {{ t(`pages.discover.index.categories.${ category.id }.description`) }}
        </span>

      </div>
    </Button>

  </ScrollablePage>
</template>

<script setup lang="ts">
import { path } from "@/lib/router.ts";
import { Box, Braces, Paintbrush } from "lucide-vue-next";
import { useI18n } from "vue-i18n";

const {t} = useI18n();

onMounted(() => {
  path.value = "discover.index";
});

interface DiscoverCategory {
  id: string;
  iconPath?: string;
  iconComponent?: any;
}

const categories: DiscoverCategory[] = [
  {
    id: "mods",
    iconComponent: Box,
  },
  {
    id: "resourcePacks",
    iconComponent: Paintbrush,
  },
  {
    id: "dataPacks",
    iconComponent: Braces,
  },
];
</script>