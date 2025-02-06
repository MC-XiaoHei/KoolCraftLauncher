use chrono::Utc;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use uuid::Uuid;
use crate::task_manager::task_section::{TaskSection, TaskSectionInfo};

pub struct TaskGroup {
	id: String,
	start_time: String,
	name: String,
	sections: Arc<Mutex<Vec<Arc<TaskSection>>>>,
	canceling: AtomicBool,
}

impl TaskGroup {
	pub fn new(name: String) -> Self {
		TaskGroup {
			id: Uuid::now_v7().to_string(),
			name,
			start_time: Utc::now().to_rfc3339(),
			sections: Arc::new(Mutex::new(vec![])),
			canceling: AtomicBool::new(false),
		}
	}

	pub fn get_id(&self) -> String {
		self.id.clone()
	}

	pub fn get_progress(&self) -> u8 {
		let sections = self.sections.lock();
		let mut total_progress_number: usize = 0;
		for section in sections.iter() {
			total_progress_number += section.get_progress_percent() as usize;
		}
		(total_progress_number / sections.len()) as u8
	}

	pub fn get_group_info(&self) -> TaskGroupInfo {
		let sections = self.sections.lock();
		let sections_data = sections
			.iter()
			.map(|section| section.get_section_info())
			.collect::<Vec<_>>();

		TaskGroupInfo {
			start_time: self.start_time.clone(),
			name: self.name.clone(),
			sections: sections_data,
		}
	}

	pub async fn submit_section(
		&self,
		section: TaskSection,
		semaphore: Arc<Semaphore>,
	) -> Arc<TaskSection> {
		let section = Arc::new(section);
		if !self.canceling.load(Ordering::SeqCst) {
			self.sections.lock().push(section.clone());
		}
		section.submit(semaphore).await;
		section
	}

	pub async fn cancel(&self) -> anyhow::Result<()> {
		self.canceling.store(true, Ordering::SeqCst);
		let sections = self.sections.lock();
		for section in sections.iter() {
			section.cancel_all().await?
		}
		Ok(())
	}
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskGroupInfo {
	pub start_time: String,
	pub name: String,
	pub sections: Vec<TaskSectionInfo>,
}
