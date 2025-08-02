use async_trait::async_trait;
use repository_click_house::event::{Event, EventRepository};
use std::sync::Arc;

#[async_trait]
pub trait CollectService: Send + Sync {
    async fn create_event(&self, event: Event) -> anyhow::Result<()>;
}

pub struct CollectServiceImpl {
    repository: Arc<dyn EventRepository>,
}

impl CollectServiceImpl {
    pub fn new(repository: Arc<dyn EventRepository>) -> Arc<Self> {
        Arc::new(Self { repository })
    }
}

#[async_trait]
impl CollectService for CollectServiceImpl {
    async fn create_event(&self, event: Event) -> anyhow::Result<()> {
        self.repository
            .create_event(event)
            .await?;

        Ok(())
    }
}
