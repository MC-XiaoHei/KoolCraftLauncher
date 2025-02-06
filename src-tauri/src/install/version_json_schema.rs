#![allow(dead_code)] // TODO: remove this

use serde::Deserialize;
use std::collections::HashMap;
use std::env::consts::ARCH;
use crate::utils::is_rule_allowed;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VersionJson {
	pub id: String,
	pub asset_index: AssetIndex,
	pub downloads: Downloads,
	pub libraries: Vec<Library>,
	pub logging: Option<Logging>,
	pub main_class: String,
	pub minecraft_arguments: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Library {
	pub downloads: LibraryDownloads,
	pub name: String,
	pub natives: Option<LibraryNatives>,
	pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
	pub action: String,
	pub features: Option<HashMap<String, bool>>,
	pub os: Option<Os>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Os {
	pub name: String,
	pub version: Option<String>,
	pub arch: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryNatives {
	pub linux: Option<String>,
	pub osx: Option<String>,
	pub windows: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryDownloads {
	pub artifact: Option<LibraryArtifact>,
	pub classifiers: Option<HashMap<String, LibraryArtifact>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryArtifact {
	pub path: String,
	pub sha1: String,
	pub size: u64,
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Downloads {
	pub client: DownloadsElement,
	pub server: Option<DownloadsElement>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DownloadsElement {
	pub url: String,
	pub sha1: String,
	pub size: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Logging {
	pub client: LoggingClient,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingClient {
	pub argument: String,
	pub file: LoggingClientFile,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingClientFile {
	pub id: String,
	pub sha1: String,
	pub size: u64,
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
	pub id: String,
	pub sha1: String,
	pub size: u64,
	pub total_size: u64,
	pub url: String,
}

impl Library {
	pub fn is_native(&self) -> bool {
		self.natives.is_some()
			|| (self.downloads.classifiers.is_none()
				&& self.rules.is_some()
				&& self.name.contains("natives"))
	}

	pub fn is_needed(&self) -> bool {
		if self.is_native() && self.natives.is_none() {
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
			let mut is_allowed = native_pattern.is_empty();
			for pattern in native_pattern {
				if self.name.ends_with(pattern) {
					is_allowed = true;
					break;
				}
			}
			return is_allowed && is_rule_allowed(&self.rules);
		}
		is_rule_allowed(&self.rules)
	}

	pub fn get_artifact(&self) -> Option<&LibraryArtifact> {
		if let Some(natives) = &self.natives {
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
				self.downloads
					.classifiers
					.as_ref()
					.and_then(|classifiers| classifiers.get(native_key))
			} else {
				None
			}
		} else {
			self.downloads.artifact.as_ref()
		}
	}
}

impl LibraryArtifact {
	pub fn get_library_path(&self, minecraft_dir: &str) -> String {
		format!("{}/libraries/{}", minecraft_dir, self.path)
	}
}
