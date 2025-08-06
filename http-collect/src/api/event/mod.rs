use actix_web::web;
use utoipa::OpenApi;

mod auth;
mod app;
mod category;
mod main;
mod cart;
mod faq;
mod notice;
mod event;
mod product;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("events")
            .configure(product::routes)
            .configure(event::routes)
            .configure(notice::routes)
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
    doc.merge(notice::ApiDoc::openapi());
    doc.merge(event::ApiDoc::openapi());
    doc.merge(product::ApiDoc::openapi());
    doc
}