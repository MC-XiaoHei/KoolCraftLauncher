use std::sync::Arc;
use tokio::sync::RwLock;
use crate::download::rux::download_manager::DownloadManager;
use crate::download::rux::download_task::DownloadTask;

pub async fn download_version_json(
    ver_json_url: &str,
    minecraft_dir: &str,
    version_name: &str,
    rux: Arc<DownloadManager>
) {
    let task = DownloadTask::new(
        ver_json_url.parse().unwrap(),
    ).save_to(format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name).as_str());
    let shared_task = Arc::new(RwLock::new(task));
    rux.add_task(shared_task.clone()).await;
}