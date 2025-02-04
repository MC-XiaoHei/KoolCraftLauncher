export interface VersionData {
  id: string;
  type: string;
  url: string;
  time: string;
  releaseTime: string;
}

export interface VersionListData {
  latest: {
    release: string;
    snapshot: string;
  };
  versions: VersionData[];
}

export enum VersionCacheStatus {
  Fetching,
  Ok,
  Error,
}