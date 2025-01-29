use std::sync::Arc;
use tauri::{App, AppHandle, Manager};
use tauri::http::{HeaderMap, HeaderValue};
use tauri::http::header::USER_AGENT;
use tauri_plugin_http::reqwest;
use crate::download::rux::download_manager::DownloadManager;
use crate::download::rux::store::DownloadManagerStore;

pub fn inject_rux_download_manager(app: &App) {
	let mut headers = HeaderMap::new();
	headers.insert(
		USER_AGENT,
		HeaderValue::from_str(format!("KCl/{}", app.package_info().version.to_string()).as_str())
			.unwrap(),
	);

	let client = reqwest::Client::builder()
		.default_headers(headers)
		.build()
		.unwrap();

	let download_manager = DownloadManager::new(client);
	app.manage(DownloadManagerStore::new(download_manager));
}

fn get_download_manager(app: &AppHandle) -> Arc<DownloadManager> {
	let store = app.state::<DownloadManagerStore>();
	store.get().clone()
}

#[tauri::command]
pub fn get_download_speed(download_group: String, app: AppHandle) -> Option<u64> {
	let rux = get_download_manager(&app);
	rux.get_downloaded_per_sec(download_group)
}

#[tauri::command]
pub fn is_download_group_exists(download_group: String, app: AppHandle) -> bool {
	let rux = get_download_manager(&app);
	rux.is_download_group_exists(download_group)
}