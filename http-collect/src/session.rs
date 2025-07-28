use crate::response::{BasicErrorErrorResponse, ErrResponse};
use crate::status::AppStatus;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Serialize;
use uuid::Uuid;

#[post("/api/v1/sessions")]
async fn create_session(ctx: web::Data<AppStatus>) -> Result<CreateSessionResponse, ErrResponse> {
    let service = &ctx.session_service;

    let uuid = service
        .create(1)
        .await
        .map_err(|e| BasicErrorErrorResponse {
            code: 500,
            message: format!("fail create session: {:?}", e)
        })?;

    Ok(CreateSessionResponse { code: 200, uuid })
}

#[derive(Serialize)]
pub struct CreateSessionResponse {
    pub code: u16,
    pub uuid: Uuid,
}

impl Responder for CreateSessionResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.code).unwrap()).json(self)
    }
}
