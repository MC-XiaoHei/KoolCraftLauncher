export interface DownloadGroup {
  downloadSpeed: number;
  totalSize: number;
  totalDownloaded: number;
  name?: string;
  time?: Date;
}

export type DownloadGroups = Map<string, DownloadGroup>;