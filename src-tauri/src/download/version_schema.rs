#![allow(dead_code)] // TODO: remove this

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub natives: Option<LibraryNatives>,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub action: String,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<Os>,
}

#[derive(Deserialize, Debug)]
pub struct Os {
    pub name: String,
    pub version: Option<String>,
    pub arch: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryNatives {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryArtifact>,
    pub classifiers: Option<HashMap<String, LibraryArtifact>>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
    pub client: DownloadsElement,
    pub server: Option<DownloadsElement>
}

#[derive(Deserialize, Debug)]
pub struct DownloadsElement {
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Deserialize, Debug)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingClientFile,
}

#[derive(Deserialize, Debug)]
pub struct LoggingClientFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: String,
}
