use crate::context::{DbContext, InsertBuffer};
use async_trait::async_trait;
use clickhouse::Row;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Row)]
pub struct Session {
    client_id: i32,
    #[serde(with = "uuid::serde::compact")]
    uuid: Uuid,
    #[serde(with = "chrono::serde::ts_microseconds")]
    created_at: chrono::DateTime<Utc>,
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, client_id: i32) -> anyhow::Result<Uuid>;
}

pub struct SessionRepositoryImpl {
    db_context: Arc<DbContext>,
}

impl SessionRepositoryImpl {
    pub fn new(db_context: Arc<DbContext>) -> Arc<Self> {
        Arc::new(Self { db_context })
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create(&self, client_id: i32) -> anyhow::Result<Uuid> {
        let uuid = Uuid::new_v4();

        let session = Session {
            client_id,
            uuid,
            created_at: Utc::now(),
        };

        InsertBuffer::push(self.db_context.insert_sessions.clone(), session).await?;

        Ok(uuid)
    }
}
