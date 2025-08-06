mod scroll;
mod view_start;
mod view_end;
mod search;
mod like_add;
mod like_remove;

use actix_web::web;
use utoipa::OpenApi;
use crate::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("product")
            .service(search::action)
            .service(view_start::action)
            .service(view_end::action)
            .service(scroll::action)
            .service(like_add::action)
            .service(like_remove::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "product", description = "상품")
    )
)]
pub struct ApiDoc;
