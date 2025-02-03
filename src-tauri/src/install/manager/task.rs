use async_trait::async_trait;

#[async_trait]
pub trait Task {
	fn get_progress_percent(&self) -> u8;
	async fn run(&self) -> anyhow::Result<()>;
	async fn cancel(&self) -> anyhow::Result<()>;
}
