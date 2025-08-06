use crate::response::{BasicErrorErrorResponse, ErrResponse, ValidationErrorResponse};
use crate::status::AppStatus;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/sessions",
    tag = "session",
    responses(
        (status = 201, description = "success", body = CreateSessionResponse)
    )
)]
#[post("")]
async fn action(
    ctx: web::Data<AppStatus>,
    request: web::Json<CreateSessionRequest>,
) -> Result<CreateSessionResponse, ErrResponse> {
    let body = request.into_inner();
    body.validate().map_err(ValidationErrorResponse::from)?;

    let service = &ctx.session_service;

    let uuid = service
        .create(
            body.client_id.expect("client_id is required"),
            body.device_id.expect("device_id is required"),
        )
        .await
        .map_err(BasicErrorErrorResponse::from)?;

    Ok(CreateSessionResponse { code: 200, uuid })
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct CreateSessionRequest {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required)]
    pub device_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
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
