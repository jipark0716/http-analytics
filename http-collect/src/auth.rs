use crate::response::{BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse, CREATED_RESPONSE};
use crate::status::AppStatus;
use actix_web::{post, web};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[post("/api/v1/events/login")]
async fn create_login_event(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateLoginEventRequest>,
) -> Result<SimpleResponse, ErrResponse> {
    let body = request.into_inner();
    body.validate().map_err(|e| ValidationErrorResponse {
        code: 400,
        message: format!("validation error: {:?}", e),
        error: e,
    })?;

    let service = &ctx.collect_service;

    let uuid = service
        .create_login_event(
            body.client_id.expect("client_id validated"),
            body.uuid.expect("uuid validated"),
            body.login_id.expect("product_id validated"),
            body.phone_number,
        )
        .await
        .map_err(|e| BasicErrorErrorResponse {
            code: 500,
            message: format!("fail create event: {:?}", e)
        })?;

    Ok(CREATED_RESPONSE)
}

#[post("/api/v1/events/pre-login")]
async fn create_pre_login_event(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateLoginEventRequest>,
) -> Result<SimpleResponse, ErrResponse> {
    let body = request.into_inner();
    body.validate().map_err(|e| ValidationErrorResponse {
        code: 400,
        message: format!("validation error: {:?}", e),
        error: e,
    })?;

    let service = &ctx.collect_service;

    let uuid = service
        .create_pre_login_event(
            body.client_id.expect("client_id validated"),
            body.uuid.expect("uuid validated"),
            body.login_id.expect("product_id validated"),
            body.phone_number,
        )
        .await
        .map_err(|e| BasicErrorErrorResponse {
            code: 500,
            message: format!("fail create event: {:?}", e)
        })?;

    Ok(CREATED_RESPONSE)
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLoginEventRequest {
    #[serde(default)]
    #[validate(required)]
    pub uuid: Option<Uuid>,

    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub login_id: Option<String>,

    #[serde(default)]
    pub phone_number: Option<String>,
}