use actix_web::post;
use crate::{response, SimpleResponse};

#[post("/api/v1/collect")]
async fn collect() -> SimpleResponse {
    response::SUCCESS_RESPONSE
}