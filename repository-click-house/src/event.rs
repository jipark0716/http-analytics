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
    None = 0,
    
    Login = 1,
    PreLogin = 2,
    LogOut = 3,
    SignInStart = 4,
    SignInEnd = 5,
    PasswordFindStart = 6,
    PasswordFindEnd = 7,

    PageHide = 11,
    PageShow = 12,

    CategoryViewStart = 21,
    CategoryScroll = 22,
    CategoryViewEnd = 23,

    MainViewStart = 31,
    MainScroll = 32,
    MainViewEnd = 33,

    CartAdd = 41,
    CartRemove = 42,
    CartViewStart = 43,
    CartViewEnd = 44,

    FaqViewStart = 51,
    FaqScroll = 52,
    FaqViewEnd = 53,
    FaqSearch = 54,
    FaqDetailView = 55,

    NoticeViewStart = 61,
    NoticeScroll = 62,
    NoticeViewEnd = 63,
    NoticeSearch = 64,
    NoticeDetailView = 65,

    EventViewStart = 71,
    EventScroll = 72,
    EventViewEnd = 73,
    EventSearch = 74,
    EventDetailViewStart = 75,
    EventDetailScroll = 76,
    EventDetailViewEnd = 77,

    ProductSearch = 81,
    ProductViewStart = 82,
    ProductViewEnd = 83,
    ProductScroll = 84,
    ProductLikeAdd = 85,
    ProductLikeRemove = 86,

    EtcViewStart = 91,
    EtcScroll = 92,
    EtcViewEnd = 93,

    OrderListViewStart = 101,
    OrderListViewEnd = 102,
    OrderDetailViewStart = 103,
    OrderDetailViewEnd = 104,
    DeliveryTracking = 105,

    BoardListViewStart = 111,
    BoardListViewEnd = 112,
    BoardCategoryViewStart = 113,
    BoardCategoryViewEnd = 114,
    BoardPostViewStart = 115,
    BoardPostViewEnd = 116,
    BoardPostWriteStart = 117,
    BoardPostWriteEnd = 118,

    CheckoutStart = 121,
    PaymentStart = 122,
    PaymentComplete = 123,
}

pub trait EventBuilder : Sync + Send {
    fn into_inner(self) -> Event;
}

#[derive(Debug, Serialize, Row)]
pub struct Event {
    #[serde(with = "clickhouse::serde::uuid")]
    pub event_id: Uuid,

    pub client_id: i32,

    #[serde(with = "clickhouse::serde::uuid")]
    pub uuid: Uuid,

    pub event_type: EventType,

    pub product_id: Option<String>,

    pub login_id: Option<String>,

    pub phone_number: Option<String>,

    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub created_at: OffsetDateTime,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            event_id: Uuid::nil(),
            client_id: 0,
            uuid: Uuid::nil(),
            event_type: EventType::None,
            product_id: None,
            login_id: None,
            phone_number: None,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn create_event(&self, event: Event) -> anyhow::Result<()>;
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
    async fn create_event(&self, event: Event) -> anyhow::Result<()> {
        InsertBuffer::push(
            self.db_context.insert_event.clone(),
            event,
        ).await?;

        Ok(())
    }
}
