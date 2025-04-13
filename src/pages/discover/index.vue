<template>
  <ScrollablePage>
    <div class="flex flex-col gap-2 pt-2">
      <Button
          v-for="(category, index) in categories"
          :key="index"
          variant="float"
          class="group h-auto w-full justify-start">
        <div class="icon-container">
          <component :is="category.icon"
                     :size="32"
                     :strokeWidth="1.5" />
        </div>
        <div class="flex-grow">
          <h3 class="text-base font-medium">
            {{ t(`pages.discover.index.categories.${ category.id }.title`) }}
          </h3>
          <p class="text-xs opacity-60">
            {{ t(`pages.discover.index.categories.${ category.id }.description`) }}
          </p>
        </div>
        <div class="group-hover:opacity-50 group-hover:translate-x-0.5
                    transition-all transform opacity-0">
          <ChevronRight />
        </div>
      </Button>
    </div>
  </ScrollablePage>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { path } from "@/lib/router.ts";
import { Blocks, Box, Braces, ChevronRight, Glasses, PackageOpen, Paintbrush } from "lucide-vue-next";
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
  {
    id: "shaders",
    icon: Glasses,
  },
  {
    id: "modPacks",
    icon: PackageOpen,
  },
  {
    id: "plugins",
    icon: Blocks,
  },
];
</script>