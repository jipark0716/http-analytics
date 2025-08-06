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
    path = "/api/v1/events/category/scroll",
    operation_id = "scroll",
    tag = "category",
    responses(
        (status = 201, description = "success", body = SimpleResponse)
    )
)]
#[post("scroll")]
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
#[event_type("CategoryScroll")]
#[schema(as = CategoryScrollRequestRequest)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required, length(min = 1))]
    pub category_id: Option<String>,

    #[serde(default)]
    #[validate(required)]
    pub scroll_bucket: Option<u8>,
}