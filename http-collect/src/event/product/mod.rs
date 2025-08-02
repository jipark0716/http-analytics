pub mod start_view;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(start_view::action);
}