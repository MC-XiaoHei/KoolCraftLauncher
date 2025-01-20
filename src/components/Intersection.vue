<template>
  <div v-intersect="onIntersection">
    <slot v-if="intersecting" />
  </div>
</template>

<script setup lang="ts">
const intersecting = ref(false);

function onIntersection(isIntersecting: boolean, _: never, __: never) {
  intersecting.value = isIntersecting;
}
</script>

<style scoped lang="scss">

</style>