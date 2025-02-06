#![allow(unused)]

use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::task_manager::task::Task;

pub struct TaskSection {
	name: String,
	tasks: Arc<Vec<Arc<dyn Task + Send + Sync>>>,
}

impl TaskSection {
	pub fn new(name: String, tasks: Vec<Arc<dyn Task + Send + Sync>>) -> Self {
		TaskSection {
			name,
			tasks: Arc::new(tasks),
		}
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn get_running_task_num(&self) -> usize {
		self.tasks
			.iter()
			.filter(|task| task.get_progress_percent() == 100)
			.count()
	}

	pub fn get_section_info(&self) -> TaskSectionInfo {
		TaskSectionInfo {
			name: self.name.clone(),
			progress_percent: self.get_progress_percent(),
		}
	}

	pub fn get_progress_percent(&self) -> u8 {
		let mut total_progress_number: usize = 0;
		for task in self.tasks.iter() {
			total_progress_number += task.get_progress_percent() as usize;
		}
		(total_progress_number / self.tasks.len()) as u8
	}

	pub async fn submit(&self, semaphore: Arc<Semaphore>) {
		let tasks = self.tasks.clone();
		for i in 0..tasks.len() {
			let task = tasks[i].clone();
			let semaphore = semaphore.clone();
			tokio::spawn(async move {
				let permit = semaphore.acquire_owned().await.unwrap();
				let result = task.run().await;
				drop(permit);
				result
			});
		}
	}

	pub async fn cancel_all(&self) -> anyhow::Result<()> {
		for task in self.tasks.iter() {
			task.cancel().await?
		}
		Ok(())
	}
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSectionInfo {
	pub name: String,
	pub progress_percent: u8,
}
