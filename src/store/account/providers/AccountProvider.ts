import { Account } from "../models.ts";

export interface SkinData {
  skinUrl: string,
  capeUrl?: string,
}

export abstract class AccountProvider {
  abstract build(name: string, uuid?: string): Account;

  abstract getSkinData(account: Account): Promise<SkinData>;
}