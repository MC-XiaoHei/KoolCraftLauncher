<template>
  <div class="flex flex-col gap-3">
    <v-spacer />
    <div v-show="noAvailableAccount" class="w-full flex justify-center">
      <v-btn
          variant="outlined"
          size="large"
          @click="router.push('/account/add')"
      >
        <template v-slot:prepend>
          <v-icon size="32" :icon="mdiAccountPlusOutline" />
        </template>
        <span class="mt--0.25">{{ t("pages.index.label.add-account") }}</span>
      </v-btn>
    </div>
    <div class="pos-relative overflow-clip flex-grow-1000"
         v-show="!noAvailableAccount"
         ref="skinViewerContainer"
         v-resize="resizeSkinViewer"
    >
      <canvas class="pos-absolute top-1/2 left-1/2 transform -translate-1/2"
              id="skin-viewer"
      ></canvas>
      <div v-if="!noAvailableAccount"
           class="pos-absolute opacity-75 rounded-1 overflow-clip h-7"
      >
        <v-btn-group>
          <v-btn
              size="28"
              variant="tonal"
              v-tooltip:bottom="t('pages.index.label.change-skin-tooltip')"
              v-show="accountStore.currentAccount?.type !== AccountType.Offline"
          >
            <v-icon :icon="mdiHanger" size="18" />
          </v-btn>
          <v-btn
              size="28"
              variant="tonal"
              v-tooltip:bottom="t('pages.index.label.modify-account-info-tooltip')"
              v-show="accountStore.currentAccount?.type !== AccountType.Offline"
          >
            <v-icon :icon="mdiPencilOutline" size="18" />
          </v-btn>
          <v-btn
              size="28"
              variant="tonal"
              v-tooltip:bottom="t('pages.index.label.account-list-tooltip')"
              @click="router.push('/account/list')"
          >
            <v-icon :icon="mdiFormatListBulletedType" size="18" />
          </v-btn>
        </v-btn-group>
      </div>
    </div>
    <v-spacer />
    <div class="w-full text-center text-monocraft">
      <span v-if="noAvailableAccount">
        {{ t("pages.index.label.no-available-account") }}
      </span>
      <span v-else>
        [{{
          t(`pages.index.label.account-type.${ accountStore.currentAccountTypeLabel() }`)
        }}]&thinsp;{{ accountStore.currentAccount!.name }}
      </span>
    </div>
    <div class="flex gap-3">
      <v-btn class="flex-grow !h-full justify-start"
             size="x-large"
             variant="tonal"
      >
        <template v-slot:prepend>
          <v-icon class="ml--1" :icon="mdiCompassOutline" size="32" />
        </template>
        <span class="mt--1">
          {{ t("pages.index.label.explore-resources") }}
        </span>
      </v-btn>
      <v-btn
          size="48"
          variant="tonal"
      >
        <v-icon :icon="mdiCog" size="32" />
      </v-btn>
      <v-btn
          size="48"
          variant="tonal"
      >
        <v-icon :icon="mdiApps" size="32" />
      </v-btn>
    </div>
    <div class="flex rounded-1 overflow-clip h-18">
      <v-btn :rounded="false"
             class="flex-grow !h-18 justify-start"
             stacked
             variant="tonal"
      >
        <div class="items-start flex flex-col">
          <span class="text-h5">{{ t("pages.index.label.launch-game") }}</span>
          <span class="text-medium-emphasis text-transform-none">1.21.4-RE:Vanilla-1.0.0</span>
        </div>
      </v-btn>
      <v-btn
          :rounded="false"
          class="!h-18 !w-12"
          size="48"
          variant="tonal"
      >
        <v-icon :icon="mdiChevronDoubleUp" size="32" />
      </v-btn>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  mdiAccountPlusOutline,
  mdiApps,
  mdiChevronDoubleUp,
  mdiCog,
  mdiCompassOutline,
  mdiFormatListBulletedType,
  mdiHanger,
  mdiPencilOutline,
} from "@mdi/js";
import { SkinViewer, WalkingAnimation } from "skinview3d";
import { AccountProviders, useAccountStore } from "../store/account/account.ts";
import { AccountType } from "../store/account/models.ts";

const { t } = useI18n();
const accountStore = useAccountStore();
const router = useRouter();
const noAvailableAccount = computed(() => accountStore.currentAccount === null);
const skinViewerContainer = ref<HTMLDivElement | null>(null);
let skinViewer: SkinViewer;

function updateSkinViewerSkin() {
  if (accountStore.currentAccount) {
    const provider = AccountProviders.get(accountStore.currentAccount);
    if (skinViewer) {
      skinViewer.loadSkin(provider.skinUrl(accountStore.currentAccount));
      const capeUrl = provider.capeUrl(accountStore.currentAccount);
      if (capeUrl) {
        skinViewer.loadCape(capeUrl);
      }
    }
  }
}

function resizeSkinViewer() {
  if (skinViewerContainer.value && skinViewer) {
    skinViewer.width = skinViewerContainer.value.clientWidth;
    skinViewer.height = skinViewerContainer.value.clientHeight;
  }
}

onMounted(() => {
  skinViewer = new SkinViewer({
    canvas: document.getElementById("skin-viewer") as HTMLCanvasElement,
    width: 200,
    height: 200,
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

  updateSkinViewerSkin();
  resizeSkinViewer();
});

watch(() => accountStore.currentAccount, () => updateSkinViewerSkin());
</script>