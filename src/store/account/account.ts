import { Account, AccountType } from "./models.ts";
import { AccountProvider } from "./providers/AccountProvider.ts";
import { OfflineAccountProvider } from "./providers/OfflineAccountProvider.ts";

export const useAccountStore = defineStore("account", () => {
  const accounts = ref<Account[]>([]);
  const currentAccount = ref<Account | null>(null);

  watch(() => accounts.value.length, (length) => {
    if (length === 0) {
      currentAccount.value = null;
    } else if (length === 1) {
      currentAccount.value = accounts.value[0];
    }
  });

  function currentAccountTypeLabel() {
    try {
      return currentAccount.value!.type.toString();
    } catch (_) {
      return "";
    }
  }

  function addAccount(account: Account) {
    accounts.value.push(account);
    // TODO - Save to tauri storage
  }

  return {
    accounts,
    currentAccount,
    currentAccountTypeLabel,
    addAccount,
  };
}, {
  persist: true,
});

export const AccountProviders = {
  Offline: new OfflineAccountProvider(),
  get(account: Account): AccountProvider<Account> {
    switch (account.type) {
      case AccountType.Offline:
        return AccountProviders.Offline;
      default:
        throw new Error(`Unknown account type: ${ account.type }`); // TODO
    }
  },
};