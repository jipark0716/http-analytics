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
    path = "/api/v1/events/product/search",
    operation_id = "search",
    tag = "product",
    responses(
        (status = 201, description = "success", body = SimpleResponse)
    )
)]
#[post("search")]
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
#[event_type("ProductSearch")]
#[schema(as = ProductSearchRequest)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required, length(min = 1))]
    pub keyword: Option<String>,

    #[serde(default)]
    pub sort_by: Option<String>,
}