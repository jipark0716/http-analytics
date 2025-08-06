use actix_web::web;
use utoipa::OpenApi;
use crate::response::SimpleResponse;

mod view_start;
mod view_end;
mod view_start_detail;
mod view_end_detail;
mod delivery_tracking;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("order")
            .service(view_start::action)
            .service(view_end::action)
            .service(view_start_detail::action)
            .service(view_end_detail::action)
            .service(delivery_tracking::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        view_start::action,
        view_end::action,
        view_start_detail::action,
        view_end_detail::action,
        delivery_tracking::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "order", description = "주문")
    )
)]
pub struct ApiDoc;
