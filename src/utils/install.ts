export interface TaskGroupNameInfo {
  type: string;
  versionName: string;
  minecraftDir: string;
}

export function parseInstallTaskName(taskName: string): TaskGroupNameInfo {
  const [type, versionName, ...minecraftDirParts] = taskName.split("/");
  return {
    type,
    versionName,
    minecraftDir: minecraftDirParts.join("/"),
  };
}