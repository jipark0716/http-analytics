extern crate core;

mod status;
mod api;

use std::sync::OnceLock;
use actix_cors::Cors;
use crate::status::AppStatus;
use actix_web::{web, App, HttpServer};
use config::collect::HttpCollectConfig;
use config::import;
use utoipa_swagger_ui::SwaggerUi;
use actix_files::Files;

#[cfg(feature = "development")]
static CONFIG_BIN: &[u8] = include_bytes!("../config/development.bin");

static CONFIG: OnceLock<HttpCollectConfig<'static>> = OnceLock::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    CONFIG.set(import::<HttpCollectConfig>(CONFIG_BIN)).unwrap();

    println!("{:?}", CONFIG.get().unwrap());

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(AppStatus::new(CONFIG.get().unwrap())))
        .wrap(Cors::permissive())
        .service(Files::new("/public", "./public").show_files_listing())
        .configure(api::routes)
        .default_service(web::route().to(http::not_found))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/openapi.json", api::openapi()),
        ))
    .bind(("127.0.0.1", CONFIG.get().unwrap().http.port))?
    .run()
    .await
}