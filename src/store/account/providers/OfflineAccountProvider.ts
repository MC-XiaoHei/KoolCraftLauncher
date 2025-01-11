import { calcOfflineUuid, getDefaultSkinUrl } from "../../../utils/offline-utils.ts";
import { AccountType, OfflineAccount } from "../models.ts";
import { AccountProvider } from "./AccountProvider.ts";

export class OfflineAccountProvider extends AccountProvider<OfflineAccount> {
  build(name: string): OfflineAccount {
    return {
      type: AccountType.Offline,
      namespace: "offline",
      uuid: calcOfflineUuid(name),
      name: name,
    };
  }

  skinUrl(account: OfflineAccount): string {
    return getDefaultSkinUrl(account.uuid);
  }

  capeUrl(_: OfflineAccount): string | null {
    return null;
  }
}