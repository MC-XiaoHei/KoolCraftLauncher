<template>
  <div ref="skinViewerContainer" :class="props.class" :id="id">
    <SkinView3d
        ref="skinViewer3d"
        :render-paused="false"
        :height="height"
        :width="width"
        :layers="layers"
        :enable-rotate="controls.rotate"
        :enable-zoom="controls.zoom"
        :enable-pan="controls.pan"
        :global-light="light.global"
        :camera-light="light.camera"
        :auto-rotate="autoRotate.enabled"
        :auto-rotate-speed="autoRotate.speed"
        :animation="animationClass"
        skin-url="http://textures.minecraft.net/texture/44fe2178b884d0f6d255c142c0c0e79c8d2a182bc26176e12dc123449245d4bc"
        :zoom="camera.zoom"
    />
  </div>
</template>
<script setup lang="ts">
import { animationClass, autoRotate, camera, controls, layers, light } from "@/lib/indexSkinViewerConfig.ts";
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

onMounted(() => {
  resizeViewer();
  updateCamera();
});

useEventListener("resize", resizeViewer);

watch(camera, updateCamera);

function updateCamera() {
  skinViewer.value.camera.position.set(
      camera.position.x,
      camera.position.y,
      camera.position.z,
  );
  skinViewer.value.camera.rotation.set(
      camera.rotation.x,
      camera.rotation.y,
      camera.rotation.z,
  );
  skinViewer.value.camera.zoom = camera.zoom
}

function resizeViewer() {
  const container = skinViewerContainer.value;
  if (container) {
    width.value = container.clientWidth;
    height.value = container.clientHeight;
  }
}
</script>