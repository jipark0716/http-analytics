use actix_web::{post, web};
use crate::response::ErrResponse;
use crate::session::CreateSessionResponse;
use crate::status::AppStatus;

#[post("/api/v1/events/login")]
async fn create(ctx: web::Data<AppStatus>) -> Result<CreateSessionResponse, ErrResponse> {

}