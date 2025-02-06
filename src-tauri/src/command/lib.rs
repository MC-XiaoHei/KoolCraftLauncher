use tauri::{Builder, Wry};
use crate::window::*;
use crate::account::*;
use crate::task_manager::*;
use crate::install::*;

mod window;
mod account;
mod task_manager;
mod install;

pub fn invoke_handler(builder: Builder<Wry>) -> Builder<Wry> {
	
	builder.invoke_handler(tauri::generate_handler![
		// windows commands
		get_vibrancy_state,
		should_custom_window,
		// account commands
		start_microsoft_login,
		terminate_microsoft_login,
		// install commands
		install_vanilla,
		// task manager commands
		get_task_group_infos,
	])
}