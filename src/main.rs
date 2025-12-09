mod gateway_app;
mod request_handler;

use crate::gateway_app::GatewayApp;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};

use crate::request_handler::request_handler;
use env_logger::Env;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up the logger
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filter = match app_env.as_str() {
        "prod" => "warn,axum=info,tower_http=info",
        // "staging" => "info,axum=info,tower_http=debug", << We might need later on
        _ => "debug", // dev
    };
    env_logger::init_from_env(Env::default().default_filter_or(filter));

    let shared_app = web::Data::new(GatewayApp::new());
    HttpServer::new(move || {
        App::new()
            .app_data(shared_app.clone())
            .wrap(Logger::default())
            .default_service(web::to(request_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
