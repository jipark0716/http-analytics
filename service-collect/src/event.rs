use async_trait::async_trait;
use repository_click_house::event::EventRepository;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait CollectService : Send + Sync {
    async fn create_start_view_product_event(&self, client_id: i32, uuid: Uuid, product_id: String) -> anyhow::Result<Uuid>;
    async fn create_login_event(&self, client_id: i32, uuid: Uuid, login_id: String, phone_number: Option<String>) -> anyhow::Result<Uuid>;
    async fn create_pre_login_event(&self, client_id: i32, uuid: Uuid, login_id: String, phone_number: Option<String>) -> anyhow::Result<Uuid>;
}

pub struct CollectServiceImpl {
    repository: Arc<dyn EventRepository>,
}

impl CollectServiceImpl {
    pub fn new(repository: Arc<dyn EventRepository>) -> Arc<Self> {
        Arc::new(Self {
            repository
        })
    }
}

#[async_trait]
impl CollectService for CollectServiceImpl {
    async fn create_start_view_product_event(&self, client_id: i32, uuid: Uuid, product_id: String) -> anyhow::Result<Uuid> {
        self.repository.create_view_product_event(client_id, uuid, product_id).await
    }

    async fn create_login_event(&self, client_id: i32, uuid: Uuid, login_id: String, phone_number: Option<String>) -> anyhow::Result<Uuid> {
        self.repository.create_login_event(client_id, uuid, login_id, phone_number).await
    }

    async fn create_pre_login_event(&self, client_id: i32, uuid: Uuid, login_id: String, phone_number: Option<String>) -> anyhow::Result<Uuid> {
        self.repository.create_pre_login_event(client_id, uuid, login_id, phone_number).await
    }
}