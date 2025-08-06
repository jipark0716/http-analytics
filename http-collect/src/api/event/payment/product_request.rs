use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use repository_click_house::event::{Event, EventType};

#[derive(Debug, Deserialize, Validate, ToSchema, Serialize)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub product_id: Option<String>,

    #[serde(default)]
    pub product_option_id1: Option<String>,

    #[serde(default)]
    pub product_option_id2: Option<String>,

    #[serde(default)]
    #[validate(required)]
    pub product_quantity: Option<u8>,

    #[serde(default)]
    #[validate(required)]
    pub amount: Option<u32>,

    #[serde(default)]
    #[validate(required)]
    pub price: Option<u32>,
}

impl Request {
    pub fn new(self, event_type: EventType, event_id: Uuid, client_id: Option<i32>, uuid: Option<Uuid>) -> Event {
        Event {
            event_id,
            client_id: client_id.unwrap(),
            uuid: uuid.unwrap(),
            event_type,
            product_id: self.product_id,
            product_option_id1: self.product_option_id1,
            product_option_id2: self.product_option_id2,
            product_quantity: self.product_quantity,
            amount: self.amount,
            price: self.price,
            ..Default::default()
        }
    }
}