import { TaskGroupInfo } from "@/data/install/models.ts";
import { invoke } from "@tauri-apps/api/core";

export const taskGroupInfos: Ref<TaskGroupInfo[]> = ref([]);

export async function updateTaskGroupInfo() {
  taskGroupInfos.value = await invoke("get_task_group_infos") as TaskGroupInfo[];
  console.log(taskGroupInfos.value);
}