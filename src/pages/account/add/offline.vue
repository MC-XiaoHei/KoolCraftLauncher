<template>
  <div class="flex flex-col">
    <v-form v-model="canAddAccount">
      <div class="h-12 pt-2">
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
      <div class="pt-8" v-if="!accountStore.haveMicrosoftAccount">
        <v-checkbox :label="t('pages.account.add.offline.label.support-mojang-when-conditions-allow')"
                    v-model="supportMojangWhenConditionsAllow"
                    :rules="[(value: boolean) => accountStore.haveMicrosoftAccount || value]"
        />
      </div>
    </v-form>
    <v-spacer />
    <div class="pb-5 w-full">
      <v-btn variant="tonal"
             class="w-full"
             :disabled="!canAddAccount"
             :text="t('pages.account.add.offline.label.add-account')"
             @click="submit"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { AccountProviders, useAccountStore } from "@/data/account/account";
import { AccountType } from "@/data/account/models";

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
const supportMojangWhenConditionsAllow = ref(false);
const canAddAccount = ref(false);

function submit() {
  const account = AccountProviders.Offline.build(username.value);
  accountStore.addAccount(account);
  accountStore.currentAccount = account;
  router.push("/");
}
</script>