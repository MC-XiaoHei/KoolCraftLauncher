export interface DownloadGroup {
  downloadSpeed: number;
  totalSize: number;
  totalDownloaded: number;
  name?: string;
}

export type DownloadGroups = Map<string, DownloadGroup>;