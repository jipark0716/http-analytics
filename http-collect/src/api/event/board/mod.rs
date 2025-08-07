mod search_article;
mod search_board;
mod view_end;
mod view_end_article;
mod view_end_board;
mod view_start;
mod view_start_article;
mod view_start_board;
mod write_end;
mod write_start;

use actix_web::web;
use utoipa::OpenApi;
use http::response::SimpleResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("board")
            .service(search_article::action)
            .service(search_board::action)
            .service(view_end::action)
            .service(view_end_article::action)
            .service(view_end_board::action)
            .service(view_start::action)
            .service(view_start_article::action)
            .service(view_start_board::action)
            .service(write_end::action)
            .service(write_start::action)
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        search_article::action,
        search_board::action,
        view_end::action,
        view_end_article::action,
        view_end_board::action,
        view_start::action,
        view_start_article::action,
        view_start_board::action,
        write_end::action,
        write_start::action,
    ),
    components(
        schemas(SimpleResponse)
    ),
    tags(
        (name = "board", description = "게시판")
    )
)]
pub struct ApiDoc;
