use crate::context::{DbContext, InsertBuffer};
use async_trait::async_trait;
use clickhouse::Row;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Row)]
pub struct Session {
    client_id: i32,
    uuid: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, client_id: i32) -> anyhow::Result<Uuid, Box<dyn Error>>;
}

struct SessionRepositoryImpl {
    db_context: Arc<DbContext>,
}

impl SessionRepositoryImpl {
    fn new(db_context: Arc<DbContext>) -> Self {
        Self { db_context }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create(&self, client_id: i32) -> anyhow::Result<Uuid, Box<dyn Error>> {
        let uuid = Uuid::new_v4();

        let session = Session {
            client_id,
            uuid,
            created_at: chrono::Utc::now(),
        };

        InsertBuffer::push(self.db_context.insert_sessions.clone(), session).await?;

        Ok(uuid)
    }
}
