use tauri_plugin_http::reqwest::Url;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DownloadTaskStatus {
	Pending,
	Downloading,
	Paused,
	Failed(String),
	Finished,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DownloadTask {
	pub url: Url,
	pub file_name: String,
	pub save_to: String,
	pub size: Option<u64>,
	pub downloaded: u64,
	pub status: DownloadTaskStatus,
}

impl DownloadTask {
	pub fn new(url: Url) -> Self {
		let file_name = get_filename_from_url(url.as_str()).unwrap();
		DownloadTask {
			url,
			file_name: file_name.clone(),
			save_to: file_name,
			size: None,
			downloaded: 0,
			status: DownloadTaskStatus::Pending,
		}
	}

	pub fn save_to(mut self, path: &str) -> Self {
		self.save_to = path.to_string();
		self
	}
}

fn get_filename_from_url(url: &str) -> Option<String> {
	let parsed_url = Url::parse(url).ok()?;
	parsed_url
		.path_segments()
		.and_then(|segments| segments.last())
		.map(|segment| segment.to_string())
}
