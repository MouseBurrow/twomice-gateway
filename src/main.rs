use actix_web::{get, App, HttpResponse, HttpServer};
use config::config::Config;

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // If it is dev mode, load env variables from .env
    #[cfg(debug_assertions)]
    Config::load_local_env();

    // Start listening endpoint
    let config = Config::load("GATEWAY");
    let addr = format!("0.0.0.0:{}", config.port);
    HttpServer::new(|| App::new().service(ping))
        .bind(addr)?
        .run()
        .await?;

    Ok(())
}
