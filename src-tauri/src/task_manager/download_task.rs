use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use anyhow::Context;
use futures::StreamExt;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Url;
use tokio::fs::{create_dir_all, remove_file, File};
use tokio::io::AsyncWriteExt;
use crate::utils::network::HTTP_CLIENT;
use crate::task_manager::task::Task;

pub struct DownloadTask {
	size: Option<u64>,
	url: Url,
	save_to: String,
	progress_percent: Arc<AtomicU8>,
	running: AtomicBool,
}

impl DownloadTask {
	pub fn new(url: Url, save_to: String, size: Option<u64>) -> Self {
		DownloadTask {
			url,
			save_to,
			size,
			progress_percent: Arc::new(AtomicU8::new(0)),
			running: AtomicBool::new(false),
		}
	}

	async fn get_total_size(&self) -> anyhow::Result<u64> {
		let response = HTTP_CLIENT.head(self.url.clone()).send().await?;
		let total_size = response
			.headers()
			.get(reqwest::header::CONTENT_LENGTH)
			.ok_or_else(|| anyhow::anyhow!(""))?
			.to_str()?
			.parse::<u64>()?;

		Ok(total_size)
	}
}

#[async_trait]
impl Task for DownloadTask {
	fn get_progress_percent(&self) -> u8 {
		self.progress_percent.load(Ordering::SeqCst)
	}

	async fn run(&self) -> anyhow::Result<()> {
		self.running.store(true, Ordering::SeqCst);
		if let Some(parent) = std::path::Path::new(&self.save_to).parent() {
			create_dir_all(parent).await?;
		}
		let mut file = File::create(&self.save_to).await?;
		if let Some(total_size) = self.size {
			file.set_len(total_size).await?;
		} else if let Ok(size) = self.get_total_size().await {
			file.set_len(size).await?;
		}

		let url = self.url.to_string();
		let response = HTTP_CLIENT
			.get(&url)
			.send()
			.await
			.map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

		let mut downloaded = 0;
		let mut stream = response.bytes_stream();
		while let Some(chunk) = stream.next().await {
			if !self.running.load(Ordering::SeqCst) {
				drop(file);
				remove_file(&self.save_to).await?;
				return Ok(());
			}
			let chunk = chunk.context("Failed to read response chunk")?;
			downloaded += chunk.len() as u64;
			file.write_all(&chunk)
				.await
				.context("Failed to write chunk to file")?;
			let progress_percent = if let Some(total) = self.size {
				downloaded * 100 / total
			} else {
				50
			} as u8;
			if progress_percent == 100 {
				break;
			}
			self.progress_percent
				.store(progress_percent, Ordering::SeqCst);
		}

		self.progress_percent.store(100, Ordering::SeqCst);
		Ok(())
	}

	async fn cancel(&self) -> anyhow::Result<()> {
		self.running.store(false, Ordering::SeqCst);
		Ok(())
	}
}
