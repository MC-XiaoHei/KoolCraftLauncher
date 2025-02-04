export interface TaskGroupInfo {
  startTime: string,
  name: string,
  sections: TaskSectionInfo[],
}

export interface TaskSectionInfo {
  name: string,
  progressPercent: number,
}