use std::collections::HashMap;
use std::sync::Arc;
use tauri::{App, AppHandle, Manager};
use tauri::http::{HeaderMap, HeaderValue};
use tauri::http::header::USER_AGENT;
use tauri_plugin_http::reqwest;
use crate::download::rux::download_manager::{DownloadGroup, DownloadManager};
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
pub fn get_download_groups(app: AppHandle) -> HashMap<String, DownloadGroup> {
	let rux = get_download_manager(&app);
	rux.get_download_groups()
}