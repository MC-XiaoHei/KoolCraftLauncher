use anyhow::Result;
use std::env::consts::ARCH;
use std::io::{Read, Write};
use std::option::Option;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::create_dir_all;
use zip::ZipArchive;
use task_manager::task_section::TaskSection;
use crate::version_json_schema::Rule;

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

				let mut buffer = vec![];
				file.read_to_end(&mut buffer)?;
				output_file.write_all(&buffer)?;
			}
		}
	}

	Ok(())
}

pub async fn wait_task_section(section: Arc<TaskSection>) {
	loop {
		if section.get_progress_percent() == 100 {
			return;
		} else {
			tokio::time::sleep(Duration::from_millis(100)).await;
		}
	}
}
