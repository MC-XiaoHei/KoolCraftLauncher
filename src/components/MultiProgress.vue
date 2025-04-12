<template>
  <div
      data-slot="multi-progress"
      :class="
      cn(
        'bg-primary/20 relative overflow-hidden rounded-full',
        props.class
      )
    "
      :style="`height: ${props.height};`"
  >
    <div
        v-for="(segment, index) in segments"
        :key="index"
        data-slot="multi-progress-segment"
        class="absolute top-0 bottom-0 h-full transition-all"
        :style="getSegmentStyle(segment, index)"
    />
  </div>
</template>

<script setup lang="ts">
import { cn } from '@/lib/utils'
import { type HTMLAttributes } from 'vue'

interface ProgressSegment {
  value: number
  color?: string
}

interface Props {
  segments: ProgressSegment[]
  class?: HTMLAttributes['class']
  height?: string
}

const props = withDefaults(defineProps<Props>(), {
  segments: () => [],
  height: '0.5rem',
})

function getSegmentStyle(segment: ProgressSegment, index: number) {
  const color = segment.color || 'primary'
  return `
    left: 0;
    width: ${segment.value}%;
    z-index: ${props.segments.length - index};
    background-color: var(--${color});
  `
}
</script>