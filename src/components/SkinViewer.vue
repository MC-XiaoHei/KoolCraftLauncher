<template>
  <div ref="skinViewerContainer">
    <canvas id="skin-viewer" class="size-full" />
  </div>
</template>
<script setup lang="ts">
import { SkinViewer, WalkingAnimation } from "skinview3d";

let skinViewer: SkinViewer;
const skinViewerContainer = ref<HTMLElement | null>(null);

function initSkinViewer() {
  const canvas = document.getElementById("skin-viewer") as HTMLCanvasElement;
  const container = skinViewerContainer.value;
  const width = container?.clientWidth ?? canvas.clientWidth;
  const height = container?.clientHeight ?? canvas.clientHeight;

  console.log("width", width);
  console.log("height", height);

  skinViewer = new SkinViewer({
    canvas: canvas,
    width,
    height,
    animation: new WalkingAnimation(),
    pixelRatio: window.devicePixelRatio * 2,
  });

  skinViewer.camera.rotation.x = -0.62;
  skinViewer.camera.rotation.y = 0.534;
  skinViewer.camera.rotation.z = 0.348;
  skinViewer.camera.position.x = 30.5;
  skinViewer.camera.position.y = 22.0;
  skinViewer.camera.position.z = 42.0;

  skinViewer.zoom = 0.85;
  skinViewer.animation!.speed = 0.5;
  skinViewer.controls.enableZoom = false;
}

onMounted(() => {
  initSkinViewer();
  // noinspection HttpUrlsUsage
  skinViewer.loadSkin("http://textures.minecraft.net/texture/44fe2178b884d0f6d255c142c0c0e79c8d2a182bc26176e12dc123449245d4bc");
});
</script>