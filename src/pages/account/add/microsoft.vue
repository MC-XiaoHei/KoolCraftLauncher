<template>
  <div class="flex flex-col justify-center items-center">
    <v-spacer />
    <div class="text-body-1 pb-4 mt--8">
      {{ title }}
    </div>
    <v-progress-linear
        v-if="loginStatus !== MicrosoftLoginStatus.Error && loginStatus !== MicrosoftLoginStatus.Cancelled"
        indeterminate />
    <div v-else-if="loginStatus !== MicrosoftLoginStatus.Cancelled">
      {{
        t(`pages.account.add.microsoft.label.${ error ?? MicrosoftLoginErrorType.UnknownError }`)
      }}
    </div>
    <div v-else>
      {{ t("pages.account.add.microsoft.label.Cancelled") }}
    </div>
    <v-spacer />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { AccountProviders, useAccountStore } from "@/store/account/account.ts";

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
  CreateWindowError = "CreateWindowError",
  OAuthError = "OAuthError",
  NetworkError = "NetworkError",
  ProfileNotFoundError = "ProfileNotFoundError",
  APIError = "APIError",
  FileSystemError = "FileSystemError",
  UnknownError = "UnknownError",
}

// noinspection JSUnusedGlobalSymbols
enum AuthProgress {
  GetAccessToken = "GetAccessToken",
  GetXBLToken = "GetXBLToken",
  GetXSTSToken = "GetXSTSToken",
  GetMinecraftToken = "GetMinecraftToken",
  GetMinecraftProfile = "GetMinecraftProfile",
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
  auth_progress: AuthProgress,
  error?: LoginError;
  profile?: ProfileData;
}

const { t } = useI18n();
const router = useRouter();
const accountStore = useAccountStore();
const appWindow = getCurrentWindow();
const loginStatus = ref(MicrosoftLoginStatus.LoadingPage);
const authProgress = ref<AuthProgress>(AuthProgress.GetAccessToken);
const error = ref<LoginError | null>(null);
const title = computed(() => {
  switch (loginStatus.value) {
    case MicrosoftLoginStatus.Success:
      return "";
    case MicrosoftLoginStatus.Cancelled:
      return "";
    case MicrosoftLoginStatus.Error:
      return t(`pages.account.add.microsoft.label.${ error.value }`);
    case MicrosoftLoginStatus.Authenticating:
      return t(`pages.account.add.microsoft.label.${ authProgress.value }`);
    default:
      return t(`pages.account.add.microsoft.label.${ loginStatus.value }`);
  }
});

appWindow.listen<string>("microsoft-login-status", (event) => {
  const payload = event.payload as unknown as UpdateStatusEvent;
  if (payload.status === MicrosoftLoginStatus.Success) {
    const profile = payload.profile;
    if (!profile) {
      loginStatus.value = MicrosoftLoginStatus.Error;
      error.value = {
        error_type: MicrosoftLoginErrorType.UnknownError,
        message: "",
      };
      return;
    }

    const account = AccountProviders.Microsoft.build(
        profile.name,
        profile.uuid,
    );
    accountStore.addAccount(account);
    accountStore.currentAccount = account;
    router.push("/");
    return;
  }
  loginStatus.value = payload.status;
  authProgress.value = payload.auth_progress ?? AuthProgress.GetAccessToken;
  error.value = payload.error ?? null;
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
  authProgress.value = AuthProgress.GetAccessToken;
});

onBeforeRouteLeave(() => {
  invoke("terminate_microsoft_login");
});
</script>