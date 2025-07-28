use std::error::Error;
use async_trait::async_trait;
use repository_click_house::event::EventRepository;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait CollectService : Send + Sync {
    async fn create_start_view_product_event(&self, client_id: i32, uuid: Uuid, product_id: String) -> anyhow::Result<Uuid, Box<dyn Error>>;
}

pub struct CollectServiceImpl {
    repository: Arc<dyn EventRepository>,
}

impl CollectServiceImpl {
    pub fn new(repository: Arc<dyn EventRepository>) -> Self {
        Self {
            repository
        }
    }
}

#[async_trait]
impl CollectService for CollectServiceImpl {
    async fn create_start_view_product_event(&self, client_id: i32, uuid: Uuid, product_id: String) -> anyhow::Result<Uuid, Box<dyn Error>> {
        self.repository.create_view_product_event(client_id, uuid, product_id)
    }
}