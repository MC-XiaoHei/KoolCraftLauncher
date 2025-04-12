<template>
  <div ref="skinViewerContainer" :class="props.class" :id="id">
    <SkinView3d
        ref="skinViewer3d"
        :render-paused="false"
        :height="height"
        :width="width"
        :layers="config.layers"
        :enable-rotate="config.controls.rotate"
        :enable-zoom="config.controls.zoom"
        :enable-pan="config.controls.pan"
        :global-light="config.light.global"
        :camera-light="config.light.camera"
        :auto-rotate="config.autoRotate.enabled"
        :auto-rotate-speed="config.autoRotate.speed"
        :animation="animation as never"
        skin-url="http://textures.minecraft.net/texture/44fe2178b884d0f6d255c142c0c0e79c8d2a182bc26176e12dc123449245d4bc"
    />
  </div>
</template>
<script setup lang="ts">
import { getAnimation, getSkinViewerConfig } from "@/lib/index-skin-viewer-config.ts";
import { useEventListener } from "@vueuse/core";
import type { HTMLAttributes } from "vue";
import { SkinView3d } from "vue-skinview3d";

interface Props {
  id: string;
  class?: HTMLAttributes["class"];
}

const props = defineProps<Props>();
const skinViewerContainer = ref<HTMLElement | null>(null);
const height = ref(0);
const width = ref(0);
const skinViewer3d = ref(null);
// @ts-expect-error
const skinViewer = computed(() => skinViewer3d.value!.viewer);
const config = getSkinViewerConfig(props.id);
const animation = getAnimation(props.id);

onMounted(() => {
  resizeViewer();
  updateCamera();
});

useEventListener("resize", resizeViewer);

watch(config.value, updateCamera);

function updateCamera() {
  skinViewer.value.camera.position.set(
      config.value.camera.position.x,
      config.value.camera.position.y,
      config.value.camera.position.z,
  );
  skinViewer.value.camera.rotation.set(
      config.value.camera.rotation.x,
      config.value.camera.rotation.y,
      config.value.camera.rotation.z,
  );
  skinViewer.value.camera.zoom = config.value.camera.zoom;
}

function resizeViewer() {
  const container = skinViewerContainer.value;
  if (container) {
    width.value = container.clientWidth;
    height.value = container.clientHeight;
  }
  skinViewer.value.pixelRatio = window.devicePixelRatio * config.value.pixelRatioByDevice;
}
</script>