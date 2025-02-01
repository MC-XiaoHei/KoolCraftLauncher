use crate::download::rux::download_task::{DownloadTask, DownloadTaskStatus};
use anyhow::{Context, Result};
use futures::stream::StreamExt;
use parking_lot::{Mutex, RwLock};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::time::Duration;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Client;
use tokio::fs::create_dir_all;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadGroup {
	pub download_speed: u64,
	pub total_size: Option<u64>,
	pub total_downloaded: u64,
}

#[allow(dead_code)]
pub struct DownloadManager {
	download_tasks: Arc<Mutex<Vec<Arc<RwLock<DownloadTask>>>>>,
	max_concurrent_downloads: Arc<RwLock<usize>>,
	download_timeout: Arc<RwLock<Duration>>,
	http_client: Arc<Client>,
	downloading_num: Arc<RwLock<usize>>,
	ticking: Arc<RwLock<bool>>,
	downloaded_per_sec_counter: Arc<RwLock<HashMap<String, u64>>>,
	download_groups: Arc<RwLock<HashMap<String, Arc<RwLock<DownloadGroup>>>>>,
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
			downloaded_per_sec_counter: Arc::new(RwLock::new(HashMap::new())),
			download_groups: Arc::new(RwLock::new(HashMap::new())),
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

	pub fn get_download_groups(self: Arc<Self>) -> HashMap<String, DownloadGroup> {
		self.download_groups
			.read()
			.iter()
			.map(|(k, v)| {
				(
					k.clone(),
					DownloadGroup {
						download_speed: v.read().download_speed,
						total_size: v.read().total_size,
						total_downloaded: v.read().total_downloaded,
					},
				)
			})
			.collect()
	}

	pub async fn add_task(self: Arc<Self>, task: DownloadTask) -> Arc<RwLock<DownloadTask>> {
		let task_size = task.get_size();
		let download_group = task.download_group.clone();
		let shared_task = Arc::new(RwLock::new(task));
		{
			let mut groups = self.download_groups.write();
			if !groups.contains_key(&download_group) {
				groups.insert(
					download_group.clone(),
					Arc::new(RwLock::new(DownloadGroup {
						download_speed: 0,
						total_size: task_size,
						total_downloaded: 0,
					})),
				);
			} else {
				let mut group = groups.get_mut(&download_group).unwrap().write();
				if let Some(total_size) = group.total_size {
					if let Some(size) = task_size {
						group.total_size = Some(total_size + size);
					}
				} else {
					group.total_size = task_size;
				}
			}
		}
		self.download_tasks.lock().push(shared_task.clone());
		self.start_tick_thread();
		shared_task
	}

	fn start_tick_thread(self: Arc<Self>) {
		if *self.ticking.read() {
			return;
		}
		*self.ticking.write() = true;
		tokio::spawn(async move {
			loop {
				self.clone().tick_tasks_per_100_ms();
				tokio::time::sleep(Duration::from_millis(100)).await;
			}
		});
	}

	pub fn tick_tasks_per_100_ms(self: Arc<Self>) {
		for (group, counter) in self.downloaded_per_sec_counter.read().iter() {
			let mut download_groups = self.download_groups.write();
			let mut download_group = download_groups.get_mut(group).unwrap().write();
			download_group.download_speed = *counter * 10;
			download_group.total_downloaded = download_group.total_downloaded + *counter;
		}
		self.downloaded_per_sec_counter.write().clear();
		let mut past_downloading = self.clone().get_downloading_num();
		let max_concurrent_downloads = self.max_concurrent_downloads.read().clone();

		let mut downloading = 0;
		let mut for_removal = vec![];
		let mut for_submission = vec![];
		let mut groups = vec![];

		{
			let mut tasks = self.download_tasks.lock();
			for (index, task) in tasks.iter().enumerate().rev() {
				let download_group = task.read().download_group.clone();
				if !groups.contains(&download_group) {
					groups.push(download_group.clone());
				}
				let status = task.read().status.clone();
				match status {
					DownloadTaskStatus::Downloading(_, _) => {
						downloading += 1;
					}
					DownloadTaskStatus::Pending(_) => {
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

		self.downloaded_per_sec_counter
			.write()
			.retain(|k, _| groups.contains(k));
		self.download_groups
			.write()
			.retain(|k, _| groups.contains(k));

		*self.downloading_num.write() = downloading;

		for task in for_submission {
			self.clone().submit_task(task);
		}
	}

	fn submit_task(self: Arc<Self>, task: Arc<RwLock<DownloadTask>>) {
		tokio::spawn(async move {
			let url = task.clone().read().url.to_string();
			let task_size = task.clone().read().get_size().clone();
			let total_size = if let Some(size) = task_size {
				Some(size)
			} else {
				match self.clone().get_total_size(url.clone()).await {
					Ok(total_size) => {
						let mut groups = self.download_groups.write();
						let mut group =
							groups.get_mut(&task.read().download_group).unwrap().write();
						let group_total_size = group.clone().total_size.unwrap_or(0);
						group.total_size = Some(total_size + group_total_size);
						Some(total_size)
					}
					Err(_) => None,
				}
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
		let download_group = task.read().download_group.clone();
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
			*self
				.downloaded_per_sec_counter
				.write()
				.entry(download_group.clone())
				.or_insert(0) += chunk.len() as u64;
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
