use crate::api::event::payment::product_request;
use crate::response::{BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse, CREATED_RESPONSE};
use crate::status::AppStatus;
use actix_web::{post, web};
use repository_click_house::event::{Event, EventType};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/events/payment/payment-complete",
    operation_id = "checkout-complete",
    tag = "payment",
    responses(
        (status = 201, description = "success", body = SimpleResponse)
    )
)]
#[post("payment-complete")]
async fn action(
    ctx: web::Data<AppStatus>,
    request: web::Json<Request>,
) -> Result<SimpleResponse, ErrResponse> {
    let body = request.into_inner();
    let service = &ctx.collect_service;
    let event_id = Uuid::new_v4();

    body.validate().map_err(ValidationErrorResponse::from)?;

    let events: Vec<Event> = body.products
        .unwrap()
        .into_iter()
        .map(|product| product.new(EventType::PaymentCompleteItem, event_id, body.client_id, body.uuid))
        .collect();

    service
        .create_events(events)
        .await
        .map_err(BasicErrorErrorResponse::from)?;

    service
        .create_event(Event {
            event_id,
            client_id: body.client_id.unwrap(),
            uuid: body.uuid.unwrap(),
            event_type: EventType::PaymentComplete,
            ..Default::default()
        })
        .await
        .map_err(BasicErrorErrorResponse::from)?;

    Ok(CREATED_RESPONSE)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(as = PaymentStartRequest)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required)]
    pub amount: Option<u32>,

    #[serde(default)]
    #[validate(required)]
    pub products: Option<Vec<product_request::Request>>,
}