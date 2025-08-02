mod session;
mod response;
mod status;
mod product;
mod auth;

// use crate::auth::{create_login_event, create_pre_login_event};
use crate::product::create_start_view_product_event;
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
            .service(create_start_view_product_event)
            // .service(create_pre_login_event)
            // .service(create_login_event)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", config_for_server.clone().http.port))?
    .run()
    .await
}

async fn not_found() -> SimpleResponse {
    response::NOT_FOUND_RESPONSE
}