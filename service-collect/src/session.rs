use anyhow::Result;
use async_trait::async_trait;
use repository_click_house::session::SessionRepository;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait SessionService: Send + Sync {
    async fn create(&self, client_id: i32) -> Result<Uuid, Box<dyn Error>>;
}

pub struct SessionServiceImpl {
    click_house_db_context: Arc<dyn SessionRepository>,
}

impl SessionServiceImpl {
    pub fn new(click_house_db_context: Arc<dyn SessionRepository>) -> Self {
        Self {
            click_house_db_context,
        }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create(&self, client_id: i32) -> Result<Uuid, Box<dyn Error>> {
        self.click_house_db_context.create(client_id).await
    }
}
