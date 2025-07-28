use async_trait::async_trait;

#[async_trait]
pub trait CollectService {
    async fn create_start_view_product_event(&self);
}

pub struct CollectServiceImpl {}