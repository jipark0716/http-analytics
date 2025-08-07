mod analyze;

use actix_web::web;
use utoipa::OpenApi;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(analyze::routes)
    );
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    analyze::ApiDoc::openapi()
}