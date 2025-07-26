use async_trait::async_trait;

#[async_trait]
pub trait CollectService {
    async fn collect(&self);
}