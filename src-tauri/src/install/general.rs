use crate::install::asset_index_json_schema::AssetIndexJson;
use crate::install::manager::download_task::DownloadTask;
use crate::install::manager::task::Task;
use crate::install::manager::task_group::TaskGroup;
use crate::install::manager::task_section::TaskSection;
use crate::install::utils::{unzip_natives, wait_task_section};
use crate::install::version_json_schema::VersionJson;
use crate::TASK_MANAGER;
use anyhow::Result;
use futures::future::try_join3;
use std::sync::Arc;
use tauri::async_runtime::TokioJoinHandle;

#[tauri::command]
pub async fn install_vanilla(
	ver_json_url: String,
	minecraft_dir: String,
	version_name: String,
) -> Result<(), String> {
	_install_vanilla(ver_json_url, minecraft_dir, version_name)
		.await
		.map_err(|e| e.to_string())?;
	Ok(())
}

async fn _install_vanilla(
	ver_json_url: String,
	minecraft_dir: String,
	version_name: String,
) -> Result<()> {
	let task_group = TASK_MANAGER.create_group(TaskGroup::new(format!(
		"install-vanilla@{}@{}",
		version_name, minecraft_dir
	)));
	let semaphore = TASK_MANAGER.get_semaphore();

	let download_version_json_task_section = build_resolve_version_json_task_section(
		ver_json_url.as_str(),
		minecraft_dir.as_str(),
		version_name.as_str(),
	)
	.await?;
	let download_version_json_task_section = task_group
		.submit_section(download_version_json_task_section, semaphore.clone())
		.await;

	wait_task_section(download_version_json_task_section).await;

	let ver_json = parse_version_json(minecraft_dir.as_str(), version_name.as_str()).await?;

	let resolve_client_jar_task_section = build_resolve_client_jar_task_section(
		minecraft_dir.as_str(),
		version_name.as_str(),
		&ver_json,
	)
	.await?;
	let resolve_client_jar_task_section = task_group
		.submit_section(resolve_client_jar_task_section, semaphore.clone())
		.await;

	let resolve_client_jar_handle: TokioJoinHandle<Result<()>> = tokio::spawn(async move {
		wait_task_section(resolve_client_jar_task_section).await;
		Ok(())
	});

	let resolve_libraries_tasks_section =
		build_resolve_libraries_task_section(minecraft_dir.as_str(), &ver_json).await?;
	let resolve_libraries_tasks_section = task_group
		.submit_section(resolve_libraries_tasks_section, semaphore.clone())
		.await;

	let ver_json_clone = ver_json.clone();
	let minecraft_dir_clone = minecraft_dir.clone();
	let unzip_natives_handle: TokioJoinHandle<Result<()>> = tokio::spawn(async move {
		wait_task_section(resolve_libraries_tasks_section).await;
		unzip_all_natives(minecraft_dir_clone.as_str(), &version_name, &ver_json_clone).await?;
		Ok(())
	});

	let ver_json_clone = ver_json.clone();
	let minecraft_dir_clone = minecraft_dir.clone();
	let task_group_clone = task_group.clone();
	let resolve_assets_handle: TokioJoinHandle<Result<()>> = tokio::spawn(async move {
		let build_resolve_assets_index_task_section =
			build_resolve_assets_index_task_section(minecraft_dir_clone.as_str(), &ver_json_clone)
				.await?;
		let build_resolve_assets_index_task_section = task_group_clone
			.submit_section(build_resolve_assets_index_task_section, semaphore.clone())
			.await;
		wait_task_section(build_resolve_assets_index_task_section).await;

		let asset_index_json =
			parse_asset_index_json(minecraft_dir_clone.as_str(), &ver_json_clone.asset_index.id)
				.await?;

		let resolve_assets_task_section =
			build_resolve_assets_task_section(minecraft_dir_clone.as_str(), asset_index_json)
				.await?;
		let resolve_assets_task_section = task_group_clone
			.submit_section(resolve_assets_task_section, semaphore.clone())
			.await;
		wait_task_section(resolve_assets_task_section).await;
		Ok(())
	});

	let (resolve_client_jar_res, unzip_natives_res, resolve_assets_res) = try_join3(
		resolve_client_jar_handle,
		unzip_natives_handle,
		resolve_assets_handle,
	)
	.await?;

	resolve_client_jar_res?;
	unzip_natives_res?;
	resolve_assets_res?;
	
	TASK_MANAGER.remove_group(task_group);
	Ok(())
}

async fn build_resolve_version_json_task_section(
	ver_json_url: &str,
	minecraft_dir: &str,
	version_name: &str,
) -> Result<TaskSection> {
	let task = DownloadTask::new(
		ver_json_url.parse()?,
		format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name),
		None,
	);
	let task = Arc::new(task);
	Ok(TaskSection::new(
		"resolve-version-json".to_string(),
		vec![task],
	))
}

async fn build_resolve_client_jar_task_section(
	minecraft_dir: &str,
	version_name: &str,
	version_json: &VersionJson,
) -> Result<TaskSection> {
	let task = DownloadTask::new(
		version_json.downloads.client.url.parse()?,
		format!("{0}/versions/{1}/{1}.jar", minecraft_dir, version_name),
		Some(version_json.downloads.client.size),
	);
	let task = Arc::new(task);
	Ok(TaskSection::new(
		"resolve-client-jar".to_string(),
		vec![task],
	))
}

async fn build_resolve_assets_index_task_section(
	minecraft_dir: &str,
	version_json: &VersionJson,
) -> Result<TaskSection> {
	let task = DownloadTask::new(
		version_json.asset_index.url.parse()?,
		format!(
			"{0}/assets/indexes/{1}.json",
			minecraft_dir, version_json.asset_index.id
		),
		Some(version_json.asset_index.size),
	);
	let task = Arc::new(task);
	Ok(TaskSection::new(
		"resolve-assets-index".to_string(),
		vec![task],
	))
}

async fn parse_asset_index_json(minecraft_dir: &str, name: &str) -> Result<AssetIndexJson> {
	let path = format!("{0}/assets/indexes/{1}.json", minecraft_dir, name);
	let json = tokio::fs::read(path).await?;
	Ok(serde_json::from_slice(&json)?)
}

async fn build_resolve_assets_task_section(
	minecraft_dir: &str,
	asset_index_json: AssetIndexJson,
) -> Result<TaskSection> {
	let mut tasks: Vec<Arc<dyn Task + Send + Sync>> = vec![];
	for (_key, value) in asset_index_json.objects.iter() {
		let hash = &value.hash;
		let hash_prefix = &hash[0..2];
		let asset_path = format!(
			"{0}/assets/objects/{1}/{2}",
			minecraft_dir, hash_prefix, hash
		);
		let task = DownloadTask::new(
			format!(
				"https://resources.download.minecraft.net/{0}/{1}",
				hash_prefix, hash
			)
			.parse()?,
			asset_path,
			Some(value.size),
		);
		tasks.push(Arc::new(task));
	}
	Ok(TaskSection::new("resolve-assets".to_string(), tasks))
}

async fn build_resolve_libraries_task_section(
	minecraft_dir: &str,
	version_json: &VersionJson,
) -> Result<TaskSection> {
	let mut tasks: Vec<Arc<dyn Task + Send + Sync>> = vec![];
	let libraries = version_json.libraries.iter().filter(|lib| lib.is_needed());
	for lib in libraries {
		if let Some(artifact) = lib.get_artifact() {
			let task = DownloadTask::new(
				artifact.url.parse()?,
				artifact.get_library_path(minecraft_dir),
				Some(artifact.size),
			);
			tasks.push(Arc::new(task));
		}
	}
	Ok(TaskSection::new("resolve-libraries".to_string(), tasks))
}

async fn unzip_all_natives(
	minecraft_dir: &str,
	version_name: &str,
	version_json: &VersionJson,
) -> Result<()> {
	let natives = version_json
		.libraries
		.iter()
		.filter(|lib| lib.is_native() && lib.is_needed());
	for lib in natives {
		if let Some(artifact) = lib.get_artifact() {
			let jar_path = artifact.get_library_path(&minecraft_dir);
			let natives_dir = format!("{0}/bin/natives/{1}", minecraft_dir, version_name);
			unzip_natives(&jar_path, &natives_dir).await?;
		}
	}
	Ok(())
}

pub async fn parse_version_json(minecraft_dir: &str, version_name: &str) -> Result<VersionJson> {
	let ver_json_path = format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name);
	let ver_json = tokio::fs::read(ver_json_path).await?;
	let ver_json: VersionJson = serde_json::from_slice(&ver_json)?;
	Ok(ver_json)
}
