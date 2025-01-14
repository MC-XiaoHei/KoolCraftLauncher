import { calcOfflineUuid, getDefaultSkinUrl } from "../../../utils/offline-utils.ts";
import { Account, AccountType } from "../models.ts";
import { AccountProvider, SkinData } from "./AccountProvider.ts";

export class OfflineAccountProvider extends AccountProvider {
  build(name: string): Account {
    return {
      type: AccountType.Offline,
      namespace: "offline",
      uuid: calcOfflineUuid(name),
      name: name,
    };
  }

  async getSkinData(account: Account): Promise<SkinData> {
    return {
      skinUrl: getDefaultSkinUrl(account.uuid),
    };
  }
}