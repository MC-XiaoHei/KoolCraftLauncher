use tauri::AppHandle;
use crate::account::microsoft::{close_microsoft_login_webview, open_microsoft_login_webview};

#[tauri::command]
pub async fn start_microsoft_login(app: AppHandle) {
	open_microsoft_login_webview(app).await;
}

#[tauri::command]
pub async fn terminate_microsoft_login(app: AppHandle) {
	close_microsoft_login_webview(app).await;
}