mod query;

use actix_web::web;
use utoipa::OpenApi;
use http::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("analyze")
            .service(query::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        query::action,
    ),
    components(
        schemas(SimpleResponse),
    ),
    tags(
        (name = "analyze", description = "분석")
    )
)]
pub struct ApiDoc;