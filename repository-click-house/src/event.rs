use chrono::Utc;
use clickhouse::Row;
use uuid::Uuid;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Nullable;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum EventType {
    #[sea_orm(num_value = 1)]
    StartViewProduct,
}

#[derive(Debug, Serialize, Row)]
pub struct Event {
    pub client_id: i32,

    #[serde(with = "uuid::serde::compact")]
    pub uuid: Uuid,

    event_type: EventType,

    product_id: Option<String>,

    #[serde(with = "chrono::serde::ts_microseconds")]
    created_at: chrono::DateTime<Utc>,
}