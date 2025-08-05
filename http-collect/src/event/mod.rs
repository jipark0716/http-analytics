use actix_web::web;
use utoipa::OpenApi;

mod product;
mod auth;
mod app;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/events")
            .configure(product::routes)
            .configure(auth::routes)
            .configure(app::routes)
    );
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    auth::ApiDoc::openapi()
    // OpenApi::merge(vec![
    //     auth::openapi(),
    //     UserApi::openapi(),
    // ])
}