<template>
  <div class="flex flex-col">
    <v-form v-model="canAddAccount">
      <div class="h-12">
        <v-text-field autofocus
                      clearable
                      counter
                      variant="outlined"
                      maxlength="16"
                      v-model="username"
                      :rules="[
                        usernameRules.required,
                        usernameRules.size,
                        usernameRules.character,
                        usernameRules.alreadyExists
                      ]"
                      :label="t('pages.account.add.offline.label.name-field')"
        />
      </div>
      <div class="pt-8" v-if="!haveMinecraftAccount">
        <v-checkbox :label="t('pages.account.add.offline.label.support-mojang-when-conditions-allow')"
                    v-model="supportMojangWhenConditionsAllow"
                    :rules="[(value: boolean) => haveMinecraftAccount || value]"
        />
      </div>
    </v-form>
    <v-spacer />
    <v-btn size="x-large"
           variant="tonal"
           :disabled="!canAddAccount"
           :text="t('pages.account.add.offline.label.add-account')"
           @click="() => submit()"
    />
  </div>
</template>

<script setup lang="ts">
import { AccountProviders, useAccountStore } from "../../../store/account/account.ts";
import { AccountType } from "../../../store/account/models.ts";

const { t } = useI18n();
const accountStore = useAccountStore();
const router = useRouter();
const usernameRules = {
  required: (value: string) => !!value
      || t("pages.account.add.offline.label.username-required"),
  size: (value: string) => value.length >= 3
      || t("pages.account.add.offline.label.username-too-short"),
  character: (value: string) => /^[a-zA-Z0-9_]+$/.test(value)
      || t("pages.account.add.offline.label.username-invalid-characters"),
  alreadyExists: (value: string) => !accountStore.accounts
          .some((account) => account.name === value && account.type === AccountType.Offline)
      || t("pages.account.add.offline.label.username-already-exists"),
};
const username = ref("");
const haveMinecraftAccount = computed(() => {
  accountStore.accounts.forEach((account) => {
    if (account.type === AccountType.Microsoft) {
      return true;
    }
  });
  return false;
});
const supportMojangWhenConditionsAllow = ref(false);
const canAddAccount = ref(false);

function submit() {
  accountStore.addAccount(AccountProviders.Offline.build(username.value));
  router.push("/");
}
</script>