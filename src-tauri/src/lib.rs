use crate::vibrancy::VibrancyStateStore;
use tauri::{Manager, State};
use tauri_plugin_os::Version::Semantic;

mod setup;
mod vibrancy;
mod account;

#[tauri::command]
fn get_vibrancy_state(state: State<'_, VibrancyStateStore>) -> String {
    state.get()
}

#[tauri::command]
fn should_custom_window() -> bool {
    let os_version = tauri_plugin_os::version();
    match os_version {
        Semantic(10, _, patch) => {
            !setup::is_win11(patch)
        }
        _ => true,
    }
}

#[tauri::command]
async fn start_microsoft_login(app: tauri::AppHandle) {
    account::microsoft::open_microsoft_login_webview(app).await;
}

#[tauri::command]
async fn terminate_microsoft_login(app: tauri::AppHandle) {
    account::microsoft::close_microsoft_login_webview(app).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(prevent_default())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window");
        }))
        .manage(VibrancyStateStore::new())
        .setup(|app| {
            let vibrancy_state = setup::setup_window(app).unwrap();
            let state = app.state::<VibrancyStateStore>();
            state.set(vibrancy_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_vibrancy_state,
            should_custom_window,
            start_microsoft_login,
            terminate_microsoft_login
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

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}
