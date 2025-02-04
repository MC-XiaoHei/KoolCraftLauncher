import { Account, AccountType } from "../models.ts";
import { AccountProvider, SkinData } from "./AccountProvider.ts";

export class MicrosoftAccountProvider extends AccountProvider {
  build(name: string, uuid: string): Account {
    return {
      type: AccountType.Microsoft,
      namespace: "microsoft",
      uuid,
      name: name,
    };
  }

  async getSkinData(account: Account): Promise<SkinData> {
    return {
      skinUrl: `https://crafatar.com/skins/${account.uuid}`,
      capeUrl: `https://crafatar.com/capes/${account.uuid}`
    }
  }
}