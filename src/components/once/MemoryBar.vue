<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { computed } from "vue";

interface Props {
  allocated: number;
  systemUsed: number;
  systemTotal: number;
  class?: HTMLAttributes["class"];
}

const props = withDefaults(defineProps<Props>(), {
  allocated: 0,
  systemUsed: 0,
  systemTotal: 0,
});

const allocatedPercentage = computed(() => {
  console.log("allocatedPercentage", props.allocated / props.systemTotal * 100);
  return (props.allocated / props.systemTotal) * 100;
});

const systemUsedPercentage = computed(() => {
  console.log("systemUsedPercentage", props.systemUsed / props.systemTotal * 100);
  return (props.systemUsed / props.systemTotal) * 100;
});

function formatMemory(value: number) {
  return `${ value.toFixed(1) } GB`;
}
</script>

<template>
  <div class="relative w-full">
    <div class="relative w-full h-[18px] mb-1.5">
      <div class="absolute text-[0.7rem] text-muted-foreground whitespace-nowrap left-0 select-none">
        系统已用
      </div>
      <div class="absolute text-[0.7rem] text-muted-foreground whitespace-nowrap right-0 select-none">
        游戏分配
      </div>
    </div>

    <MultiProgress
        height="6px"
        :segments="[
          { value: systemUsedPercentage, color: 'primary' },
          { value: systemUsedPercentage + allocatedPercentage, color: 'ring' }
        ]"
        class="w-full"
    />

    <div class="relative w-full h-[18px] mt-1.5">
      <div class="absolute text-[0.75rem] font-medium whitespace-nowrap left-0 select-none">
        {{ formatMemory(systemUsed) }}
      </div>
      <div class="absolute text-[0.75rem] font-medium whitespace-nowrap right-0 select-none">
        {{ formatMemory(allocated) }}
      </div>
    </div>
  </div>
</template>