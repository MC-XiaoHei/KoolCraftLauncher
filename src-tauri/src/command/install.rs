#[tauri::command]
pub async fn install_vanilla(
	ver_json_url: String,
	minecraft_dir: String,
	version_name: String,
) -> Result<(), String> {
	crate::install::general::install_vanilla(ver_json_url, minecraft_dir, version_name)
		.await
		.map_err(|e| e.to_string())?;
	Ok(())
}