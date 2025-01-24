use crate::vibrancy::VibrancyStateStore;
use download::rux::download_manager::DownloadManager;
use tauri::{App, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use crate::download::rux::store::DownloadManagerStore;

mod setup;
mod vibrancy;
mod account;
mod download;

#[allow(unused_qualifications)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            inject_rux_download_manager(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::vibrancy::get_vibrancy_state,
            crate::vibrancy::should_custom_window,
            crate::account::microsoft::start_microsoft_login,
            crate::account::microsoft::terminate_microsoft_login,
            crate::download::vanilla::install_vanilla,
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

fn inject_rux_download_manager(app: &App) {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(format!(
            "KCl/{}",
            app.package_info().version.to_string()).as_str()
        ).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let download_manager = DownloadManager::new(client);
    app.manage(DownloadManagerStore::new(download_manager));
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}
