export enum AccountType {
  Microsoft = "microsoft",
  Offline = "offline",
  UniPass = "uni-pass",
  AuthLib = "auth-lib",
}

export interface Account {
  type: AccountType;
  namespace: string;
  uuid: string;
  name: string;
}