use crate::install::manager::task_section::{TaskSection, TaskSectionInfo};
use chrono::Utc;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct TaskGroup {
	start_time: String,
	name: String,
	sections: Arc<Mutex<Vec<Arc<TaskSection>>>>,
	canceling: AtomicBool,
}

impl TaskGroup {
	pub fn new(name: String) -> Self {
		TaskGroup {
			name,
			start_time: Utc::now().to_rfc3339(),
			sections: Arc::new(Mutex::new(vec![])),
			canceling: AtomicBool::new(false),
		}
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
