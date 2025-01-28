use crate::download::rux::download_task::{DownloadTask, DownloadTaskStatus};
use anyhow::{Context, Result};
use futures::stream::StreamExt;
use parking_lot::{Mutex, RwLock};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::time::Duration;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Client;
use tokio::fs::create_dir_all;

#[allow(dead_code)]
pub struct DownloadManager {
	download_tasks: Arc<Mutex<Vec<Arc<RwLock<DownloadTask>>>>>,
	max_concurrent_downloads: Arc<RwLock<usize>>,
	download_timeout: Arc<RwLock<Duration>>,
	http_client: Arc<Client>,
	downloading_num: Arc<RwLock<usize>>,
	ticking: Arc<RwLock<bool>>,
	downloaded_per_sec_counter: Arc<RwLock<u64>>,
	downloaded_per_sec: Arc<RwLock<u64>>,
}

#[allow(dead_code)]
impl DownloadManager {
	pub fn new(client: Client) -> Arc<Self> {
		let download_manager = Arc::new(DownloadManager {
			download_tasks: Arc::new(Mutex::new(Vec::new())),
			max_concurrent_downloads: Arc::new(RwLock::new(32)),
			download_timeout: Arc::new(RwLock::new(Duration::from_secs(5))),
			http_client: Arc::new(client),
			downloading_num: Arc::new(RwLock::new(0)),
			ticking: Arc::new(RwLock::new(false)),
			downloaded_per_sec_counter: Arc::new(RwLock::new(0)),
			downloaded_per_sec: Arc::new(RwLock::new(0)),
		});
		download_manager
	}

	pub fn set_max_concurrent_downloads(self: Arc<Self>, max_concurrent_downloads: usize) {
		*self.max_concurrent_downloads.write() = max_concurrent_downloads;
	}

	pub fn set_download_timeout(self: Arc<Self>, download_timeout: Duration) {
		*self.download_timeout.write() = download_timeout;
	}

	pub fn get_downloading_num(self: Arc<Self>) -> usize {
		*self.downloading_num.read()
	}

	pub fn get_downloaded_per_sec(self: Arc<Self>) -> u64 {
		*self.downloaded_per_sec.read()
	}

	pub async fn add_task(self: Arc<Self>, task: DownloadTask) -> Arc<RwLock<DownloadTask>> {
		let shared_task = Arc::new(RwLock::new(task));
		self.download_tasks.lock().push(shared_task.clone());
		self.start_tick_thread().await;
		shared_task
	}

	async fn start_tick_thread(self: Arc<Self>) {
		if *self.ticking.read() {
			return;
		}
		*self.ticking.write() = true;
		tokio::spawn(async move {
			loop {
				self.clone().tick_tasks_per_sec().await;
				tokio::time::sleep(Duration::from_secs(1)).await;
			}
		});
	}

	pub async fn tick_tasks_per_sec(self: Arc<Self>) {
		*self.downloaded_per_sec.write() = *self.downloaded_per_sec_counter.read();
		*self.downloaded_per_sec_counter.write() = 0;
		let mut past_downloading = self.clone().get_downloading_num();
		let max_concurrent_downloads = self.max_concurrent_downloads.read().clone();

		let mut downloading = 0;
		let mut for_removal = vec![];
		let mut for_submission = vec![];

		{
			let mut tasks = self.download_tasks.lock();
			for (index, task) in tasks.iter().enumerate().rev() {
				let status = task.read().status.clone();
				match status {
					DownloadTaskStatus::Downloading(_, _) => {
						downloading += 1;
					}
					DownloadTaskStatus::Pending => {
						if past_downloading < max_concurrent_downloads {
							past_downloading += 1;
							for_submission.push(task.clone());
						}
					}
					DownloadTaskStatus::Finished => {
						for_removal.push(index);
					}
					_ => {}
				}
			}

			for index in for_removal {
				tasks.remove(index);
			}
		}

		*self.downloading_num.write() = downloading;

		for task in for_submission {
			self.clone().submit_task(task).await;
		}
	}

	async fn submit_task(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) {
		tokio::spawn(async move {
			let url = task.clone().read().url.to_string();
			let total_size = self.clone().get_total_size(url.clone()).await;
			let total_size = match total_size {
				Ok(total_size) => Some(total_size),
				Err(_) => None,
			};
			task.clone().write().status = DownloadTaskStatus::Downloading(0, total_size);
			let result = self.start_download_task(task.clone()).await;
			match result {
				Ok(_) => {}
				Err(e) => task.write().status = DownloadTaskStatus::Failed(e.to_string()),
			}
		});
	}

	async fn start_download_task(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) -> Result<()> {
		let DownloadTaskStatus::Downloading(_, total_size) = task.read().status.clone() else {
			anyhow::bail!("Task is not in downloading status")
		};
		let file_path = task.read().save_to.clone();
		let dir_path = std::path::Path::new(&file_path).parent().unwrap();
		create_dir_all(dir_path).await?;
		let file = File::create(file_path)?;
		if let Some(total_size) = total_size {
			file.set_len(total_size)?;
		}
		let mut writer = BufWriter::new(file);

		let url = task.read().url.to_string();
		let resp = self
			.http_client
			.get(&url)
			.send()
			.await
			.map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

		let mut downloaded = 0;
		let mut stream = resp.bytes_stream();
		while let Some(chunk) = stream.next().await {
			let chunk = chunk.context("Failed to read response chunk")?;
			downloaded += chunk.len() as u64;
			*self.downloaded_per_sec_counter.write() += chunk.len() as u64;
			writer
				.write_all(&chunk)
				.context("Failed to write chunk to file")?;
			task.write().status = DownloadTaskStatus::Downloading(downloaded, total_size);
		}

		task.write().status = DownloadTaskStatus::Finished;

		Ok(())
	}

	async fn get_total_size(self: Arc<Self>, url: String) -> Result<u64> {
		let response = self.http_client.head(url).send().await?;
		let total_size = response
			.headers()
			.get(reqwest::header::CONTENT_LENGTH)
			.ok_or_else(|| anyhow::anyhow!("Failed to get content length"))?
			.to_str()?
			.parse::<u64>()?;

		Ok(total_size)
	}
}
