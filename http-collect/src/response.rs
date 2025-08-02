use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use validator::{Validate, ValidationErrors};
use crate::product::CreateStartViewProductEventRequest;

pub const NOT_FOUND_RESPONSE: SimpleResponse = SimpleResponse {
    code: 404,
    message: "not found",
};

pub const SUCCESS_RESPONSE: SimpleResponse = SimpleResponse {
    code: 200,
    message: "success",
};

pub const CREATED_RESPONSE: SimpleResponse = SimpleResponse {
    code: 201,
    message: "created",
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

#[derive(Debug)]
pub enum ErrResponse {
    Validation(ValidationErrorResponse),
    BasicError(BasicErrorErrorResponse),
    Unknown(anyhow::Error),
}

impl Display for ErrResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ResponseError for ErrResponse {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Validation(e) => HttpResponse::build(
                StatusCode::from_u16(e.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            )
            .json(e),
            Self::BasicError(e) => HttpResponse::build(
                StatusCode::from_u16(e.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            )
            .json(e),
            Self::Unknown(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub code: u16,
    pub message: String,
    pub error: ValidationErrors,
}

impl From<ValidationErrorResponse> for ErrResponse {
    fn from(e: ValidationErrorResponse) -> Self {
        ErrResponse::Validation(e)
    }
}

impl From<ValidationErrors> for ValidationErrorResponse {
    fn from(e: ValidationErrors) -> Self {
        Self {
            code: 400,
            message: format!("validation error: {:?}", e),
            error: e,
        }
    }
}

impl From<anyhow::Error> for BasicErrorErrorResponse {
    fn from(e: anyhow::Error) -> Self {
        Self {
            code: 500,
            message: format!("fail create event: {:?}", e)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BasicErrorErrorResponse {
    pub code: u16,
    pub message: String,
}

impl From<BasicErrorErrorResponse> for ErrResponse {
    fn from(e: BasicErrorErrorResponse) -> Self {
        ErrResponse::BasicError(e)
    }
}