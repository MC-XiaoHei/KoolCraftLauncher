<template>
  <Button variant="highlight"
          size="icon"
          @click="toggleTheme"
          class="rounded-none hover:bg-transparent">
    <SunMedium class="h-[1.2rem] w-[1.2rem] scale-100 dark:scale-0 opacity-50 transition-opacity" />
    <Moon class="absolute h-[1.2rem] w-[1.2rem] scale-0 dark:scale-100 opacity-50 transition-opacity" />
    <span class="sr-only">Toggle theme</span>
  </Button>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { useColorMode } from "@vueuse/core";
import { Moon, SunMedium } from "lucide-vue-next";

const mode = useColorMode();
const isDark = ref(mode.value === "dark");

const toggleTheme = () => {
  isDark.value = !isDark.value;
  mode.value = isDark.value ? "dark" : "light";
};
</script>