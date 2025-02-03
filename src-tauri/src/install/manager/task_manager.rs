use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use parking_lot::Mutex;
use tokio::sync::Semaphore;
use crate::install::manager::task_group::{TaskGroup, TaskGroupInfo};

pub struct TaskManager {
	max_concurrency: AtomicUsize,
	semaphore: Arc<Semaphore>,
	groups: Arc<Mutex<Vec<Arc<TaskGroup>>>>,
}

const DEFAULT_MAX_CONCURRENCY: usize = 32;

impl TaskManager {
	pub fn new() -> Self {
		TaskManager {
			max_concurrency: AtomicUsize::new(DEFAULT_MAX_CONCURRENCY),
			semaphore: Arc::new(Semaphore::new(DEFAULT_MAX_CONCURRENCY)),
			groups: Arc::new(Mutex::new(vec![])),
		}
	}
	
	pub fn get_semaphore(&self) -> Arc<Semaphore> {
		self.semaphore.clone()
	}

	pub fn set_max_concurrency(&self, num: usize) {
		let previous = self.max_concurrency.swap(num, Ordering::SeqCst);
		if previous < num {
			self.semaphore.add_permits(num - previous);
		} else {
			let actual = self.semaphore.forget_permits(previous - num);
			if actual != previous - num {
				// TODO: 更好的处理这个情况
				self.max_concurrency
					.swap(previous - actual, Ordering::SeqCst);
			}
		}
	}
	
	pub fn get_group_infos(&self) -> Vec<TaskGroupInfo> {
		self.groups
			.lock()
			.iter()
			.map(|group| group.get_group_info())
			.collect()
	}
	
	pub fn create_group(&self, group: TaskGroup) -> Arc<TaskGroup> {
		let group = Arc::new(group);
		self.groups.lock().push(group.clone());
		group
	}
}
