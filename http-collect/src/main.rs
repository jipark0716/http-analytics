mod session;
mod response;
mod status;
mod product;
mod auth;

use crate::session::create_session;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use crate::auth::{create_login_event, create_pre_login_event};
use crate::product::create_start_view_product_event;
use crate::response::SimpleResponse;
use crate::status::AppStatus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppStatus::new()))
            .service(create_session)
            .service(create_start_view_product_event)
            .service(create_pre_login_event)
            .service(create_login_event)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn not_found() -> SimpleResponse {
    response::NOT_FOUND_RESPONSE
}