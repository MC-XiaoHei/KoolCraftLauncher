<template>
  <div :style="{
    width: `${size}px`,
    height: `${size}px`,
    overflow: 'hidden',
    display: 'inline-block',
  }">
    <div class="head" :style="{
      backgroundImage: `url(${skinUrl})`,
      backgroundSize: `${size * 8}px ${size * 8}px`,
      backgroundPosition: `-${size}px -${size}px`,
      width: `${size}px`,
      height: `${size}px`,
    }"></div>
  </div>
</template>

<script setup lang="ts">
import { AccountProviders } from "@/store/account/account.ts";
import { Account } from "@/store/account/models.ts";

const props = defineProps<{
  account: Account;
  size?: number;
}>();

const provider = computed(() => AccountProviders.get(props.account));
const skinUrl = ref("");
const size = computed(() => props.size || 32);

onMounted(() => {
  provider.value.getSkinData(props.account).then((skinData) => {
    skinUrl.value = skinData.skinUrl;
  });
})
</script>

<style lang="scss" scoped>
.head {
  vertical-align: middle;
  image-rendering: pixelated;
}
</style>