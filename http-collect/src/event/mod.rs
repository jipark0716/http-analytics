use actix_web::web;

pub mod product;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(product::routes)
    );
}