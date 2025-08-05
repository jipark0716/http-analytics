use actix_web::web;

pub mod product;
pub mod auth;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/events")
            .configure(product::routes)
            .configure(auth::routes)
    );
}