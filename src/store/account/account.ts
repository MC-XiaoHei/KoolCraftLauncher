import { Account, AccountNamespace } from "./models.ts";

export const useAccountStore = defineStore("account", () => {
  const accounts = ref<Account[]>([]);
  const currentAccount = ref<Account | null>(null);

  watch(accounts, (value) => {
    if (value.length === 0) {
      currentAccount.value = null;
    } else if (value.length === 1) {
      currentAccount.value = value[0];
    }
  });

  function currentAccountTypeLabel() {
    if (currentAccount.value) {
      const namespace = currentAccount.value.namespace;
      switch (namespace) {
        case AccountNamespace.Microsoft:
          return "microsoft";
        case AccountNamespace.Offline:
          return "offline";
        default: {
          if (namespace.startsWith("uni-pass@")) {
            return "uni-pass";
          } else if (namespace.startsWith("auth-lib@")) {
            return "auth-lib";
          }
        }
      }
    }
    return "";
  }

  return {
    accounts,
    currentAccount,
    currentAccountTypeLabel,
  };
});