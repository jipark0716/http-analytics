mod scroll;
mod view_start;
mod view_end;
mod search;
mod view_detail;

use actix_web::web;
use utoipa::OpenApi;
use http::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("notice")
            .service(view_start::action)
            .service(view_end::action)
            .service(scroll::action)
            .service(search::action)
            .service(view_detail::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        view_start::action,
        view_end::action,
        scroll::action,
        search::action,
        view_detail::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "notice", description = "공지사항")
    )
)]
pub struct ApiDoc;
