use crate::download::rux::store::DownloadManagerStore;
use crate::download::utils::{download_version_json, parse_version_json};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn install_vanilla(
    ver_json_url: String,
    minecraft_dir: String,
    version_name: String,
    app: AppHandle,
) -> Result<(), String> {
    let store = app.state::<DownloadManagerStore>();
    let rux = store.get();
    download_version_json(
        ver_json_url.as_str(),
        minecraft_dir.as_str(),
        version_name.as_str(),
        rux,
    ).await;
    let ver_json = parse_version_json(
        minecraft_dir.as_str(),
        version_name.as_str(),
    ).await.map_err(|e| e.to_string())?;
    println!("{:?}", ver_json);
    Ok(())
}