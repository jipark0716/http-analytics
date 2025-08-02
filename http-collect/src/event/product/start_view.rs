use repository_click_house::event::Event;
use repository_click_house::event::EventBuilder;
use crate::response::{BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse, CREATED_RESPONSE};
use crate::status::AppStatus;
use actix_web::{post, web};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;
use repository_click_house_macro::Event;
use repository_click_house::event::EventType;

#[post("/api/v1/events/start-view-product")]
async fn action(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateStartViewProductEventRequest>,
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

#[derive(Debug, Deserialize, Validate, Event)]
#[event_type("ProductViewStart")]
pub struct CreateStartViewProductEventRequest {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required, length(min = 1))]
    pub product_id: Option<String>,
}
