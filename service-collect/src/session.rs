use anyhow::Result;
use async_trait::async_trait;
use repository_click_house::session::SessionRepository;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait SessionService: Send + Sync {
    async fn create(&self, client_id: i32) -> Result<Uuid>;
}

pub struct SessionServiceImpl {
    repository: Arc<dyn SessionRepository>,
}

impl SessionServiceImpl {
    pub fn new(click_house_db_context: Arc<dyn SessionRepository>) -> Arc<Self> {
        Arc::new(Self {
            repository: click_house_db_context,
        })
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create(&self, client_id: i32) -> Result<Uuid> {
        self.repository.create(client_id).await
    }
}
