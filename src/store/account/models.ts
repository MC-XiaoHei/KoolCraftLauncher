export const AccountNamespace = {
  Microsoft: "microsoft",
  Offline: "offline",
  UniPass(server: string) {
    return `uni-pass@${ server }`;
  },
  AuthLib(server: string) {
    return `auth-lib@${ server }`;
  },
};

export interface Account {
  namespace: string;
  uuid: string | undefined;
  name: string;
}