use std::fmt;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, ResponseError};
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

#[derive(Serialize, Debug)]
pub struct ErrResponse {
    pub code: u16,
    pub message: String,
}

impl fmt::Display for ErrResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message, self.code)
    }
}

impl ResponseError for ErrResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(
            StatusCode::from_u16(self.code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .json(self)
    }
}
