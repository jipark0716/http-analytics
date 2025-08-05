mod foreground;
mod background;

use crate::response::SimpleResponse;
use actix_web::web;
use utoipa::OpenApi;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("app")
            .service(foreground::action)
            .service(background::action),
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        foreground::action,
        background::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "auth", description = "Authentication related endpoints")
    )
)]
pub struct ApiDoc;
