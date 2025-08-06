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
        (name = "app", description = "어플리케이션")
    )
)]
pub struct ApiDoc;
