use actix_web::body::BoxBody;
use actix_web::{HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

pub const NOT_FOUND_RESPONSE: SimpleResponse = SimpleResponse {
    code: 404,
    message: "not found",
};

pub const SUCCESS_RESPONSE: SimpleResponse = SimpleResponse {
    code: 200,
    message: "success",
};

#[derive(Serialize)]
pub struct SimpleResponse {
    pub code: u16,
    pub message: &'static str,
}

impl Responder for SimpleResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.code).unwrap()).json(self)
    }
}
