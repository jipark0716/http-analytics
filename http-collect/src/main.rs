mod session;
mod response;
mod status;
mod auth;
mod event;

use crate::response::SimpleResponse;
use crate::session::create_session;
use crate::status::AppStatus;
use actix_web::{web, App, HttpServer, Responder};
use config::collect::HttpCollectConfig;
use config::import;
use serde::Serialize;

#[cfg(feature = "development")]
static CONFIG_BIN: &[u8] = include_bytes!("../config/development.bin");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = import::<HttpCollectConfig>(CONFIG_BIN);
    let config_for_server = config.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppStatus::new(config.clone())))
            .service(create_session)
            .configure(event::routes)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", config_for_server.clone().http.port))?
    .run()
    .await
}

async fn not_found() -> SimpleResponse {
    response::NOT_FOUND_RESPONSE
}