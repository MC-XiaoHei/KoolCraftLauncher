use crate::download::rux::download_manager::DownloadManager;
use crate::download::rux::download_task::DownloadTask;
use crate::download::rux::download_task::DownloadTaskStatus::Finished;
use crate::download::version_schema::{Library, LibraryArtifact, Rule, VersionJson};
use anyhow::Result;
use parking_lot::RwLock;
use std::env::consts::ARCH;
use std::io::{Read, Write};
use std::option::Option;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::create_dir_all;
use zip::ZipArchive;

pub async fn submit_resolve_version_json(
	ver_json_url: &str,
	minecraft_dir: &str,
	version_name: &str,
	download_group: &str,
	rux: Arc<DownloadManager>,
) -> Result<Arc<RwLock<DownloadTask>>> {
	let task = DownloadTask::new(
		ver_json_url.parse()?,
		download_group.to_string(),
		"download-version-json".to_string(),
	)
	.save_to(format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name).as_str());
	Ok(rux.add_task(task).await)
}

pub async fn parse_version_json(minecraft_dir: &str, version_name: &str) -> Result<VersionJson> {
	let ver_json_path = format!("{0}/versions/{1}/{1}.json", minecraft_dir, version_name);
	let ver_json = tokio::fs::read(ver_json_path).await?;
	let ver_json: VersionJson = serde_json::from_slice(&ver_json)?;
	Ok(ver_json)
}

pub fn is_rule_allowed(rule: &Option<Vec<Rule>>) -> bool {
	let mut default_is_allowed = true;
	let mut is_allowed: Option<bool> = None;
	if let Some(rules) = rule {
		for rule in rules {
			if let Some(os) = &rule.os {
				let os_name = if cfg!(target_os = "windows") {
					"windows"
				} else if cfg!(target_os = "macos") {
					"osx"
				} else if cfg!(target_os = "linux") {
					"linux"
				} else {
					""
				};
				if os_name == os.name && os.arch.as_ref().map_or(true, |arch| arch == ARCH) {
					is_allowed = Some(rule.action == "allow");
				} else {
					default_is_allowed = rule.action != "allow";
				}
			} else {
				default_is_allowed = rule.action == "allow";
			}
		}
		return is_allowed.unwrap_or(default_is_allowed);
	}
	true
}

pub fn is_native_library(library: &Library) -> bool {
	library.natives.is_some()
		|| (library.downloads.classifiers.is_none()
			&& library.rules.is_some()
			&& library.name.contains("natives"))
}

pub async fn unzip_natives(jar_path: &str, natives_dir: &str) -> Result<()> {
	let natives_path = Path::new(natives_dir);

	if !natives_path.exists() {
		create_dir_all(natives_dir).await?;
	}

	let jar = std::fs::File::open(jar_path)?;
	let mut archive = ZipArchive::new(jar)?;

	for i in 0..archive.len() {
		let mut file = archive.by_index(i)?;
		let path = Path::new(file.name());

		if let Some(ext) = path.extension() {
			if ["dll", "so", "dylib"].contains(&ext.to_str().unwrap_or_default()) {
				let output_file_path = natives_path.join(path.file_name().unwrap());
				let mut output_file = std::fs::File::create(output_file_path)?;

				let mut buffer = Vec::new();
				file.read_to_end(&mut buffer)?;
				output_file.write_all(&buffer)?;
			}
		}
	}

	Ok(())
}

pub fn is_need(library: &Library) -> bool {
	if is_native_library(library) && library.natives.is_none() {
		let native_pattern = if cfg!(target_os = "windows") {
			match ARCH {
				"x86" => vec!["natives-windows-x86"],
				"x86_64" => vec!["natives-windows", "natives-windows-x86_64"],
				"aarch64" => vec!["natives-windows-arm64"],
				_ => vec![],
			}
		} else if cfg!(target_os = "macos") {
			match ARCH {
				"x86" => vec!["natives-macos-patch", "natives-macos", "natives-macos-x86"],
				"x86_64" => vec![
					"natives-macos-patch",
					"natives-macos",
					"natives-macos-x86_64",
				],
				"aarch64" => vec![
					"natives-macos-patch",
					"natives-macos",
					"natives-macos-arm64",
				],
				_ => vec![],
			}
		} else if cfg!(target_os = "linux") {
			match ARCH {
				"x86_64" => vec!["natives-linux", "linux-x86_64"],
				"aarch64" => vec!["natives-linux", "linux-aarch64"],
				_ => vec![],
			}
		} else {
			vec![]
		};
		let mut is_allowed = native_pattern.len() == 0;
		for pattern in native_pattern {
			if library.name.ends_with(pattern) {
				is_allowed = true;
				break;
			}
		}
		return is_allowed && is_rule_allowed(&library.rules);
	}
	is_rule_allowed(&library.rules)
}

pub fn get_library_path(minecraft_dir: &str, artifact: &LibraryArtifact) -> String {
	format!("{}/libraries/{}", minecraft_dir, artifact.path)
}

pub fn get_artifact(library: &Library) -> Option<&LibraryArtifact> {
	if let Some(natives) = &library.natives {
		let native_key = if cfg!(target_os = "windows") {
			natives.windows.as_ref()
		} else if cfg!(target_os = "macos") {
			natives.osx.as_ref()
		} else if cfg!(target_os = "linux") {
			natives.linux.as_ref()
		} else {
			None
		};
		if let Some(native_key) = native_key {
			library
				.downloads
				.classifiers
				.as_ref()
				.and_then(|classifiers| classifiers.get(native_key))
		} else {
			None
		}
	} else {
		library.downloads.artifact.as_ref()
	}
}

pub async fn wait_all_tasks(tasks: &Vec<Arc<RwLock<DownloadTask>>>) {
	for task in tasks.iter() {
		wait_task(task.clone()).await;
	}
}

pub async fn wait_task(task: Arc<RwLock<DownloadTask>>) {
	loop {
		if task.read().status == Finished {
			break;
		} else {
			tokio::time::sleep(Duration::from_millis(100)).await;
		}
	}
}
