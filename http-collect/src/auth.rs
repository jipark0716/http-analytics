use crate::product::CreateStartViewProductEventRequest;
use crate::response::{ErrResponse, SimpleResponse, CREATED_RESPONSE};
use crate::status::AppStatus;
use actix_web::{post, web};

#[post("/api/v1/events/login")]
async fn create_login_event(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateStartViewProductEventRequest>,
) -> Result<SimpleResponse, ErrResponse> {
    Ok(CREATED_RESPONSE)
}

#[post("/api/v1/events/pre-login")]
async fn create_pre_login_event(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateStartViewProductEventRequest>,
) -> Result<SimpleResponse, ErrResponse> {
    Ok(CREATED_RESPONSE)
}
