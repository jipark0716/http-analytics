mod checkout_start;
mod payment_start;
mod payment_complete;
mod product_request;

use actix_web::web;
use utoipa::OpenApi;
use http::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("payment")
            .service(checkout_start::action)
            .service(payment_start::action)
            .service(payment_complete::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        checkout_start::action,
        payment_start::action,
        payment_complete::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "payment", description = "결제")
    )
)]
pub struct ApiDoc;
