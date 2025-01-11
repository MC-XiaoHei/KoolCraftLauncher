import { Account } from "../models.ts";

export abstract class AccountProvider<T extends Account> {
  abstract build(name: string): T;

  abstract skinUrl(account: T): string;

  abstract capeUrl(account: T): string | null;
}