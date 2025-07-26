mod event;
mod response;
mod status;

use crate::event::collect;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use crate::response::SimpleResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(collect)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn not_found() -> SimpleResponse {
    response::NOT_FOUND_RESPONSE
}