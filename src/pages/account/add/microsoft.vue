<template>
  <div>
    {{ microsoftLoginStatus }}
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

enum MicrosoftLoginStatus {
  LoadingPage = "LoadingPage",
  WaitingForOAuth = "WaitingForOAuth",
  Authenticating = "Authenticating",
  Success = "Success",
  Error = "Error",
  Cancelled = "Cancelled",
}

const appWindow = getCurrentWindow();
const microsoftLoginStatus = ref(MicrosoftLoginStatus.LoadingPage);

appWindow.listen<string>("microsoft-login-status", (event) => {
  microsoftLoginStatus.value = MicrosoftLoginStatus[event.payload as keyof typeof MicrosoftLoginStatus];
});

appWindow.listen<string>("microsoft-login-window-closed", () => {
  if (microsoftLoginStatus.value !== MicrosoftLoginStatus.Success &&
      microsoftLoginStatus.value !== MicrosoftLoginStatus.Error) {
    microsoftLoginStatus.value = MicrosoftLoginStatus.Cancelled;
  }
});

onMounted(() => {
  microsoftLoginStatus.value = MicrosoftLoginStatus.LoadingPage;
})

onBeforeRouteLeave(() => {
  invoke("terminate_microsoft_login");
});
</script>