import { Account, AccountType } from "./models.ts";
import { AccountProvider } from "./providers/AccountProvider.ts";
import { MicrosoftAccountProvider } from "./providers/MicrosoftAccountProvider.ts";
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

  function currentAccountTypeLabel(): string {
    if (!currentAccount.value) {
      return "";
    }
    return getAccountTypeLabel(currentAccount.value!);
  }

  function getAccountTypeLabel(account: Account): string {
    return account.type.toString();
  }

  function addAccount(account: Account) {
    accounts.value.push(account);
  }

  function isCurrentAccount(account: Account): boolean {
    return currentAccount.value?.uuid === account.uuid
        && currentAccount.value?.namespace === account.namespace;
  }

  return {
    accounts,
    currentAccount,
    currentAccountTypeLabel,
    getAccountTypeLabel,
    addAccount,
    isCurrentAccount,
  };
}, {
  persist: true,
});

export const AccountProviders = {
  Offline: new OfflineAccountProvider(),
  Microsoft: new MicrosoftAccountProvider(),
  get(account: Account): AccountProvider {
    switch (account.type) {
      case AccountType.Offline:
        return AccountProviders.Offline;
      case AccountType.Microsoft:
        return AccountProviders.Microsoft;
      default:
        throw new Error(`Unknown account type: ${ account.type }`); // TODO
    }
  },
};