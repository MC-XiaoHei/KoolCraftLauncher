<template>
  <div>
    {{ loginStatus }}<br/>
    <div v-if="loginStatus === MicrosoftLoginStatus.Error">
      {{
        t(`pages.account.add.microsoft.label.${
          toKebabCase((error?.error_type ?? MicrosoftLoginErrorType.UnknownError).toString())
        }`)
      }}
    </div>
    <div v-else-if="loginStatus === MicrosoftLoginStatus.Success">
      {{ profile?.name }} ({{ profile?.uuid }})
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

function toKebabCase(str: string): string {
  return str.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();
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

// noinspection JSUnusedGlobalSymbols
enum MicrosoftLoginErrorType {
  CreateWindowError,
  OAuthError,
  NetworkError,
  ProfileNotFoundError,
  APIError,
  FileSystemError,
  UnknownError,
}

interface ProfileData {
  uuid: string;
  name: string;
}

interface LoginError {
  error_type: MicrosoftLoginErrorType;
  message: string;
}

interface UpdateStatusEvent {
  status: MicrosoftLoginStatus;
  error?: LoginError;
  profile?: ProfileData;
}

const { t } = useI18n();
const appWindow = getCurrentWindow();
const loginStatus = ref(MicrosoftLoginStatus.LoadingPage);
const error = ref<LoginError | null>(null);
const profile = ref<ProfileData | null>(null);

appWindow.listen<string>("microsoft-login-status", (event) => {
  const payload = event.payload as unknown as UpdateStatusEvent;
  loginStatus.value = payload.status;
  error.value = payload.error ?? null;
  profile.value = payload.profile ?? null;
});

appWindow.listen<string>("microsoft-login-window-closed", () => {
  if (loginStatus.value !== MicrosoftLoginStatus.Authenticating &&
      loginStatus.value !== MicrosoftLoginStatus.Success &&
      loginStatus.value !== MicrosoftLoginStatus.Error
  ) {
    loginStatus.value = MicrosoftLoginStatus.Cancelled;
  }
});

onMounted(() => {
  loginStatus.value = MicrosoftLoginStatus.LoadingPage;
});

onBeforeRouteLeave(() => {
  invoke("terminate_microsoft_login");
});
</script>