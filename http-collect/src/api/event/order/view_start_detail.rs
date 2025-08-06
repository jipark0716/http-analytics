use repository_click_house::event::Event;
use crate::response::{BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse, CREATED_RESPONSE};
use crate::status::AppStatus;
use actix_web::{post, web};
use repository_click_house_macro::Event;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use repository_click_house::event::EventBuilder;
use repository_click_house::event::EventType;

#[utoipa::path(
    post,
    path = "/api/v1/events/order/view-start-detail",
    operation_id = "view-start-detail",
    tag = "order",
    responses(
        (status = 201, description = "success", body = SimpleResponse)
    )
)]
#[post("view-start-detail")]
async fn action(
    ctx: web::Data<AppStatus>,
    request: web::Json<Request>,
) -> Result<SimpleResponse, ErrResponse> {
    let body = request.into_inner();
    let service = &ctx.collect_service;

    body.validate().map_err(ValidationErrorResponse::from)?;

    service
        .create_event(body.into_inner())
        .await
        .map_err(BasicErrorErrorResponse::from)?;

    Ok(CREATED_RESPONSE)
}

#[derive(Debug, Deserialize, Validate, Event, ToSchema)]
#[event_type("OrderDetailViewStart")]
#[schema(as = OrderDetailViewStartRequestRequest)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required, length(min = 1))]
    pub order_id: Option<String>,
}