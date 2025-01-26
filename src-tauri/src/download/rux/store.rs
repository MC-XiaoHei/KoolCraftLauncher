use crate::download::rux::download_manager::DownloadManager;
use std::sync::Arc;

pub struct DownloadManagerStore {
	instance: Arc<DownloadManager>,
}

impl DownloadManagerStore {
	pub fn new(instance: Arc<DownloadManager>) -> Self {
		Self { instance }
	}

	pub fn get(&self) -> Arc<DownloadManager> {
		self.instance.clone()
	}
}
