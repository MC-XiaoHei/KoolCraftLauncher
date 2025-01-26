use crate::download::rux::download_task::{DownloadTask, DownloadTaskStatus};
use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use std::fs::File;
use std::io::{Seek, Write};
use std::sync::Arc;
use std::time::Duration;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Client;
use tokio::fs::create_dir_all;
use tokio::sync::{Mutex, RwLock};

#[allow(dead_code)]
pub struct DownloadManager {
	pub download_tasks: Arc<Mutex<Vec<Arc<RwLock<DownloadTask>>>>>,
	pub max_concurrent_downloads: Arc<RwLock<usize>>,
	pub download_timeout: Arc<RwLock<Duration>>,
	http_client: Arc<Client>,
}

#[allow(dead_code)]
impl DownloadManager {
	pub fn new(client: Client) -> Arc<Self> {
		let download_manager = Arc::new(DownloadManager {
			download_tasks: Arc::new(Mutex::new(Vec::new())),
			max_concurrent_downloads: Arc::new(RwLock::new(32)),
			download_timeout: Arc::new(RwLock::new(Duration::from_secs(5))),
			http_client: Arc::new(client),
		});
		download_manager
	}

	pub async fn set_max_concurrent_downloads(self: Arc<Self>, max_concurrent_downloads: usize) {
		*self.max_concurrent_downloads.write().await = max_concurrent_downloads;
	}

	pub async fn set_download_timeout(self: Arc<Self>, download_timeout: Duration) {
		*self.download_timeout.write().await = download_timeout;
	}

	pub async fn get_downloading_num(self: Arc<Self>) -> usize {
		let tasks = self.download_tasks.lock().await.clone();
		stream::iter(tasks)
			.filter_map(|task| async move {
				let task_read = task.read().await;
				if task_read.status == DownloadTaskStatus::Downloading {
					Some(task_read.clone())
				} else {
					None
				}
			})
			.count()
			.await
	}

	pub async fn add_task(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) {
		self.download_tasks.lock().await.push(task);
	}
	pub async fn tick_tasks(self: Arc<Self>) {
		let mut tasks = self.download_tasks.lock().await.clone();
		let mut downloading = self.clone().get_downloading_num().await;
		let max_concurrent_downloads = self.max_concurrent_downloads.read().await.clone();

		let mut for_removal = vec![];

		for (index, task) in tasks.iter().enumerate().rev() {
			let status = task.read().await.status.clone();
			match status {
				DownloadTaskStatus::Pending => {
					if downloading < max_concurrent_downloads {
						task.write().await.status = DownloadTaskStatus::Downloading;
						downloading += 1;
						self.clone().submit_task(task.clone()).await;
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

		log::info!("Download tasks: {}", tasks.len());

		if tasks.len() < 10 {
			for x in tasks.iter() {
				log::info!(
					"Task: {}, progress: {}, total: {}",
					x.read().await.file_name,
					x.read().await.downloaded,
					x.read().await.size.unwrap_or(0)
				);
			}
		}
	}

	async fn submit_task(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) {
		tokio::spawn(async move {
			let total_size = self
				.clone()
				.get_total_size(task.clone())
				.await
				.unwrap_or_else(|_| 0);
			task.write().await.size = Some(total_size);

			if let Err(err) = self.start_download_task(task.clone()).await {
				task.write().await.status = DownloadTaskStatus::Failed(err.to_string());
			}
		});
	}

	async fn start_download_task(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) -> Result<()> {
		if let None = task.read().await.size {
			return Err(anyhow::anyhow!("Failed to get content length"));
		}

		let total_size = task.read().await.size.unwrap_or(0);
		let file_path = task.read().await.save_to.clone();
		let dir_path = std::path::Path::new(&file_path).parent().unwrap();
		create_dir_all(dir_path).await?;
		let file = Arc::new(Mutex::new(File::create(file_path)?));
		file.lock().await.set_len(total_size)?;

		let url = task.read().await.url.to_string();
		let resp = self
			.http_client
			.get(&url)
			.send()
			.await
			.map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

		let mut file = file.lock().await;

		let mut stream = resp.bytes_stream();
		while let Some(chunk) = stream.next().await {
			let chunk = chunk.context("Failed to read response chunk")?;
			task.write().await.downloaded += chunk.len() as u64;
			file.write_all(&chunk)
				.context("Failed to write chunk to file")?;
		}
		
		task.write().await.status = DownloadTaskStatus::Finished;
		log::info!("Download finished: {}", task.read().await.file_name);

		Ok(())
	}

	async fn get_total_size(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) -> Result<u64> {
		let response = self
			.http_client
			.head(task.read().await.url.clone())
			.send()
			.await?;
		let total_size = response
			.headers()
			.get(reqwest::header::CONTENT_LENGTH)
			.ok_or_else(|| anyhow::anyhow!("Failed to get content length"))?
			.to_str()?
			.parse::<u64>()?;

		Ok(total_size)
	}
}
