<template>
  <v-empty-state
      :title="t('pages.dynamic.label.not-found')"
      :text="router.currentRoute.value.fullPath"
      :action-text="t('pages.dynamic.label.back-to-home')"
      @click:action="() => router.push('/')"
  ></v-empty-state>
</template>

<script setup lang="ts">
const router = useRouter();
const { t } = useI18n();
</script>

<style scoped lang="scss">

</style>