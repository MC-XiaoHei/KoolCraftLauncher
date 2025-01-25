use crate::download::rux::download_manager::DownloadManager;
use crate::download::rux::download_task::DownloadTask;
use crate::download::rux::store::DownloadManagerStore;
use crate::download::utils::{
	download_version_json, get_artifact, get_library_path, is_native_library, is_need,
	parse_version_json, unzip_natives, wait_all_tasks, wait_task,
};
use crate::download::version_schema::VersionJson;
use anyhow::Result;
use std::sync::Arc;
use tauri::async_runtime::TokioJoinHandle;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use tokio::try_join;

#[tauri::command]
pub async fn install_vanilla(
	ver_json_url: String,
	minecraft_dir: String,
	version_name: String,
	app: AppHandle,
) -> Result<(), String> {
	_install_vanilla(ver_json_url, minecraft_dir, version_name, app)
		.await
		.map_err(|e| e.to_string())?;
	Ok(())
}

async fn _install_vanilla(
	ver_json_url: String,
	minecraft_dir: String,
	version_name: String,
	app: AppHandle,
) -> Result<()> {
	let store = app.state::<DownloadManagerStore>();
	let rux = store.get();
	let download_version_json_task = download_version_json(
		ver_json_url.as_str(),
		minecraft_dir.as_str(),
		version_name.as_str(),
		rux.clone(),
	)
	.await?;
	wait_task(download_version_json_task).await;

	let ver_json = parse_version_json(minecraft_dir.as_str(), version_name.as_str()).await?;

	let _resolve_client_jar_task = submit_resolve_client_jar(
		minecraft_dir.as_str(),
		version_name.as_str(),
		&ver_json,
		rux.clone(),
	)
	.await?;

	let resolve_libraries_tasks =
		submit_resolve_libraries(minecraft_dir.as_str(), &ver_json, rux.clone()).await?;

	let ver_json_clone = ver_json.clone();
	let minecraft_dir_clone = minecraft_dir.clone();
	let unzip_natives_handle: TokioJoinHandle<Result<()>> = tokio::spawn(async move {
		wait_all_tasks(resolve_libraries_tasks).await;
		unzip_all_natives(minecraft_dir_clone.as_str(), &version_name, &ver_json_clone).await?;
		Ok(())
	});

	let ver_json_clone = ver_json.clone();
	let minecraft_dir_clone = minecraft_dir.clone();
	let resolve_assets_handle: TokioJoinHandle<Result<()>> = tokio::spawn(async move {
		wait_task(
			submit_resolve_assets_index(minecraft_dir_clone.as_str(), &ver_json_clone, rux.clone())
				.await?,
		)
		.await;

		Ok(())
	});

	let (unzip_natives_res, resolve_assets_res) =
		try_join!(unzip_natives_handle, resolve_assets_handle)?;
	
	unzip_natives_res?;
	resolve_assets_res?;

	Ok(())
}

async fn submit_resolve_client_jar(
	minecraft_dir: &str,
	version_name: &str,
	version_json: &VersionJson,
	rux: Arc<DownloadManager>,
) -> Result<Arc<RwLock<DownloadTask>>> {
	let task = DownloadTask::new(version_json.downloads.client.url.parse()?)
		.save_to(format!("{0}/versions/{1}/{1}.jar", minecraft_dir, version_name).as_str());
	let shared_task = Arc::new(RwLock::new(task));
	rux.add_task(shared_task.clone()).await;
	Ok(shared_task)
}

async fn submit_resolve_assets_index(
	minecraft_dir: &str,
	version_json: &VersionJson,
	rux: Arc<DownloadManager>,
) -> Result<Arc<RwLock<DownloadTask>>> {
	let task = DownloadTask::new(version_json.asset_index.url.parse()?)
		.save_to(format!("{0}/assets/indexes/{1}.json", minecraft_dir, version_json.asset_index.id).as_str());
	let shared_task = Arc::new(RwLock::new(task));
	rux.add_task(shared_task.clone()).await;
	Ok(shared_task)
}

async fn submit_resolve_libraries(
	minecraft_dir: &str,
	version_json: &VersionJson,
	rux: Arc<DownloadManager>,
) -> Result<Vec<Arc<RwLock<DownloadTask>>>> {
	let mut tasks: Vec<Arc<RwLock<DownloadTask>>> = Vec::new();
	let libraries = version_json.libraries.iter().filter(|lib| is_need(lib));
	for lib in libraries {
		if let Some(artifact) = get_artifact(lib) {
			let task = DownloadTask::new(artifact.url.parse()?)
				.save_to(get_library_path(minecraft_dir, &artifact).as_str());
			let shared_task = Arc::new(RwLock::new(task));
			tasks.push(shared_task.clone());
			rux.clone().add_task(shared_task).await;
		}
	}
	Ok(tasks)
}

async fn unzip_all_natives(
	minecraft_dir: &str,
	version_name: &str,
	version_json: &VersionJson,
) -> Result<()> {
	let natives = version_json
		.libraries
		.iter()
		.filter(|lib| is_native_library(lib) && is_need(lib));
	for lib in natives {
		if let Some(artifact) = get_artifact(lib) {
			let jar_path = get_library_path(&minecraft_dir, &artifact);
			let natives_dir = format!("{0}/bin/natives/{1}", minecraft_dir, version_name);
			unzip_natives(&jar_path, &natives_dir).await?;
		}
	}
	Ok(())
}
