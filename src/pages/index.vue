<template>
  <Page class="flex flex-col gap-3">
    <Spacer />
    <div class="h-12 flex gap-3">
      <Button variant="outline" class="h-12 flex-grow justify-start">
        <Compass stroke-width="1.5" class="size-6" />
        <div class="text-lg pl-1">
          探索资源
        </div>
      </Button>
      <Button variant="outline" class="size-12">
        <Settings stroke-width="1.5" class="size-6" />
      </Button>
      <Button variant="outline" class="size-12">
        <LayoutGrid stroke-width="1.5" class="size-6" />
      </Button>
    </div>
    <div class="h-18 flex">
      <Button variant="outline" class="h-18 flex-grow rounded-r-none">
        <div class="size-full flex flex-col items-start justify-center">
          <div class="text-[1.35rem]">
            启动游戏
          </div>
          <div class="text-muted-foreground">
            1.21.5-Re:Vanilla
          </div>
        </div>
      </Button>
      <Button variant="outline" class="h-18 w-12 rounded-l-none border-l-transparent">
        <ChevronsUp stroke-width="1.5" class="size-6" />
      </Button>
    </div>
  </Page>
</template>
<script setup lang="ts">
import { ChevronsUp, Compass, Settings, LayoutGrid } from "lucide-vue-next";
</script>