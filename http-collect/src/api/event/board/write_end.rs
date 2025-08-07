use repository_click_house::event::Event;
use http::response::{BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse, CREATED_RESPONSE};
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
    path = "/api/v1/events/board/article/write-end",
    operation_id = "article/write-end",
    tag = "board",
    responses(
        (status = 201, description = "success", body = SimpleResponse)
    )
)]
#[post("article/write-end")]
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
#[event_type("ArticleWriteEnd")]
#[schema(as = ArticleWriteEndRequest)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required, length(min = 1))]
    pub board_id: Option<String>,
}