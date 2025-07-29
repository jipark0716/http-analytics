use crate::response::{BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse, CREATED_RESPONSE};
use crate::status::AppStatus;
use actix_web::{post, web};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[post("/api/v1/events/start-view-product")]
async fn create_start_view_product_event(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateStartViewProductEventRequest>,
) -> Result<SimpleResponse, ErrResponse> {
    let body = request.into_inner();
    body.validate().map_err(|e| ValidationErrorResponse {
        code: 400,
        message: format!("validation error: {:?}", e),
        error: e,
    })?;

    let service = &ctx.collect_service;

    let uuid = service
        .create_start_view_product_event(
            body.client_id.expect("client_id validated"),
            body.uuid.expect("uuid validated"),
            body.product_id.expect("product_id validated"),
        )
        .await
        .map_err(|e| BasicErrorErrorResponse {
            code: 500,
            message: format!("fail create event: {:?}", e)
        })?;

    Ok(CREATED_RESPONSE)
}

#[derive(Debug, Deserialize, Validate)]
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
