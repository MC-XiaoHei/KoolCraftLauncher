<template>
  <svg
      :width="size"
      :height="size"
      :viewBox="viewBox"
      :stroke-width="computedStrokeWidth"
      :class="props.class"
      xmlns="http://www.w3.org/2000/svg"
      class="inline-block align-middle flex-shrink-0"
  >
    <path fill="currentColor" :d="path" />
  </svg>
</template>

<script setup lang="ts">
import { computed, type HTMLAttributes } from "vue";

interface Props {
  path: string;
  size?: number | string;
  strokeWidth?: number;
  absoluteStrokeWidth?: number;
  viewBoxSize?: number;
  class?: HTMLAttributes['class']
}

const props = withDefaults(defineProps<Props>(), {
  size: 24,
  strokeWidth: 1.5,
  absoluteStrokeWidth: undefined,
  viewBoxSize: 24,
});

const viewBox = computed(() => {
  return `0 0 ${props.viewBoxSize} ${props.viewBoxSize}`;
});

const computedStrokeWidth = computed(() => {
  return props.absoluteStrokeWidth !== undefined
      ? props.absoluteStrokeWidth
      : props.strokeWidth;
});
</script>