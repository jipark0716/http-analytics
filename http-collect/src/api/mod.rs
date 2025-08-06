use actix_web::web;
use utoipa::OpenApi;

mod event;
mod session;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(event::routes)
            .configure(session::routes)
    );
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    let mut doc = event::openapi();
    doc.merge(session::ApiDoc::openapi());
    doc
}