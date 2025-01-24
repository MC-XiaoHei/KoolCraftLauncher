use crate::download::rux::download_manager::DownloadManager;
use crate::download::rux::download_task::DownloadTask;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use crate::download::version_schema::VersionJson;

pub async fn download_version_json(
    ver_json_url: &str,
    minecraft_dir: &str,
    version_name: &str,
    rux: Arc<DownloadManager>,
) {
    let task = DownloadTask::new(
        ver_json_url.parse().unwrap(),
    ).save_to(format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name).as_str());
    let shared_task = Arc::new(RwLock::new(task));
    rux.add_task(shared_task.clone()).await;
}

pub async fn parse_version_json(
    minecraft_dir: &str,
    version_name: &str,
) -> Result<VersionJson> {
    let ver_json_path = format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name);
    let ver_json = tokio::fs::read(ver_json_path).await?;
    let ver_json: VersionJson = serde_json::from_slice(&ver_json)?;
    Ok(ver_json)
}