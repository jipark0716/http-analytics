use crate::context::DbContext;
use async_trait::async_trait;
use clickhouse::Row;
use serde::Serialize;
use std::sync::Arc;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Row)]
pub struct Session {
    client_id: i32,
    #[serde(with = "clickhouse::serde::uuid")]
    uuid: uuid::Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    device_id: uuid::Uuid,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub created_at: OffsetDateTime,
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, client_id: i32, device_id: uuid::Uuid) -> anyhow::Result<uuid::Uuid>;
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
    async fn create(&self, client_id: i32, device_id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
        let uuid = uuid::Uuid::new_v4();

        let session = Session {
            client_id,
            uuid,
            device_id,
            created_at: OffsetDateTime::now_utc(),
        };

        self.db_context.insert_sessions.push(session).await?;

        Ok(uuid)
    }
}
