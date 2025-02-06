use crate::task_manager::task_group::TaskGroupInfo;
use crate::task_manager::TASK_MANAGER;

#[tauri::command]
pub fn get_task_group_infos() -> Vec<TaskGroupInfo> {
	TASK_MANAGER.get_group_infos()
}