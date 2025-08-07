mod scroll;
mod view_start;
mod view_end;
mod search;
mod view_start_detail;
mod view_end_detail;
mod scroll_detail;

use actix_web::web;
use utoipa::OpenApi;
use http::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("event")
            .service(scroll::action)
            .service(view_start::action)
            .service(view_end::action)
            .service(search::action)
            .service(view_start_detail::action)
            .service(view_end_detail::action)
            .service(scroll_detail::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        scroll::action,
        view_start::action,
        view_end::action,
        search::action,
        view_start_detail::action,
        view_end_detail::action,
        scroll_detail::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "event", description = "이벤트")
    )
)]
pub struct ApiDoc;
