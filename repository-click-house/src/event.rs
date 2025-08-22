use crate::context::DbContext;
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

    AppBackground = 11,
    AppForeground = 12,

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
    FaqViewEnd = 52,
    FaqScroll = 53,
    FaqSearch = 54,
    FaqDetailView = 55,

    NoticeViewStart = 61,
    NoticeViewEnd = 62,
    NoticeScroll = 63,
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
    BoardSearch = 113,
    BoardViewStart = 114,
    BoardViewEnd = 115,
    ArticleViewStart = 116,
    ArticleViewEnd = 117,
    ArticleSearch = 118,
    ArticleWriteStart = 119,
    ArticleWriteEnd = 120,

    CheckoutStart = 121,
    CheckoutStartItem = 122,
    PaymentStart = 123,
    PaymentStartItem = 124,
    PaymentComplete = 125,
    PaymentCompleteItem = 126,
}

pub trait EventBuilder: Sync + Send {
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
    pub order_id: Option<String>,
    pub tracking_id: Option<String>,
    pub product_id: Option<String>,
    pub product_option_id1: Option<String>,
    pub product_option_id2: Option<String>,
    pub product_quantity: Option<u8>,
    pub keyword: Option<String>,
    pub sort_by: Option<String>,
    pub page: Option<u8>,
    pub amount: Option<u32>,
    pub price: Option<u32>,
    pub board_id: Option<String>,
    pub article_id: Option<String>,
    pub login_id: Option<String>,
    pub phone_number: Option<String>,
    pub page_url: Option<String>,
    pub category_id: Option<String>,
    pub scroll_bucket: Option<u8>,

    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub created_at: OffsetDateTime,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            event_id: Uuid::new_v4(),
            client_id: 0,
            uuid: Uuid::nil(),
            event_type: EventType::None,
            order_id: None,
            tracking_id: None,
            product_id: None,
            product_option_id1: None,
            product_option_id2: None,
            product_quantity: None,
            board_id: None,
            article_id: None,
            keyword: None,
            sort_by: None,
            page: None,
            amount: None,
            price: None,
            login_id: None,
            phone_number: None,
            category_id: None,
            scroll_bucket: None,
            page_url: None,
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
        self.db_context.insert_event.push(event).await?;

        Ok(())
    }
}
