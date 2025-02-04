use utils::install::manager::task_group::TaskGroupInfo;
use utils::install::manager::task_manager::TaskManager;
use crate::vibrancy::VibrancyStateStore;
use lazy_static::lazy_static;
use tauri::Manager;

mod account;
mod setup;
mod vibrancy;
mod install;

lazy_static! {
	pub static ref TASK_MANAGER: TaskManager = TaskManager::new();
}

#[allow(unused_qualifications)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::new()
				.level(log::LevelFilter::Info)
				.build(),
		)
		.plugin(tauri_plugin_store::Builder::new().build())
		.plugin(prevent_default())
		.plugin(tauri_plugin_os::init())
		.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
			let _ = app
				.get_webview_window("main")
				.expect("no main window")
				.set_focus();
		}))
		.manage(VibrancyStateStore::new())
		.setup(|app| {
			let vibrancy_state = setup::setup_window(app).unwrap();
			app.state::<VibrancyStateStore>().set(vibrancy_state);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			crate::vibrancy::get_vibrancy_state,
			crate::vibrancy::should_custom_window,
			crate::account::microsoft::start_microsoft_login,
			crate::account::microsoft::terminate_microsoft_login,
			crate::install::general::install_vanilla,
			get_task_group_infos,
		])
		.plugin(tauri_plugin_http::init())
		.plugin(tauri_plugin_shell::init())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
	use tauri_plugin_prevent_default::Flags;

	tauri_plugin_prevent_default::Builder::new()
		.with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD))
		.build()
}

#[tauri::command]
fn get_task_group_infos() -> Vec<TaskGroupInfo> {
	TASK_MANAGER.get_group_infos()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
	tauri_plugin_prevent_default::init()
}
