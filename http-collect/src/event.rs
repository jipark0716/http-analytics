use crate::response::ErrResponse;
use crate::{response, status::AppStatus, SimpleResponse};
use actix_web::{post, web};

#[post("/api/v1/collect")]
async fn collect(ctx: web::Data<AppStatus>) -> Result<SimpleResponse, ErrResponse> {
    let service = &ctx.session_service;

    service.create(1).await.map_err(|e| ErrResponse {
        code: 500,
        message: format!("error create session {e}"),
    })?;

    Ok(response::SUCCESS_RESPONSE)
}