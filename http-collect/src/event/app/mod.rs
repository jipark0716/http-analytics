mod foreground;
mod background;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("app")
            .service(foreground::action)
            .service(background::action),
    );
}
