use tauri_plugin_http::reqwest::Url;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum DownloadTaskStatus {
	Pending(Option<u64>),
	Downloading(u64, Option<u64>),
	Failed(String),
	Finished,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DownloadTask {
	pub download_group: String,
	pub task_type: String,
	pub url: Url,
	pub file_name: String,
	pub save_to: String,
	pub status: DownloadTaskStatus,
}

impl DownloadTask {
	pub fn new(url: Url, download_group: String, task_type: String, size: Option<u64>) -> Self {
		let file_name = get_file_name_from_url(url.as_str()).unwrap();
		DownloadTask {
			download_group,
			task_type,
			url,
			file_name: file_name.clone(),
			save_to: file_name,
			status: DownloadTaskStatus::Pending(size),
		}
	}

	pub fn save_to(mut self, path: &str) -> Self {
		self.save_to = path.to_string();
		self
	}

	pub fn get_size(&self) -> Option<u64> {
		match self.status {
			DownloadTaskStatus::Pending(size) => size,
			DownloadTaskStatus::Downloading(_, size) => size,
			_ => None,
		}
	}
}

fn get_file_name_from_url(url: &str) -> Option<String> {
	let parsed_url = Url::parse(url).ok()?;
	parsed_url
		.path_segments()
		.and_then(|segments| segments.last())
		.map(|segment| segment.to_string())
}
