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
    SignIn = 4,
    PasswordFindStart = 5,
    PasswordFindEnd = 6,

    PageHide = 7,
    PageShow = 8,

    CategoryViewStart = 9,
    CategoryScroll = 10,
    CategoryViewEnd = 11,

    MainViewStart = 12,
    MainScroll = 13,
    MainViewEnd = 14,

    CartAdd = 15,
    CartRemove = 16,
    CartViewStart = 17,
    CartViewEnd = 18,

    FaqViewStart = 19,
    FaqScroll = 20,
    FaqViewEnd = 21,
    FaqSearch = 22,
    FaqDetailView = 23,

    NoticeViewStart = 24,
    NoticeScroll = 25,
    NoticeViewEnd = 26,
    NoticeSearch = 27,
    NoticeDetailView = 28,

    EventViewStart = 29,
    EventScroll = 30,
    EventViewEnd = 31,
    EventSearch = 32,
    EventDetailViewStart = 33,
    EventDetailScroll = 34,
    EventDetailViewEnd = 35,

    ProductSearch = 36,
    ProductViewStart = 37,
    ProductViewEnd = 38,
    ProductScroll = 39,
    ProductLikeAdd = 40,
    ProductLikeRemove = 41,

    EtcViewStart = 42,
    EtcScroll = 43,
    EtcViewEnd = 44,

    OrderListViewStart = 45,
    OrderListViewEnd = 46,
    OrderDetailViewStart = 47,
    OrderDetailViewEnd = 48,
    DeliveryTracking = 49,

    BoardListViewStart = 50,
    BoardListViewEnd = 51,
    BoardCategoryViewStart = 52,
    BoardCategoryViewEnd = 53,
    BoardPostViewStart = 54,
    BoardPostViewEnd = 55,
    BoardPostWriteStart = 56,
    BoardPostWriteEnd = 57,

    CheckoutStart = 58,
    PaymentStart = 59,
    PaymentComplete = 60,
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
