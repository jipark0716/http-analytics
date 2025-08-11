mod status;
mod api;

use std::sync::OnceLock;
use crate::status::AppStatus;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::analyze::HttpAnalyzeConfig;
use config::import;
use utoipa_swagger_ui::SwaggerUi;
use http::not_found;

#[cfg(feature = "development")]
static CONFIG_BIN: &[u8] = include_bytes!("../config/development.bin");

static CONFIG: OnceLock<HttpAnalyzeConfig<'static>> = OnceLock::new();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    CONFIG.set(import::<HttpAnalyzeConfig>(CONFIG_BIN)).unwrap();

    println!("{:?}", CONFIG.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppStatus::new(CONFIG.get().unwrap())))
            .wrap(Cors::permissive())
            .configure(api::routes)
            .default_service(web::route().to(not_found))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/openapi.json", api::openapi()),
            )
    })
        .bind(("127.0.0.1", CONFIG.get().unwrap().http.port))?
        .run()
        .await
}