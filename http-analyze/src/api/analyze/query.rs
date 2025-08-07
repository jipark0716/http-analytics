use crate::status::AppStatus;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, post, web};
use http::response::{
    BasicErrorErrorResponse, ErrResponse, SimpleResponse, ValidationErrorResponse,
};
use serde::{Deserialize, Serialize};
use service_analyze::create_query::{Query, QueryType};
use utoipa::ToSchema;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/analyze/query",
    tag = "analyze",
    responses(
        (status = 201, description = "success", body = SimpleResponse)
    )
)]
#[post("query")]
async fn action(
    ctx: web::Data<AppStatus>,
    request: web::Json<Request>,
) -> Result<Response, ErrResponse> {
    let body = request.into_inner();
    let create_query_service = &ctx.create_query_service;

    body.validate().map_err(ValidationErrorResponse::from)?;

    let query = create_query_service
        .create_query(body.client_id.unwrap(), body.query.unwrap())
        .await
        .map_err(|e| BasicErrorErrorResponse {
            code: 500,
            message: format!("Fail Create Query: {:?}", e),
        })?;

    let response: Response = query.into();

    Ok(response)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(as = AnalyzeCreateQueryRequest)]
pub struct Request {
    #[serde(default)]
    #[validate(required)]
    pub client_id: Option<i32>,

    #[serde(default)]
    #[validate(required, length(min = 1))]
    pub query: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[schema(as = AnalyzeCreateQueryResponse)]
pub struct Response {
    pub query: String,
    pub query_type: String,
}

impl From<Query> for Response {
    fn from(value: Query) -> Self {
        Self {
            query: value.query,
            query_type: if value.query_type == QueryType::List {
                "List".to_string()
            } else if value.query_type == QueryType::Group {
                "Group".to_string()
            } else {
                "None".to_string()
            },
        }
    }
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(200).unwrap()).json(self)
    }
}
