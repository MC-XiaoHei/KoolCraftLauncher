use tauri::State;
use tauri_plugin_os::Version::Semantic;
use utils::window::setup_window;
use utils::window::vibrancy::VibrancyStateStore;

#[tauri::command]
pub fn get_vibrancy_state(state: State<'_, VibrancyStateStore>) -> String {
	state.get()
}

#[tauri::command]
pub fn should_custom_window() -> bool {
	let os_version = tauri_plugin_os::version();
	match os_version {
		Semantic(10, _, patch) => !setup_window::is_win11(patch),
		_ => true,
	}
}