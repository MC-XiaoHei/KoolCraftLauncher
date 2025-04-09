<template>
  <Page class="flex flex-col gap-3">
    <div class="flex-grow relative">
      <SkinViewer class="size-full" />
      <div class="absolute top-0 left-0">
<!--        <Button-->
<!--            variant="highlight"-->
<!--            size="icon"-->
<!--        >-->
<!--          <ChevronLeft class="size-5 text-gray-500 transition-colors" />-->
<!--        </Button>-->
      </div>
    </div>
    <div class="h-3 w-full flex items-center justify-center gap-1">
      <div>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="24px" height="24px">
          <path fill="#ff5722" d="M6 6H22V22H6z" transform="rotate(-180 14 14)" />
          <path fill="#4caf50" d="M26 6H42V22H26z" transform="rotate(-180 34 14)" />
          <path fill="#ffc107" d="M26 26H42V42H26z" transform="rotate(-180 34 34)" />
          <path fill="#03a9f4" d="M6 26H22V42H6z" transform="rotate(-180 14 34)" />
        </svg>
      </div>
      <div class="font-minecraft mb-0.5">
        MC_XiaoHei
      </div>
    </div>
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
            1.21.5-RE:Vanilla
          </div>
        </div>
      </Button>
      <Button variant="outline" class="h-18 w-12 rounded-l-none border-l-transparent!">
        <ChevronsUp stroke-width="1.5" class="size-6" />
      </Button>
    </div>
  </Page>
</template>
<script setup lang="ts">
import { ChevronLeft, ChevronsUp, Compass, LayoutGrid, Settings } from "lucide-vue-next";
</script>