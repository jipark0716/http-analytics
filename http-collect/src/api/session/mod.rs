use actix_web::web;
use utoipa::OpenApi;
use create::CreateSessionResponse;

pub mod create;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("sessions")
            .service(create::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create::action,
    ),
    components(
        schemas(CreateSessionResponse)
    ),
    tags(
        (name = "session", description = "세션")
    )
)]
pub struct ApiDoc;
