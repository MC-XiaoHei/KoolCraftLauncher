<template>
  <ScrollablePage class="flex flex-col gap-3">
    <Button variant="ghost" v-for="(category, index) in categories" :key="index" class="flex w-full h-12">
      <component :size="32"
                 :strokeWidth="1.5"
                 class="size-8 opacity-80"
                 :is="category.icon"
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
  icon: any;
}

const categories: DiscoverCategory[] = [
  {
    id: "mods",
    icon: Box,
  },
  {
    id: "resourcePacks",
    icon: Paintbrush,
  },
  {
    id: "dataPacks",
    icon: Braces,
  },
];
</script>