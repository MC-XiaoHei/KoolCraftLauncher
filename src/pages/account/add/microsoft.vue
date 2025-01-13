<template>
  <div>
    {{ microsoftLoginStatus }}
    {{ errorMsg}}
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

function toKebabCase(str: string): string {
  return str.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase();
}

// noinspection JSUnusedGlobalSymbols
enum MicrosoftLoginStatus {
  LoadingPage = "LoadingPage",
  WaitingForOAuth = "WaitingForOAuth",
  Authenticating = "Authenticating",
  Success = "Success",
  Error = "Error",
  Cancelled = "Cancelled",
}

const { t } = useI18n();
const appWindow = getCurrentWindow();
const microsoftLoginStatus = ref(MicrosoftLoginStatus.LoadingPage);
const errorMsg = ref("");

appWindow.listen<string>("microsoft-login-status", (event) => {
  const payload = event.payload;
  if (Object.values(MicrosoftLoginStatus).includes(payload as MicrosoftLoginStatus)) {
    microsoftLoginStatus.value = MicrosoftLoginStatus[payload as keyof typeof MicrosoftLoginStatus];
  } else {
    const object = payload as unknown as any;
    const error = object.Error as string;
    const networkError = object.Error.NetworkError as string;
    if (networkError) {
      errorMsg.value = t("pages.account.add.microsoft.error.label.network-error")
          .replace("{code}", networkError);
    } else {
      const errorId = toKebabCase(error);
      errorMsg.value = t(`pages.account.add.microsoft.label.${errorId}`);
    }
    microsoftLoginStatus.value = MicrosoftLoginStatus.Error;
  }
});

appWindow.listen<string>("microsoft-login-window-closed", () => {
  if (microsoftLoginStatus.value !== MicrosoftLoginStatus.Authenticating &&
      microsoftLoginStatus.value !== MicrosoftLoginStatus.Success &&
      microsoftLoginStatus.value !== MicrosoftLoginStatus.Error
  ) {
    microsoftLoginStatus.value = MicrosoftLoginStatus.Cancelled;
  }
});

watch(microsoftLoginStatus, (status) => {
  if (status === MicrosoftLoginStatus.Success) {
    appWindow.close();
  }
});

onMounted(() => {
  microsoftLoginStatus.value = MicrosoftLoginStatus.LoadingPage;
});

onBeforeRouteLeave(() => {
  invoke("terminate_microsoft_login");
});
</script>