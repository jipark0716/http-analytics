use crate::context::{DbContext, InsertBuffer};
use async_trait::async_trait;
use chrono::Utc;
use clickhouse::Row;
use sea_orm::entity::prelude::*;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EventType {
    StartViewProduct = 1,
}

#[derive(Debug, Serialize, Row)]
pub struct Event {
    #[serde(with = "uuid::serde::compact")]
    pub event_id: Uuid,

    pub client_id: i32,

    #[serde(with = "uuid::serde::compact")]
    pub uuid: Uuid,

    event_type: EventType,

    product_id: Option<String>,

    #[serde(with = "chrono::serde::ts_microseconds")]
    created_at: chrono::DateTime<Utc>,
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn create_view_product_event(&self, client_id: i32, uuid: Uuid, product_id: String) -> anyhow::Result<Uuid>;
}

pub struct EventRepositoryImpl {
    db_context: Arc<DbContext>,
}

impl EventRepositoryImpl {
    pub fn new(db_context: Arc<DbContext>) -> Arc<Self> {
        Arc::new(Self { db_context })
    }
}

#[async_trait]
impl EventRepository for EventRepositoryImpl {
    async fn create_view_product_event(&self, client_id: i32, uuid: Uuid, product_id: String) -> anyhow::Result<Uuid>{
        let event_id = Uuid::new_v4();

        let event = Event {
            event_id,
            client_id,
            uuid,
            event_type: EventType::StartViewProduct,
            product_id: Some(product_id),
            created_at: Utc::now(),
        };

        InsertBuffer::push(self.db_context.insert_event.clone(), event).await?;

        Ok(event_id)
    }
}
