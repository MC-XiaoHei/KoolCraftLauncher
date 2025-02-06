use lazy_static::lazy_static;
use crate::task_manager::TaskManager;

pub mod download_task;
pub mod task;
pub mod task_manager;
pub mod task_group;
pub mod task_section;

lazy_static! {
	pub static ref TASK_MANAGER: TaskManager = TaskManager::new();
}