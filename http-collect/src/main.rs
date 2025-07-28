mod session;
mod response;
mod status;
mod event;

use crate::session::create_session;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use crate::response::SimpleResponse;
use crate::status::AppStatus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppStatus::new()))
            .service(create_session)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn not_found() -> SimpleResponse {
    response::NOT_FOUND_RESPONSE
}