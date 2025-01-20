<template>
  <div class="w-full h-full flex flex-col gap-3 position-relative">
    <div v-for="account in accountStore.accounts"
         :key="account.namespace + account.uuid"
         class="px-0 flex h-11 w-full rounded-1 overflow-clip"
    >
      <span>
        <account-avatar :size="44"
                        :account="account"
                        :class="{
                            'rounded-tr-1': !accountStore.isCurrentAccount(account),
                            'rounded-br-1': !accountStore.isCurrentAccount(account),
                        }"
        />
      </span>
      <span class="w-full">
        <v-btn size="large"
               :variant="accountStore.isCurrentAccount(account) ? 'tonal' : 'plain'"
               class="pt-0 w-full custom-btn"
               :rounded="false"
               :active="accountStore.isCurrentAccount(account)"
               @click="accountStore.currentAccount = account">
          <div class="w-full flex">
            <span class="text-transform-none text-left text-body-1 ml--3">
              {{ account.name }}
            </span>
            <v-spacer />
            <v-chip class="mr--3"
                    size="small"
                    :text="t(`account.type-short.${ accountStore.getAccountTypeLabel(account) }`)"
            />
          </div>
        </v-btn>
      </span>
    </div>
    <v-fab
        variant="tonal"
        absolute
        :icon="mdiPlus"
        @click="() => router.push('/account/add')"
    />
  </div>
</template>

<script setup lang="ts">
import { mdiPlus } from "@mdi/js";
import AccountAvatar from "../../components/AccountAvatar.vue";
import { useAccountStore } from "../../store/account/account.ts";

const { t } = useI18n();
const router = useRouter();
const accountStore = useAccountStore();
</script>