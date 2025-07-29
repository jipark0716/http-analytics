use crate::context::{DbContext, InsertBuffer};
use async_trait::async_trait;
use clickhouse::Row;
use sea_orm::entity::prelude::*;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EventType {
    StartViewProduct = 1,
    Login = 2,
    PreLogin = 3,
}

#[derive(Debug, Serialize, Row)]
pub struct Event {
    #[serde(with = "clickhouse::serde::uuid")]
    pub event_id: Uuid,

    pub client_id: i32,

    #[serde(with = "clickhouse::serde::uuid")]
    pub uuid: Uuid,

    event_type: EventType,

    product_id: Option<String>,

    login_id: Option<String>,

    phone_number: Option<String>,

    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    created_at: OffsetDateTime,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            event_id: Uuid::nil(),
            client_id: 0,
            uuid: Uuid::nil(),
            event_type: EventType::StartViewProduct,
            product_id: None,
            login_id: None,
            phone_number: None,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn create_view_product_event(
        &self,
        client_id: i32,
        uuid: Uuid,
        product_id: String,
    ) -> anyhow::Result<Uuid>;
    async fn create_login_event(
        &self,
        client_id: i32,
        uuid: Uuid,
        login_id: String,
        phone_number: Option<String>,
    ) -> anyhow::Result<Uuid>;
    async fn create_pre_login_event(
        &self,
        client_id: i32,
        uuid: Uuid,
        login_id: String,
        phone_number: Option<String>,
    ) -> anyhow::Result<Uuid>;
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
    async fn create_view_product_event(
        &self,
        client_id: i32,
        uuid: Uuid,
        product_id: String,
    ) -> anyhow::Result<Uuid> {
        let event_id = Uuid::new_v4();

        InsertBuffer::push(
            self.db_context.insert_event.clone(),
            Event {
                event_id,
                client_id,
                uuid,
                event_type: EventType::StartViewProduct,
                product_id: Some(product_id),
                ..Default::default()
            },
        )
        .await?;

        Ok(event_id)
    }

    async fn create_login_event(
        &self,
        client_id: i32,
        uuid: Uuid,
        login_id: String,
        phone_number: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let event_id = Uuid::new_v4();

        InsertBuffer::push(
            self.db_context.insert_event.clone(),
            Event {
                event_id,
                client_id,
                uuid,
                login_id: Some(login_id),
                phone_number,
                event_type: EventType::Login,
                ..Default::default()
            },
        )
        .await?;

        Ok(event_id)
    }

    async fn create_pre_login_event(
        &self,
        client_id: i32,
        uuid: Uuid,
        login_id: String,
        phone_number: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let event_id = Uuid::new_v4();

        InsertBuffer::push(
            self.db_context.insert_event.clone(),
            Event {
                event_id,
                client_id,
                uuid,
                login_id: Some(login_id),
                phone_number,
                event_type: EventType::PreLogin,
                ..Default::default()
            },
        )
        .await?;

        Ok(event_id)
    }
}
