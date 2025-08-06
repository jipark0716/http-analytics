mod view_start;
mod view_end;
mod add;
mod remove;

use actix_web::web;
use utoipa::OpenApi;
use crate::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("cart")
            .service(view_end::action)
            .service(view_start::action)
            .service(add::action)
            .service(remove::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        view_end::action,
        view_start::action,
        add::action,
        remove::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "cart", description = "장바구니")
    )
)]
pub struct ApiDoc;
