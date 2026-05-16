mod gateway_app;
mod request_handler;

use axum::Router;
use env_logger::Env;
use gateway_app::GatewayApp;
use request_handler::request_handler;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filter = match app_env.as_str() {
        "prod" => "warn,gateway=info",
        _ => "debug",
    };
    env_logger::init_from_env(Env::default().default_filter_or(filter));

    let auth_service_url = env::var("AUTH_SERVICE_URL")
        .unwrap_or_else(|_| "http://auth:8080".into());
    let post_service_url = env::var("POST_SERVICE_URL")
        .unwrap_or_else(|_| "http://post:8080".into());
    let moderation_service_url = env::var("MODERATION_SERVICE_URL")
        .unwrap_or_else(|_| "http://moderation:8080".into());
    let social_service_url = env::var("SOCIAL_SERVICE_URL")
        .unwrap_or_else(|_| "http://social:8080".into());
    let feed_service_url = env::var("FEED_SERVICE_URL")
        .unwrap_or_else(|_| "http://social-feed:8080".into());

    let app = GatewayApp::new(
        auth_service_url,
        post_service_url,
        moderation_service_url,
        social_service_url,
        feed_service_url,
    );

    let router = Router::new()
        .fallback(request_handler)
        .layer(axum::Extension(app));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
