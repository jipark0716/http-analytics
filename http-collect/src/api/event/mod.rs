use actix_web::web;
use utoipa::OpenApi;

mod auth;
mod app;
mod category;
mod main;
mod cart;
mod faq;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("events")
            .configure(faq::routes)
            .configure(cart::routes)
            .configure(main::routes)
            .configure(auth::routes)
            .configure(app::routes)
            .configure(category::routes)
    );
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    let mut doc = auth::ApiDoc::openapi();
    doc.merge(app::ApiDoc::openapi());
    doc.merge(category::ApiDoc::openapi());
    doc.merge(main::ApiDoc::openapi());
    doc.merge(cart::ApiDoc::openapi());
    doc.merge(faq::ApiDoc::openapi());
    doc
}