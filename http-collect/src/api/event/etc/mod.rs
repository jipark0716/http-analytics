mod view_start;
mod scroll;
mod view_end;

use actix_web::web;
use utoipa::OpenApi;
use http::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("etc")
            .service(view_end::action)
            .service(scroll::action)
            .service(view_start::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        view_end::action,
        scroll::action,
        view_start::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "etc", description = "기타 화면")
    )
)]
pub struct ApiDoc;
